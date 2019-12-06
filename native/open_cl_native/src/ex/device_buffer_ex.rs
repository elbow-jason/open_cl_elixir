use std::default::Default;
use std::fmt;

use opencl_core::DeviceMem;
use rustler::resource::ResourceArc;

use rustler::{Encoder, NifStruct};

use crate::traits::NativeWrapper;
use super::{WrapperEx, OutputEx, WrapperExResource, DeviceEx};

use opencl_core::{Dims, Session, Work};
use crate::ex::session_ex::SessionEx;
use crate::ex::number_ex::NumberTyped;


impl WrapperExResource for DeviceBuffer {}


pub enum DeviceBuffer {
    U8{device_mem: DeviceMem<u8>, session: Session, mem_flags: MemFlags, dims: Dims},
    I8{device_mem: DeviceMem<i8>, session: Session, mem_flags: MemFlags, dims: Dims},
    U16{device_mem: DeviceMem<u16>, session: Session, mem_flags: MemFlags, dims: Dims},
    I16{device_mem: DeviceMem<i16>, session: Session, mem_flags: MemFlags, dims: Dims},
    U32{device_mem: DeviceMem<u32>, session: Session, mem_flags: MemFlags, dims: Dims},
    I32{device_mem: DeviceMem<i32>, session: Session, mem_flags: MemFlags, dims: Dims},
    F32{device_mem: DeviceMem<f32>, session: Session, mem_flags: MemFlags, dims: Dims},
    U64{device_mem: DeviceMem<u64>, session: Session, mem_flags: MemFlags, dims: Dims},
    I64{device_mem: DeviceMem<i64>, session: Session, mem_flags: MemFlags, dims: Dims},
    F64{device_mem: DeviceMem<f64>, session: Session, mem_flags: MemFlags, dims: Dims},
    Usize{device_mem: DeviceMem<usize>, session: Session, mem_flags: MemFlags, dims: Dims},
    Isize{device_mem: DeviceMem<isize>, session: Session, mem_flags: MemFlags, dims: Dims},
}

impl NumberTyped for DeviceBuffer {
    use DeviceBuffer as D;
    use NumberType as NT;
    fn number_type(&self) -> NumberType {
        D::U8(..) => NT::U8,
        D::I8(..) => NT::I8,
        D::U16(..) => NT::U16,
        D::I16(..) => NT::I16,
        D::U32(..) => NT::U32,
        D::I32(..) => NT::I32,
        D::F32(..) => NT::F32,
        D::U64(..) => NT::U64,
        D::I64(..) => NT::I64,
        D::F64(..) => NT::F64,
        D::Usize(..) => NT::Usize,
        D::Isize(..) => NT::Isize,
    }
}

macro_rules! create_device_buffer {
    ($variant:ident, $t:ty, $session:ident, $dims:ident, $mem_flags:ident, $data:ident) => {
        DeviceBuffer::$variant{
                
                session: $session.clone(),
                mem_flags: $mem_flags,
                dims: $dims.into(),
                device_mem: DeviceMem<$t>::create_from($session.context(), $mem_flags, $data),
            }
    }
} 

impl DeviceBuffer {
    fn from_number_vector<D>(
        session: Session,
        dims: D,
        mem_flags: MemFlags,
        number_vector: NumberVector,
    ) -> DeviceBuffer where D: Into<Dims> 
    {
        use DeviceBuffer as B;
        use NumberVector as NV;
        match number_vector {
            NV::U8(data) => create_device_buffer(U8, u8, session, dims mem_flags, data),
            NV::I8(data) => B::I8{
                device_mem: DeviceMem<i8>::create_from(session.context(), mem_flags, data),
                session: session.clone(),
                dims: dims.into(),
                mem_flags,
            }
            NV::U16(data) => B::U16{
                device_mem: DeviceMem<u16::create_from>(session.context(), mem_flags, data),
                session: session.clone(),
                dims: dims.into(),
                mem_flags,
            },
            NV::I16(data) => D::I16{
                device_mem: DeviceMem<i16::create_from>(session.context(), mem_flags, data),
                session: session.clone(),
                dims: $dims.into(),
                mem_flags,
            }
            NV::U32(data) => D::U32{
                device_mem: DeviceMem<u32::create_from>(session.context(), mem_flags, data),
                session: session.clone(),
                dims: dims.into(),
                mem_flags,
            }
            NV::I32(data) => D::I32{
                device_mem: DeviceMem<i32::create_from>(session.context(), mem_flags, data),
                session: session.clone()
                dims: dims.into(),
                mem_flags,
            }
            NV::F32(data) => D::F32{
                device_mem: DeviceMem<f32::create_from>(session.context(), mem_flags, data),
                session: session.clone(),
                dims: dims.into(),
                mem_flags,
            }
            NV::U64(data) => D::U64{
                device_mem: DeviceMem<u64::create_from>(session.context(), mem_flags, data),
                session: session.clone(),
                dims: dims.into(),
                mem_flags,
            }
            NV::I64(data) => D::I64{
                device_mem: DeviceMem<i64::create_from>(session.context(), mem_flags, data),
                session: session.clone(),
                dims: dims.into(),
                mem_flags,
            }
            NV::F64(data) => D::F64{
                device_mem: DeviceMem<f64::create_from>(session.context(), mem_flags, data),
                session: session.clone(),
                dims: dims.into(),
                mem_flags,
            }
            NV::Usize(data) => D::Usize{
                device_mem: DeviceMem<usize>::create_fromze>(session.context(), mem_flags, data),
                session: session.clone(),
                dims: dims.into(),
                mem_flags,
            }
            NV::Isize(data) => D::Isize{
                device_mem: DeviceMem<isize>::create_fromze>(session.context(), mem_flags, data),
                session: session.clone(),
                dims: dims.into(),
                mem_flags,
            }
        }
        
    }
}


#[derive(NifStruct)]
#[must_use]
#[module = "OpenCL.DeviceBuffer"]
pub struct DeviceBufferEx<T> {
    __native__: ResourceArc<WrapperEx<DeviceMem<T>>>,
    __session__: ResourceArc<WrapperEx<Session>>
}

impl fmt::Debug for DeviceBufferEx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DeviceBufferEx {{ native: {:?} }}", self.native())
    }
}


impl NativeWrapper<DeviceMem<T>> for DeviceBufferEx<T> {
    fn native(&self) -> &DeviceBuffer {
        &self.__native__.item
    }
}

impl<T> DeviceBufferEx<T> {

    pub fn new(device_mem: DeviceMem<T>) -> DeviceBufferEx<T> {
        DeviceBufferEx {
            __native__: device_buffer.into_resource_arc(),
        }
    }
}

#[rustler::nif]
fn device_buffer_default() -> DeviceBufferEx {
    DeviceBufferEx::default()
}

#[rustler::nif]
fn device_buffer_list_all() -> OutputEx<Vec<DeviceBufferEx>> {
    DeviceBufferEx::all()
}

impl_native_method_into_other_and_nif!(DeviceBufferEx, device_buffer, all_devices, Vec<DeviceEx>);
impl_native_method_into_other_and_nif!(DeviceBufferEx, device_buffer, cpu_devices, Vec<DeviceEx>);
impl_native_method_into_other_and_nif!(DeviceBufferEx, device_buffer, gpu_devices, Vec<DeviceEx>);
impl_native_method_into_other_and_nif!(DeviceBufferEx, device_buffer, accelerator_devices, Vec<DeviceEx>);
impl_native_method_into_other_and_nif!(DeviceBufferEx, device_buffer, custom_devices, Vec<DeviceEx>);

impl_native_method_and_nif!(DeviceBufferEx, device_buffer, name, String);
impl_native_method_and_nif!(DeviceBufferEx, device_buffer, version, String);
impl_native_method_and_nif!(DeviceBufferEx, device_buffer, profile, String);
impl_native_method_and_nif!(DeviceBufferEx, device_buffer, vendor, String);
impl_native_method_and_nif!(DeviceBufferEx, device_buffer, extensions, Vec<String>);
