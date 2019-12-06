use std::default::Default;
use std::fmt;

use opencl_core::Platform;
use rustler::resource::ResourceArc;

use rustler::{Encoder, NifStruct};

use crate::traits::NativeWrapper;

use super::{DeviceEx, OutputEx, WrapperEx, WrapperExResource};

impl WrapperExResource for Platform {}

#[derive(NifStruct)]
#[must_use]
#[module = "OpenCL.Platform"]
pub struct PlatformEx {
    __native__: ResourceArc<WrapperEx<Platform>>,
}

impl fmt::Debug for PlatformEx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PlatformEx {{ native: {:?} }}", self.native())
    }
}

impl Default for PlatformEx {
    fn default() -> PlatformEx {
        PlatformEx::new(Platform::default())
    }
}

impl NativeWrapper<Platform> for PlatformEx {
    fn native(&self) -> &Platform {
        &self.__native__.item
    }
}

impl PlatformEx {
    pub fn all() -> OutputEx<Vec<PlatformEx>> {
        Platform::all()
            .map_err(|e| e.into())
            .map(|platforms| platforms.into_iter().map(|p| PlatformEx::new(p)).collect())
    }

    pub fn new(platform: Platform) -> PlatformEx {
        PlatformEx {
            __native__: platform.into_resource_arc(),
        }
    }
}

#[rustler::nif]
fn platform_default() -> PlatformEx {
    PlatformEx::default()
}

#[rustler::nif]
fn platform_list_all() -> OutputEx<Vec<PlatformEx>> {
    PlatformEx::all()
}

impl_native_method_into_other_and_nif!(PlatformEx, platform, all_devices, Vec<DeviceEx>);
impl_native_method_into_other_and_nif!(PlatformEx, platform, cpu_devices, Vec<DeviceEx>);
impl_native_method_into_other_and_nif!(PlatformEx, platform, gpu_devices, Vec<DeviceEx>);
impl_native_method_into_other_and_nif!(PlatformEx, platform, accelerator_devices, Vec<DeviceEx>);
impl_native_method_into_other_and_nif!(PlatformEx, platform, custom_devices, Vec<DeviceEx>);

impl_native_method_and_nif!(PlatformEx, platform, name, String);
impl_native_method_and_nif!(PlatformEx, platform, version, String);
impl_native_method_and_nif!(PlatformEx, platform, profile, String);
impl_native_method_and_nif!(PlatformEx, platform, vendor, String);
impl_native_method_and_nif!(PlatformEx, platform, extensions, Vec<String>);
