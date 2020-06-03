use super::{WrapperEx, WrapperExResource};
use crate::nif;
use crate::nif::ErrorT;
use crate::DeviceEx;
use open_cl_core::{Device, Platform};
use std::default::Default;
use std::fmt;

impl WrapperExResource for Platform {}

#[derive(nif::Struct)]
#[must_use]
#[module = "OpenCL.Platform"]
pub struct PlatformEx {
    __native__: nif::ResourceArc<WrapperEx<Platform>>,
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

impl PlatformEx {
    fn native(&self) -> &Platform {
        &self.__native__.item
    }
}

impl PlatformEx {
    pub fn list_all() -> nif::Result<Vec<PlatformEx>> {
        Platform::list_all()
            .map_err(|e| e.error())
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
fn platform_list_all() -> nif::Result<Vec<PlatformEx>> {
    PlatformEx::list_all()
}

macro_rules! list_devices {
    ($func_name:ident) => {
        paste::item! {
            #[rustler::nif]
            fn [<platform_ $func_name>](platform: PlatformEx) -> nif::Result<Vec<DeviceEx>> {
                Device::$func_name(platform.native())
                    .map(|devices| {
                        devices.into_iter().map(|d| DeviceEx::new(d)).collect()
                    })
                    .map_err(|e| $crate::nif::ErrorT::error(e))
            }
        }
    };
}

list_devices!(list_all_devices);
list_devices!(list_default_devices);
list_devices!(list_cpu_devices);
list_devices!(list_gpu_devices);
list_devices!(list_accelerator_devices);
list_devices!(list_custom_devices);

impl_native_method_and_nif!(PlatformEx, platform, name, String);
impl_native_method_and_nif!(PlatformEx, platform, version, String);
impl_native_method_and_nif!(PlatformEx, platform, profile, String);
impl_native_method_and_nif!(PlatformEx, platform, vendor, String);
impl_native_method_and_nif!(PlatformEx, platform, extensions, Vec<String>);
