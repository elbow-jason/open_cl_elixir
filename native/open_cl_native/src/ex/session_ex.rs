use super::WrapperEx;
use crate::nif;
use crate::nif::ErrorT;
use crate::traits::NativeWrapper;
use crate::{
    ArrayEx, BufferEx, CommandQueueOptionsEx, CommandQueuePropEx, DeviceEx, KernelOpEx,
    MemConfigEx, NumExT, NumList, NumTypeEx,
};
// use open_cl_core::ll::cl::cl_mem;
use open_cl_core::{
    Buffer, BufferBuilder, CommandQueueOptions, CommandQueueProperties, Device, MemConfig,
    NumberTypeError, NumberTyped, Session,
};
use std::fmt;

#[derive(nif::NifStruct)]
#[must_use]
#[module = "OpenCL.Session"]
pub struct SessionEx {
    __native__: nif::ResourceArc<WrapperEx<Session>>,
    _unconstructable: (),
}

impl fmt::Debug for SessionEx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SessionEx {{ native: {:?} }}", self.native())
    }
}

impl NativeWrapper<Session> for SessionEx {
    fn native(&self) -> &Session {
        &self.__native__.item
    }
}

fn parse_command_queue_props(props: Vec<CommandQueuePropEx>) -> Option<CommandQueueProperties> {
    if props.is_empty() {
        None
    } else {
        Some(CommandQueuePropEx::vec_into_cl_type(props))
    }
}

fn wrap_sessions(sessions: Vec<Session>) -> Vec<SessionEx> {
    sessions
        .into_iter()
        .map(|sess| SessionEx::new(sess))
        .collect()
}

impl SessionEx {
    pub fn new(session: Session) -> SessionEx {
        SessionEx {
            __native__: nif::ResourceArc::new(WrapperEx::new(session)),
            _unconstructable: (),
        }
    }

    pub fn create(src: &str, props: Vec<CommandQueuePropEx>) -> nif::Result<Vec<SessionEx>> {
        let cq_props = parse_command_queue_props(props);
        let hl_sessions: Vec<Session> = Session::create(src, cq_props).map_err(|e| e.error())?;
        Ok(wrap_sessions(hl_sessions))
    }

    pub fn create_with_devices(
        src: &str,
        devices: &[DeviceEx],
        props: Vec<CommandQueuePropEx>,
    ) -> nif::Result<Vec<SessionEx>> {
        let cq_props = parse_command_queue_props(props);
        let hl_devices: Vec<Device> = devices.iter().map(|d| d.native().clone()).collect();
        let hl_sessions: Vec<Session> =
            Session::create_with_devices(hl_devices, src, cq_props).map_err(|e| e.error())?;
        Ok(wrap_sessions(hl_sessions))
    }

    pub fn device(&self) -> DeviceEx {
        DeviceEx::new(self.native().device().clone())
    }

    pub fn clone_native(&self) -> Session {
        self.native().clone()
    }

    pub fn native(&self) -> &Session {
        &self.__native__.item
    }
}

impl From<SessionEx> for Session {
    fn from(sess: SessionEx) -> Session {
        sess.clone_native()
    }
}

#[rustler::nif]
fn session_create(src: String, props: Vec<CommandQueuePropEx>) -> nif::Result<Vec<SessionEx>> {
    SessionEx::create(&src[..], props)
}

#[rustler::nif]
fn session_create_with_devices(
    src: String,
    devices: Vec<DeviceEx>,
    props: Vec<CommandQueuePropEx>,
) -> nif::Result<Vec<SessionEx>> {
    SessionEx::create_with_devices(&src[..], &devices[..], props)
}

#[rustler::nif]
fn session_self_device(session: SessionEx) -> DeviceEx {
    session.device()
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn session_self_create_buffer_with_length(
    session: SessionEx,
    num_type_ex: NumTypeEx,
    length: usize,
    config: MemConfigEx,
) -> nif::Result<BufferEx> {
    let mem_config = build_mem_config(config, &length);
    let tid = num_type_ex.number_type().number_type_id();
    apply_type_id! {
        type_id: tid,
        func: _create_buffer_from_len,
        args: [&session, length, mem_config],
        default: Err(nif::error_string("Unmatched type_id during create_buffer from length"))
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn session_self_create_buffer_from_list<'a>(
    session: SessionEx,
    num_type_ex: NumTypeEx,
    iter: nif::ListIterator<'a>,
    config: MemConfigEx,
) -> nif::Result<BufferEx> {
    let list = NumList::from_num_typed_iter(num_type_ex, iter)?;
    let mem_config = build_mem_config(config, &list);
    apply_type_id! {
        type_id: list.tid(),
        func: _create_buffer_from_list,
        args: [&session, list, mem_config],
        default: Err(nif::error_string("Unmatched type_id during create_buffer_from_list"))
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn session_self_create_buffer_from_array<'a>(
    session: SessionEx,
    array: ArrayEx,
    config: MemConfigEx,
) -> nif::Result<BufferEx> {
    let list = array.read_lock();
    let tid = list.tid();
    let mem_config = build_mem_config(config, &(*list));
    std::mem::drop(list);
    apply_type_id! {
        type_id: tid,
        func: _create_buffer_from_array,
        args: [&session, array, mem_config],
        default: Err(nif::error_string("Unmatched type_id during create_buffer from array"))
    }
}

fn build_mem_config<T: BufferBuilder>(config: MemConfigEx, buffer_builder: &T) -> MemConfig {
    config
        .into_builder()
        .with_mem_allocation(buffer_builder)
        .build()
}

#[inline]
fn _create_buffer_from_len<T: NumExT>(
    sess: &SessionEx,
    len: usize,
    mem_config: MemConfig,
) -> nif::Result<BufferEx> {
    let core_buffer = sess
        .native()
        .create_buffer_with_config::<T, usize>(len, mem_config)
        .map_err(|e| e.error())?;
    Ok(BufferEx::from_core_buffer(core_buffer))
}

fn _buffer_from_slice<T: NumExT>(
    sess: &SessionEx,
    data: &[T],
    mem_config: MemConfig,
) -> nif::Result<BufferEx> {
    sess.native()
        .create_buffer_with_config::<T, &[T]>(data, mem_config)
        .map(|b| BufferEx::from_core_buffer(b))
        .map_err(|e| e.error())
}

fn _create_buffer_from_list<T: NumExT>(
    sess: &SessionEx,
    list: NumList,
    mem_config: MemConfig,
) -> nif::Result<BufferEx> {
    _buffer_from_slice::<T>(sess, list.as_slice()?, mem_config)
}

fn _create_buffer_from_array<T: NumExT>(
    sess: &SessionEx,
    array: ArrayEx,
    mem_config: MemConfig,
) -> nif::Result<BufferEx> {
    let list = array.read_lock();
    _buffer_from_slice(sess, list.as_slice::<T>()?, mem_config)
}

fn _sync_write_buffer<T: NumExT>(
    sess: &SessionEx,
    buffer: BufferEx,
    array: ArrayEx,
    cq_options: Option<CommandQueueOptionsEx>,
) -> nif::Result<()> {
    let list = array.read_lock();
    let data: &[T] = list.as_slice().unwrap();
    let buffer_t: &Buffer = buffer.wrapper().buffer();
    let cl_cq_opts: Option<CommandQueueOptions> = cq_options.map(|o| o.into());
    sess.native()
        .sync_write_buffer::<T, &[T]>(buffer_t, data, cl_cq_opts)
        .map_err(|e| e.error())
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn session_self_write_array_to_buffer(
    session: SessionEx,
    buffer: BufferEx,
    array: ArrayEx,
    cq_options: Option<CommandQueueOptionsEx>,
) -> nif::Result<()> {
    let num_type = buffer.number_type();
    if num_type != array.number_type() {
        let e = NumberTypeError::Mismatch(num_type, array.number_type());
        return Err(e.error());
    }
    apply_type_id!(
        type_id: num_type.number_type_id(),
        func: _sync_write_buffer,
        args: [&session, buffer, array, cq_options],
        default: Err(nif::error_string("Unmatched type_id during write_array_to_buffer"))
    )
}

pub fn _sync_read_buffer<T: NumExT>(
    sess: &SessionEx,
    buf_ex: BufferEx,
    cq_opts_ex: Option<CommandQueueOptionsEx>,
) -> nif::Result<ArrayEx> {
    let buffer: &Buffer = buf_ex.wrapper().buffer();
    let data = vec![T::zero(); buffer.len()];
    let cq_opts_cl: Option<CommandQueueOptions> = cq_opts_ex.map(|o| o.into());

    sess.native()
        .sync_read_buffer(buffer, data, cq_opts_cl)
        .map_err(|e| e.error())
        .map(|num_vec| ArrayEx::from(NumList::from_vec(num_vec.unwrap())))
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn session_self_read_buffer(
    session: SessionEx,
    buffer: BufferEx,
    cq_opts_ex: Option<CommandQueueOptionsEx>,
) -> nif::Result<ArrayEx> {
    let num_type = buffer.number_type();
    apply_type_id! {
        type_id: num_type.number_type_id(),
        func: _sync_read_buffer,
        args: [&session, buffer, cq_opts_ex],
        default: Err(nif::error_string("Unmatched type_id during read_buffer"))
    }
}

// pub struct LockedMemArg<'a> {
//     _write_lock: RwLockWriteGuard<'a, Mem>,
//     _arg_ptr: ArgPtr<'a>,
// }

// impl<'a> LockedMemArg<'a> {
//     pub fn new(write_lock: RwLockWriteGuard<'a, Mem>) -> LockedMemArg<'a> {
//         let arg_ptr = unsafe {
//             ArgPtr::from_raw_parts(
//                 write_lock.mem_ptr().as_mut_ptr(),
//                 std::mem::size_of::<cl_mem>(),
//             )
//         };
//         LockedMemArg {
//             _write_lock: write_lock,
//             _arg_ptr: arg_ptr,
//         }
//     }
//     pub fn arg_ptr(&self) -> &ArgPtr<'a> {
//         &self._arg_ptr
//     }
// }

#[rustler::nif(schedule = "DirtyCpu")]
pub fn session_self_execute_kernel_operation(
    session: SessionEx,
    op_ex: KernelOpEx,
) -> nif::Result<()> {
    session
        .native()
        .execute_sync_kernel_operation(op_ex.into_kernel_operation())
        .map_err(|e| e.error())
}

#[rustler::nif]
pub fn session_self_create_copy(session: SessionEx) -> nif::Result<SessionEx> {
    session
        .native()
        .create_copy()
        .map(|s| SessionEx::new(s))
        .map_err(|e| e.error())
}
