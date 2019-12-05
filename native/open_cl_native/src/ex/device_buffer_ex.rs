use std::default::Default;
use std::fmt;

use opencl_core::DeviceMem;
use rustler::resource::ResourceArc;

use rustler::{Encoder, NifStruct};

use crate::traits::NativeWrapper;

use super::{WrapperEx, OutputEx, WrapperExResource, DeviceEx};


impl WrapperExResource for DeviceBuffer {}

#[derive(NifStruct)]
#[must_use]
#[module = "OpenCL.DeviceBuffer"]
pub struct DeviceBufferEx<T> {
    __native__: ResourceArc<WrapperEx<DeviceMem<T>>>,
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
