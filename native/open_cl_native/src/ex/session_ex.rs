use std::fmt;

use opencl_core::{ClNumber, Device, MemConfig, Session, Buffer, CommandQueueOptions};
use opencl_core::ll::{utils};
// use opencl_core::ll::{DevicePtr};
use rustler::resource::ResourceArc;
use rustler::{Encoder, NifStruct};

use super::{OutputEx, WrapperEx, WrapperExResource};

use crate::traits::NativeWrapper;

use crate::{
    BufferCreatorEx, BufferEx, DeviceEx, MemConfigEx, NumberType, NumberTyped, NumberTypedT,
    NumberListEx, NumberEx, ArrayEx, CommandQueueOptionsEx, RuntimeNumberList, KernelOpEx,
    ArgEx,
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

impl SessionEx {
    pub fn new(session: Session) -> SessionEx {
        SessionEx {
            __native__: session.into_resource_arc(),
            _unconstructable: (),
        }
    }

    pub fn create(src: &str) -> OutputEx<SessionEx> {
        let hl_session: Session = Session::create(src)?;
        Ok(SessionEx::new(hl_session))
    }

    pub fn create_with_devices(src: &str, devices: &[DeviceEx]) -> OutputEx<SessionEx> {
        let hl_devices: Vec<Device> = devices.iter().map(|d| d.native().clone()).collect();
        let hl_session: Session = Session::create_with_devices(hl_devices, src)?;
        Ok(SessionEx::new(hl_session))
    }

    pub fn devices(&self) -> Vec<DeviceEx> {
        self.native()
            .devices()
            .iter()
            .map(|d| DeviceEx::new(d.clone()))
            .collect()
    }

    pub fn clone_native(&self) -> Session {
        self.native().clone()
    }

    pub fn native(&self) -> &Session {
        &self.__native__.item
    }
    // pub fn execute_kernel(&self, name: &str, dims: DimsEx, ) -> OutputEx<()> {

    // }
}

impl From<SessionEx> for Session {
    fn from(sess: SessionEx) -> Session {
        sess.clone_native()
    }
}

#[rustler::nif]
fn session_create(src: String) -> OutputEx<SessionEx> {
    SessionEx::create(&src[..])
}

#[rustler::nif]
fn session_create_with_devices(src: String, devices: Vec<DeviceEx>) -> OutputEx<SessionEx> {
    SessionEx::create_with_devices(&src[..], &devices[..])
}

#[rustler::nif]
fn session_self_devices(session: SessionEx) -> Vec<DeviceEx> {
    session.devices()
}

// #[rustler::nif]
// fn session_self_read_buffer(session: SessionEx, buffer: BufferEx, config: MemConfigEx) -> ArrayEx {
//     // let native_session = session.native().sync_read_buffer()
//     //  pub fn sync_read_buffer<'a, T: ClNumber, H: Into<MutVecOrSlice<'a, T>>>(
//     //     &mut self,
//     //     queue_index: usize,
//     //     buffer: &Buffer<T>,
//     //     host_buffer: H,
//     //     opts: Option<CommandQueueOptions>,
//     // ) -> Output<Option<Vec<T>>> {
// }

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
        BufferCreatorEx::List(list) => {
            create_buffer_from_list(&session, list, mem_config)
        }
        BufferCreatorEx::Array(arr) => {
            create_buffer_from_array(&session, arr, mem_config)
        }
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
    let core_buffer = sess.native().create_buffer_with_config::<T, usize>(len, mem_config)?;
    Ok(BufferEx::from_core_buffer(core_buffer))
}

fn _buffer_from_slice<T: NumberEx>(sess: &SessionEx, data: &[T], mem_config: MemConfig) -> OutputEx<BufferEx> {    
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
    queue_index: usize,
    buffer: BufferEx,
    array: ArrayEx,
    cq_options: Option<CommandQueueOptionsEx>
) -> OutputEx<()> {
    let rt_list = array.read_lock();
    let data: &[T] = rt_list.force_as_slice();
    let buffer_t: &Buffer<T> = buffer.wrapper().buffer().unwrap();
    let cl_cq_opts: Option<CommandQueueOptions> = cq_options.map(|o| o.into());
    sess.native()
        .sync_write_buffer::<T, &[T]>(queue_index, buffer_t, data, cl_cq_opts)
        .map_err(From::from)
}



#[rustler::nif]
pub fn session_self_write_array_to_buffer(
    session: SessionEx,
    queue_index: usize,
    buffer: BufferEx,
    array: ArrayEx,
    cq_options: Option<CommandQueueOptionsEx>
) -> OutputEx<()> {
    let num_type = buffer.number_type();
    if num_type != array.number_type() {
        num_type.mismatch_error(array.number_type());
    }
    apply_number_type!(
        num_type,
        _sync_write_buffer,
        [&session, queue_index, buffer, array, cq_options]
    )
}

pub fn _sync_read_buffer<T: NumberEx>(
    sess: &SessionEx,
    queue_index: usize,
    buffer: BufferEx,
    cq_opts_ex: Option<CommandQueueOptionsEx>,
) -> OutputEx<ArrayEx> {
    let buffer_t: &Buffer<T> = buffer.wrapper().buffer().unwrap();
    let data = utils::vec_filled_with::<T>(T::zero(), buffer_t.len());
    let cq_opts_cl: Option<CommandQueueOptions> = cq_opts_ex.map(|o| o.into());
    
    sess.native()
        .sync_read_buffer(queue_index, buffer_t, data, cq_opts_cl)
        .map_err(From::from)
        .map(|num_vec| ArrayEx::from(RuntimeNumberList::from_vec(num_vec.unwrap())))
}

#[rustler::nif]
pub fn session_self_read_buffer(
    session: SessionEx,
    queue_index: usize,
    buffer: BufferEx,
    cq_opts_ex: Option<CommandQueueOptionsEx>
) -> OutputEx<ArrayEx> {
    let num_type = buffer.number_type();
    apply_number_type!(
        num_type,
        _sync_read_buffer,
        [&session, queue_index, buffer, cq_opts_ex]
    )
}


pub fn _execute_sync_kernel_operation<T: NumberEx>(
    session: &SessionEx,
    queue_index: usize,
    kernel_op_ex: KernelOpEx,
) -> OutputEx<()> {
    let kernel_op_cl = kernel_op_ex.into_kernel_operation::<T>()?;
    let _ = session
        .native()
        .execute_sync_kernel_operation::<T>(queue_index, kernel_op_cl)?;
    Ok(())
}

#[rustler::nif]
pub fn session_self_execute_kernel_operation(
    session: SessionEx,
    queue_index: usize,
    kernel_op_ex: KernelOpEx,
) -> OutputEx<()> {
    let num_type = kernel_op_ex.number_type();
    apply_number_type!(
        num_type,
        _execute_sync_kernel_operation,
        [&session, queue_index, kernel_op_ex]
    )
}

//     match () {
//         (U8, U8) => ,
//         (I8, I8) => _sync_write_buffer::<i8>::(&session, queue_index, buffer, opts, cq_options),
//         (U16, U16) => _sync_write_buffer::<u16>::(&session, queue_index, buffer, opts, cq_options),
//         (I16, I16) => _sync_write_buffer::<i16>::(&session, queue_index, buffer, opts, cq_options),
//         (U32, U32) => _sync_write_buffer::<u32>::(&session, queue_index, buffer, opts, cq_options),
//         (I32, I32) => _sync_write_buffer::<i32>::(&session, queue_index, buffer, opts, cq_options),
//         (F32, F32) => _sync_write_buffer::<f32>::(&session, queue_index, buffer, opts, cq_options),
//         (U64, U64) => _sync_write_buffer::<u64>::(&session, queue_index, buffer, opts, cq_options),
//         (I64, I64) => _sync_write_buffer::<i64>::(&session, queue_index, buffer, opts, cq_options),
//         (F64, F64) => _sync_write_buffer::<f64>::(&session, queue_index, buffer, opts, cq_options),
//         (Usize, Usize) => _sync_write_buffer::<usize>::(&session, queue_index, buffer, opts, cq_options),
//         (Isize, Isize) => _sync_write_buffer::<isize>::(&session, queue_index, buffer, opts, cq_options),
//         (buffer_t, array_t) => Err(buffer_t.mismatch_error(array_t))
//     }

//     pub fn sync_write_buffer<'a, T: ClNumber, H: Into<VecOrSlice<'a, T>>>(
//         &mut self,
//         queue_index: usize,
//         buffer: &Buffer<T>,
//         host_buffer: H,
//         opts: Option<CommandQueueOptions>,
//     ) -> Output<()> {
//         let queue_locker: &RwLock<ClCommandQueue> = self.get_queue_by_index(queue_index)?;
//         let mut queue_lock = queue_locker.write().unwrap();
//         let mut buffer_lock = buffer.write_lock();
//         unsafe {
//             let event: ClEvent = queue_lock.write_buffer(&mut (*buffer_lock), host_buffer, opts)?;
//             event.wait()
//         }
//     }

//     creator_ex.check_matches_type(number_type)?;

//     let mem_config = build_mem_config(config, &creator_ex);
//     match creator_ex {
//         BufferCreatorEx::List(list) => {
//             create_buffer_from_list(&session, list, mem_config)
//         }
//         BufferCreatorEx::Array(arr) => {
//             create_buffer_from_array(&session, arr, mem_config)
//         }
//         BufferCreatorEx::Length(len) => {
//             create_buffer_from_len(&session, number_type, len, mem_config)
//         }
//     }
// }
// session_self_write_array_to_buffer

// #[rustler::nif]
// fn session_self_execute(session: SessionEx, kernel_name: String) -> OutputEx<KernelEx> {

// }

// #[macro_export]
// macro_rules! impl_session_low_level_method_and_nif {
//     ($field:ident, $func_name:ident, $ret:ty) => {
//         paste::item! {
//             impl SessionEx {
//                 pub fn [<$field _ $func_name>](&self) -> OutputEx<$ret> {
//                     self.native()
//                     .$field()
//                     .$func_name()
//                     .map_err(|e| e.into())
//                 }
//             }

//             #[rustler::nif]
//             pub fn [<session_self_ $field _ $func_name>](item: SessionEx) -> OutputEx<$ret> {
//                 item.[<$field _ $func_name>]()
//             }
//         }
//     };
// }

// // impl_session_low_level_method_and_nif!(device, name, String);
// // impl_session_low_level_method_and_nif!(device, opencl_c_version, String);
// // impl_session_low_level_method_and_nif!(device, profile, String);
// // impl_session_low_level_method_and_nif!(device, vendor, String);
// // impl_session_low_level_method_and_nif!(device, version, String);
// // impl_session_low_level_method_and_nif!(device, driver_version, String);
// // impl_session_low_level_method_and_nif!(device, address_bits, u32);
// // impl_session_low_level_method_and_nif!(device, global_mem_cacheline_size, u32);
// // impl_session_low_level_method_and_nif!(device, max_clock_frequency, u32);
// // impl_session_low_level_method_and_nif!(device, max_compute_units, u32);
// // impl_session_low_level_method_and_nif!(device, max_constant_args, u32);
// // impl_session_low_level_method_and_nif!(device, max_read_image_args, u32);
// // impl_session_low_level_method_and_nif!(device, max_samplers, u32);
// // impl_session_low_level_method_and_nif!(device, max_work_item_dimensions, u32);
// // impl_session_low_level_method_and_nif!(device, max_write_image_args, u32);
// // impl_session_low_level_method_and_nif!(device, mem_base_addr_align, u32);
// // impl_session_low_level_method_and_nif!(device, min_data_type_align_size, u32);
// // impl_session_low_level_method_and_nif!(device, native_vector_width_char, u32);
// // impl_session_low_level_method_and_nif!(device, native_vector_width_short, u32);
// // impl_session_low_level_method_and_nif!(device, native_vector_width_int, u32);
// // impl_session_low_level_method_and_nif!(device, native_vector_width_long, u32);
// // impl_session_low_level_method_and_nif!(device, native_vector_width_float, u32);
// // impl_session_low_level_method_and_nif!(device, native_vector_width_double, u32);
// // impl_session_low_level_method_and_nif!(device, native_vector_width_half, u32);
// // impl_session_low_level_method_and_nif!(device, partition_max_sub_devices, u32);
// // impl_session_low_level_method_and_nif!(device, preferred_vector_width_char, u32);
// // impl_session_low_level_method_and_nif!(device, preferred_vector_width_short, u32);
// // impl_session_low_level_method_and_nif!(device, preferred_vector_width_int, u32);
// // impl_session_low_level_method_and_nif!(device, preferred_vector_width_long, u32);
// // impl_session_low_level_method_and_nif!(device, preferred_vector_width_float, u32);
// // impl_session_low_level_method_and_nif!(device, preferred_vector_width_double, u32);
// // impl_session_low_level_method_and_nif!(device, preferred_vector_width_half, u32);
// // impl_session_low_level_method_and_nif!(device, vendor_id, u32);
// // impl_session_low_level_method_and_nif!(device, available, bool);
// // impl_session_low_level_method_and_nif!(device, compiler_available, bool);
// // impl_session_low_level_method_and_nif!(device, endian_little, bool);
// // impl_session_low_level_method_and_nif!(device, error_correction_support, bool);
// // impl_session_low_level_method_and_nif!(device, host_unified_memory, bool);
// // impl_session_low_level_method_and_nif!(device, image_support, bool);
// // impl_session_low_level_method_and_nif!(device, linker_available, bool);
// // impl_session_low_level_method_and_nif!(device, preferred_interop_user_sync, bool);
// // impl_session_low_level_method_and_nif!(device, image2d_max_width, usize);
// // impl_session_low_level_method_and_nif!(device, image2d_max_height, usize);
// // impl_session_low_level_method_and_nif!(device, image3d_max_width, usize);
// // impl_session_low_level_method_and_nif!(device, image3d_max_height, usize);
// // impl_session_low_level_method_and_nif!(device, image3d_max_depth, usize);
// // impl_session_low_level_method_and_nif!(device, image_max_buffer_size, usize);
// // impl_session_low_level_method_and_nif!(device, image_max_array_size, usize);
// // impl_session_low_level_method_and_nif!(device, max_parameter_size, usize);
// // impl_session_low_level_method_and_nif!(device, max_work_group_size, usize);
// // impl_session_low_level_method_and_nif!(device, printf_buffer_size, usize);
// // impl_session_low_level_method_and_nif!(device, profiling_timer_resolution, usize);
// // impl_session_low_level_method_and_nif!(device, max_work_item_sizes, Vec<usize>);
