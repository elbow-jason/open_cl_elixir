use std::fmt;

use opencl_core::ll::utils;
use opencl_core::{
    Buffer, ClNumber, CommandQueueOptions, CommandQueueProperties, Device, MemConfig, Session,
};
// use opencl_core::ll::{DevicePtr};
use rustler::resource::ResourceArc;
use rustler::{Encoder, NifStruct};

use super::{OutputEx, WrapperEx, WrapperExResource};

use crate::traits::NativeWrapper;

use crate::{
    ArrayEx, BufferCreatorEx, BufferEx, CommandQueueOptionsEx, CommandQueuePropEx, DeviceEx,
    KernelOpEx, MemConfigEx, NumberEx, NumberListEx, NumberType, NumberTyped, NumberTypedT,
    RuntimeNumberList,
};

impl WrapperExResource for Session {}

#[derive(NifStruct)]
#[must_use]
#[module = "OpenCL.Session"]
pub struct SessionEx {
    __native__: ResourceArc<WrapperEx<Session>>,
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
            __native__: session.into_resource_arc(),
            _unconstructable: (),
        }
    }

    pub fn create(src: &str, props: Vec<CommandQueuePropEx>) -> OutputEx<Vec<SessionEx>> {
        let cq_props = parse_command_queue_props(props);
        let hl_sessions: Vec<Session> = Session::create(src, cq_props)?;
        Ok(wrap_sessions(hl_sessions))
    }

    pub fn create_with_devices(
        src: &str,
        devices: &[DeviceEx],
        props: Vec<CommandQueuePropEx>,
    ) -> OutputEx<Vec<SessionEx>> {
        let cq_props = parse_command_queue_props(props);
        let hl_devices: Vec<Device> = devices.iter().map(|d| d.native().clone()).collect();
        let hl_sessions: Vec<Session> = Session::create_with_devices(hl_devices, src, cq_props)?;
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
fn session_create(src: String, props: Vec<CommandQueuePropEx>) -> OutputEx<Vec<SessionEx>> {
    SessionEx::create(&src[..], props)
}

#[rustler::nif]
fn session_create_with_devices(
    src: String,
    devices: Vec<DeviceEx>,
    props: Vec<CommandQueuePropEx>,
) -> OutputEx<Vec<SessionEx>> {
    SessionEx::create_with_devices(&src[..], &devices[..], props)
}

#[rustler::nif]
fn session_self_device(session: SessionEx) -> DeviceEx {
    session.device()
}

#[rustler::nif]
pub fn session_self_create_buffer(
    session: SessionEx,
    number_type: NumberType,
    creator_ex: BufferCreatorEx,
    config: MemConfigEx,
) -> OutputEx<BufferEx> {
    creator_ex.check_matches_type(number_type)?;

    let mem_config = build_mem_config(config, &creator_ex);
    match creator_ex {
        BufferCreatorEx::List(list) => create_buffer_from_list(&session, list, mem_config),
        BufferCreatorEx::Array(arr) => create_buffer_from_array(&session, arr, mem_config),
        BufferCreatorEx::Length(len) => {
            create_buffer_from_len(&session, number_type, len, mem_config)
        }
    }
}

fn build_mem_config(config: MemConfigEx, creator_ex: &BufferCreatorEx) -> MemConfig {
    config
        .into_builder()
        .with_mem_location_of_buffer_creator(&creator_ex)
        .build()
}

fn create_buffer_from_len(
    sess: &SessionEx,
    nt: NumberType,
    len: usize,
    mem_config: MemConfig,
) -> OutputEx<BufferEx> {
    use NumberType as NT;
    match nt.number_type() {
        NT::U8 => _create_buffer_from_len::<u8>(sess, len, mem_config),
        NT::I8 => _create_buffer_from_len::<i8>(sess, len, mem_config),
        NT::U16 => _create_buffer_from_len::<u16>(sess, len, mem_config),
        NT::I16 => _create_buffer_from_len::<i16>(sess, len, mem_config),
        NT::U32 => _create_buffer_from_len::<u32>(sess, len, mem_config),
        NT::I32 => _create_buffer_from_len::<i32>(sess, len, mem_config),
        NT::F32 => _create_buffer_from_len::<f32>(sess, len, mem_config),
        NT::U64 => _create_buffer_from_len::<u64>(sess, len, mem_config),
        NT::I64 => _create_buffer_from_len::<i64>(sess, len, mem_config),
        NT::F64 => _create_buffer_from_len::<f64>(sess, len, mem_config),
        NT::Usize => _create_buffer_from_len::<usize>(sess, len, mem_config),
        NT::Isize => _create_buffer_from_len::<isize>(sess, len, mem_config),
    }
}

#[inline]
fn _create_buffer_from_len<T: ClNumber + NumberTypedT>(
    sess: &SessionEx,
    len: usize,
    mem_config: MemConfig,
) -> OutputEx<BufferEx> {
    let core_buffer = sess
        .native()
        .create_buffer_with_config::<T, usize>(len, mem_config)?;
    Ok(BufferEx::from_core_buffer(core_buffer))
}

fn _buffer_from_slice<T: NumberEx>(
    sess: &SessionEx,
    data: &[T],
    mem_config: MemConfig,
) -> OutputEx<BufferEx> {
    sess.native()
        .create_buffer_with_config::<T, &[T]>(data, mem_config)
        .map(|b| BufferEx::from_core_buffer(b))
        .map_err(From::from)
}

fn create_buffer_from_list(
    sess: &SessionEx,
    list: NumberListEx,
    mem_config: MemConfig,
) -> OutputEx<BufferEx> {
    use NumberListEx as L;
    match list {
        L::U8(v) => _buffer_from_slice(sess, &v[..], mem_config),
        L::I8(v) => _buffer_from_slice(sess, &v[..], mem_config),
        L::U16(v) => _buffer_from_slice(sess, &v[..], mem_config),
        L::I16(v) => _buffer_from_slice(sess, &v[..], mem_config),
        L::U32(v) => _buffer_from_slice(sess, &v[..], mem_config),
        L::I32(v) => _buffer_from_slice(sess, &v[..], mem_config),
        L::F32(v) => _buffer_from_slice(sess, &v[..], mem_config),
        L::U64(v) => _buffer_from_slice(sess, &v[..], mem_config),
        L::I64(v) => _buffer_from_slice(sess, &v[..], mem_config),
        L::F64(v) => _buffer_from_slice(sess, &v[..], mem_config),
        L::Usize(v) => _buffer_from_slice(sess, &v[..], mem_config),
        L::Isize(v) => _buffer_from_slice(sess, &v[..], mem_config),
    }
}

fn create_buffer_from_array(
    sess: &SessionEx,
    array: ArrayEx,
    mem_config: MemConfig,
) -> OutputEx<BufferEx> {
    use NumberType as NT;
    let rt_list = array.read_lock();
    match rt_list.number_type() {
        NT::U8 => _buffer_from_slice(sess, rt_list.force_as_slice::<u8>(), mem_config),
        NT::I8 => _buffer_from_slice(sess, rt_list.force_as_slice::<i8>(), mem_config),
        NT::U16 => _buffer_from_slice(sess, rt_list.force_as_slice::<u16>(), mem_config),
        NT::I16 => _buffer_from_slice(sess, rt_list.force_as_slice::<i16>(), mem_config),
        NT::U32 => _buffer_from_slice(sess, rt_list.force_as_slice::<u32>(), mem_config),
        NT::I32 => _buffer_from_slice(sess, rt_list.force_as_slice::<i32>(), mem_config),
        NT::F32 => _buffer_from_slice(sess, rt_list.force_as_slice::<f32>(), mem_config),
        NT::U64 => _buffer_from_slice(sess, rt_list.force_as_slice::<u64>(), mem_config),
        NT::I64 => _buffer_from_slice(sess, rt_list.force_as_slice::<i64>(), mem_config),
        NT::F64 => _buffer_from_slice(sess, rt_list.force_as_slice::<f64>(), mem_config),
        NT::Usize => _buffer_from_slice(sess, rt_list.force_as_slice::<usize>(), mem_config),
        NT::Isize => _buffer_from_slice(sess, rt_list.force_as_slice::<isize>(), mem_config),
    }
}

fn _sync_write_buffer<T: NumberEx>(
    sess: &SessionEx,
    buffer: BufferEx,
    array: ArrayEx,
    cq_options: Option<CommandQueueOptionsEx>,
) -> OutputEx<()> {
    let rt_list = array.read_lock();
    let data: &[T] = rt_list.force_as_slice();
    let buffer_t: &Buffer<T> = buffer.wrapper().buffer().unwrap();
    let cl_cq_opts: Option<CommandQueueOptions> = cq_options.map(|o| o.into());
    sess.native()
        .sync_write_buffer::<T, &[T]>(buffer_t, data, cl_cq_opts)
        .map_err(From::from)
}

#[rustler::nif]
pub fn session_self_write_array_to_buffer(
    session: SessionEx,
    buffer: BufferEx,
    array: ArrayEx,
    cq_options: Option<CommandQueueOptionsEx>,
) -> OutputEx<()> {
    let num_type = buffer.number_type();
    if num_type != array.number_type() {
        num_type.mismatch_error(array.number_type());
    }
    apply_number_type!(
        num_type,
        _sync_write_buffer,
        [&session, buffer, array, cq_options]
    )
}

pub fn _sync_read_buffer<T: NumberEx>(
    sess: &SessionEx,
    buffer: BufferEx,
    cq_opts_ex: Option<CommandQueueOptionsEx>,
) -> OutputEx<ArrayEx> {
    let buffer_t: &Buffer<T> = buffer.wrapper().buffer().unwrap();
    let data = utils::vec_filled_with::<T>(T::zero(), buffer_t.len());
    let cq_opts_cl: Option<CommandQueueOptions> = cq_opts_ex.map(|o| o.into());

    sess.native()
        .sync_read_buffer(buffer_t, data, cq_opts_cl)
        .map_err(From::from)
        .map(|num_vec| ArrayEx::from(RuntimeNumberList::from_vec(num_vec.unwrap())))
}

#[rustler::nif]
pub fn session_self_read_buffer(
    session: SessionEx,
    buffer: BufferEx,
    cq_opts_ex: Option<CommandQueueOptionsEx>,
) -> OutputEx<ArrayEx> {
    let num_type = buffer.number_type();
    apply_number_type!(num_type, _sync_read_buffer, [&session, buffer, cq_opts_ex])
}

pub fn _execute_sync_kernel_operation<T: NumberEx>(
    session: &SessionEx,
    kernel_op_ex: KernelOpEx,
) -> OutputEx<()> {
    let kernel_op_cl = kernel_op_ex.into_kernel_operation::<T>()?;
    let _ = session
        .native()
        .execute_sync_kernel_operation::<T>(kernel_op_cl)?;
    Ok(())
}

#[rustler::nif]
pub fn session_self_execute_kernel_operation(
    session: SessionEx,
    kernel_op_ex: KernelOpEx,
) -> OutputEx<()> {
    let num_type = kernel_op_ex.number_type();
    apply_number_type!(
        num_type,
        _execute_sync_kernel_operation,
        [&session, kernel_op_ex]
    )
}

#[rustler::nif]
pub fn session_self_create_copy(session: SessionEx) -> OutputEx<SessionEx> {
    session
        .native()
        .create_copy()
        .map(|s| SessionEx::new(s))
        .map_err(From::from)
}
