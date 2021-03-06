use std::default::Default;
use std::fmt;

use opencl_core::ll::ClPlatformID;
use opencl_core::{Device, Platform};
use rustler::resource::ResourceArc;

use rustler::{Encoder, NifStruct};

use crate::traits::NativeWrapper;

use super::DeviceEx; // , OutputEx, WrapperEx, WrapperExResource};
use super::{OutputEx, WrapperEx, WrapperExResource};
use crate::traits::LowLevelWrapper;
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

impl LowLevelWrapper<ClPlatformID> for PlatformEx {
    fn low_level(&self) -> &ClPlatformID {
        self.__native__.item.low_level_platform()
    }
}

impl PlatformEx {
    pub fn list_all() -> OutputEx<Vec<PlatformEx>> {
        Platform::list_all()
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
    PlatformEx::list_all()
}

macro_rules! list_devices {
    ($func_name:ident) => {
        paste::item! {
            #[rustler::nif]
            fn [<platform_ $func_name>](platform: PlatformEx) -> OutputEx<Vec<DeviceEx>> {
                Device::$func_name(platform.native())
                    .map(|devices| {
                        devices.into_iter().map(|d| DeviceEx::new(d)).collect()
                    })
                    .map_err(|e| e.into())
            }
        }
    };
}
// #[rustler::nif]
// fn platform_list_all_devices(platform: PlatformEx) -> OutputEx<Vec<DeviceEx>> {
//     Device::list_all_devices(platform.native())
//         .map(|devices| {
//             devices.into_iter().map(|d| DeviceEx::new(d)).collect()
//         })
//         .map_err(|e| e.into())
// }

list_devices!(list_all_devices);
list_devices!(list_default_devices);
list_devices!(list_cpu_devices);
list_devices!(list_gpu_devices);
list_devices!(list_accelerator_devices);
list_devices!(list_custom_devices);
// impl_native_method_into_other_and_nif!(PlatformEx, platform, list_all_devices, Vec<DeviceEx>);
// impl_native_method_into_other_and_nif!(PlatformEx, platform, cpu_devices, Vec<DeviceEx>);
// impl_native_method_into_other_and_nif!(PlatformEx, platform, gpu_devices, Vec<DeviceEx>);
// impl_native_method_into_other_and_nif!(PlatformEx, platform, accelerator_devices, Vec<DeviceEx>);
// impl_native_method_into_other_and_nif!(PlatformEx, platform, custom_devices, Vec<DeviceEx>);

impl_native_method_and_nif!(PlatformEx, platform, name, String);
impl_native_method_and_nif!(PlatformEx, platform, version, String);
impl_native_method_and_nif!(PlatformEx, platform, profile, String);
impl_native_method_and_nif!(PlatformEx, platform, vendor, String);
impl_native_method_and_nif!(PlatformEx, platform, extensions, Vec<String>);
