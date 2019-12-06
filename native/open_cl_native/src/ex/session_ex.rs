use std::fmt;
use std::ops::Deref;

use opencl_core::Session;
use rustler::resource::ResourceArc;
use rustler::{Encoder, NifStruct};

use super::{OutputEx, WrapperEx, WrapperExResource};

use crate::traits::NativeWrapper;

use crate::device_ex::DeviceEx;

impl WrapperExResource for Session {}

#[derive(NifStruct)]
#[must_use]
#[module = "OpenCL.Session"]
pub struct SessionEx {
    __native__: ResourceArc<WrapperEx<Session>>,
    src: String,
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
    pub fn new(session: Session, src: String) -> SessionEx {
        SessionEx {
            __native__: session.into_resource_arc(),
            src,
            _unconstructable: (),
        }
    }

    pub fn create(device: &DeviceEx, src: String) -> OutputEx<SessionEx> {
        let native_session: Session = Session::create(device.native().clone(), &src[..])?;
        Ok(SessionEx::new(native_session, src))
    }

    pub fn clone_device_ex(&self) -> DeviceEx {
        DeviceEx::new(self.native().device().clone())
    }

    pub fn clone_native(&self) -> Session {
        self.native().clone()
    }
}

#[rustler::nif]
fn session_create_with_src(device: DeviceEx, src: String) -> OutputEx<SessionEx> {
    SessionEx::create(&device, src)
}

#[rustler::nif]
fn session_self_device(session: SessionEx) -> DeviceEx {
    session.clone_device_ex()
}

// #[rustler::nif]
// fn session_self_kernel_sync_execute(session: SessionEx, kernel: KernelEx) -> OutputEx {}

#[macro_export]
macro_rules! impl_session_method_and_nif {
    ($field:ident, $func_name:ident, $ret:ty) => {
        paste::item! {
            impl SessionEx {
                pub fn [<$field _ $func_name>](&self) -> OutputEx<$ret> {
                    self.native()
                    .$field()
                    .$func_name()
                    .map_err(|e| e.into())
                }
            }

            #[rustler::nif]
            pub fn [<session_self_ $field _ $func_name>](item: SessionEx) -> OutputEx<$ret> {
                item.[<$field _ $func_name>]()
            }
        }
    };
}

impl_session_method_and_nif!(device, name, String);
impl_session_method_and_nif!(device, opencl_c_version, String);
impl_session_method_and_nif!(device, profile, String);
impl_session_method_and_nif!(device, vendor, String);
impl_session_method_and_nif!(device, version, String);
impl_session_method_and_nif!(device, driver_version, String);
impl_session_method_and_nif!(device, address_bits, u32);
impl_session_method_and_nif!(device, global_mem_cacheline_size, u32);
impl_session_method_and_nif!(device, max_clock_frequency, u32);
impl_session_method_and_nif!(device, max_compute_units, u32);
impl_session_method_and_nif!(device, max_constant_args, u32);
impl_session_method_and_nif!(device, max_read_image_args, u32);
impl_session_method_and_nif!(device, max_samplers, u32);
impl_session_method_and_nif!(device, max_work_item_dimensions, u32);
impl_session_method_and_nif!(device, max_write_image_args, u32);
impl_session_method_and_nif!(device, mem_base_addr_align, u32);
impl_session_method_and_nif!(device, min_data_type_align_size, u32);
impl_session_method_and_nif!(device, native_vector_width_char, u32);
impl_session_method_and_nif!(device, native_vector_width_short, u32);
impl_session_method_and_nif!(device, native_vector_width_int, u32);
impl_session_method_and_nif!(device, native_vector_width_long, u32);
impl_session_method_and_nif!(device, native_vector_width_float, u32);
impl_session_method_and_nif!(device, native_vector_width_double, u32);
impl_session_method_and_nif!(device, native_vector_width_half, u32);
impl_session_method_and_nif!(device, partition_max_sub_devices, u32);
impl_session_method_and_nif!(device, preferred_vector_width_char, u32);
impl_session_method_and_nif!(device, preferred_vector_width_short, u32);
impl_session_method_and_nif!(device, preferred_vector_width_int, u32);
impl_session_method_and_nif!(device, preferred_vector_width_long, u32);
impl_session_method_and_nif!(device, preferred_vector_width_float, u32);
impl_session_method_and_nif!(device, preferred_vector_width_double, u32);
impl_session_method_and_nif!(device, preferred_vector_width_half, u32);
impl_session_method_and_nif!(device, vendor_id, u32);
impl_session_method_and_nif!(device, available, bool);
impl_session_method_and_nif!(device, compiler_available, bool);
impl_session_method_and_nif!(device, endian_little, bool);
impl_session_method_and_nif!(device, error_correction_support, bool);
impl_session_method_and_nif!(device, host_unified_memory, bool);
impl_session_method_and_nif!(device, image_support, bool);
impl_session_method_and_nif!(device, linker_available, bool);
impl_session_method_and_nif!(device, preferred_interop_user_sync, bool);
impl_session_method_and_nif!(device, image2d_max_width, usize);
impl_session_method_and_nif!(device, image2d_max_height, usize);
impl_session_method_and_nif!(device, image3d_max_width, usize);
impl_session_method_and_nif!(device, image3d_max_height, usize);
impl_session_method_and_nif!(device, image3d_max_depth, usize);
impl_session_method_and_nif!(device, image_max_buffer_size, usize);
impl_session_method_and_nif!(device, image_max_array_size, usize);
impl_session_method_and_nif!(device, max_parameter_size, usize);
impl_session_method_and_nif!(device, max_work_group_size, usize);
impl_session_method_and_nif!(device, printf_buffer_size, usize);
impl_session_method_and_nif!(device, profiling_timer_resolution, usize);
impl_session_method_and_nif!(device, max_work_item_sizes, Vec<usize>);
