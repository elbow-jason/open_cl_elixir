use std::default::Default;
use std::fmt;
use opencl_core::Device;

use rustler::resource::ResourceArc;
use rustler::{Encoder, NifStruct, NifUnitEnum};

use super::{
    WrapperEx,
    WrapperExResource,
    OutputEx,
};

use crate::traits::{NativeWrapper, BitflagEx};

// use crate::platform_ex::PlatformEx;

impl WrapperExResource for Device {}

#[derive(NifStruct)]
#[must_use]
#[module = "OpenCL.Device"]
pub struct DeviceEx {
    __native__: ResourceArc<WrapperEx<Device>>,
}

impl fmt::Debug for DeviceEx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DeviceEx {{ native: {:?} }}", self.native())
    }
}

impl Default for DeviceEx {
    fn default() -> DeviceEx {
        DeviceEx::new(Device::default())
    }
}

impl NativeWrapper<Device> for DeviceEx {
    fn native(&self) -> &Device {
        &self.__native__.item
    }
}

impl DeviceEx {
    pub fn new(device: Device) -> DeviceEx {
        DeviceEx {
            __native__: device.into_resource_arc(),
        }
    }

    pub fn is_usable(&self) -> bool {
        self.native().is_usable()
    }
}


// device
#[rustler::nif]
fn device_default() -> DeviceEx {
    DeviceEx::default()
}

#[rustler::nif]
fn device_self_is_usable(device: DeviceEx) -> bool {
    device.is_usable()
}

impl_native_method_and_nif!(DeviceEx, device, name, String);
impl_native_method_and_nif!(DeviceEx, device, opencl_c_version, String);
impl_native_method_and_nif!(DeviceEx, device, profile, String);
impl_native_method_and_nif!(DeviceEx, device, vendor, String);
impl_native_method_and_nif!(DeviceEx, device, version, String);
impl_native_method_and_nif!(DeviceEx, device, driver_version, String);

impl_native_method_and_nif!(DeviceEx, device, address_bits, u32);
impl_native_method_and_nif!(DeviceEx, device, global_mem_cacheline_size, u32);
impl_native_method_and_nif!(DeviceEx, device, max_clock_frequency, u32);
impl_native_method_and_nif!(DeviceEx, device, max_compute_units, u32);
impl_native_method_and_nif!(DeviceEx, device, max_constant_args, u32);
impl_native_method_and_nif!(DeviceEx, device, max_read_image_args, u32);
impl_native_method_and_nif!(DeviceEx, device, max_samplers, u32);
impl_native_method_and_nif!(DeviceEx, device, max_work_item_dimensions, u32);
impl_native_method_and_nif!(DeviceEx, device, max_write_image_args, u32);
impl_native_method_and_nif!(DeviceEx, device, mem_base_addr_align, u32);
impl_native_method_and_nif!(DeviceEx, device, min_data_type_align_size, u32);
impl_native_method_and_nif!(DeviceEx, device, native_vector_width_char, u32);
impl_native_method_and_nif!(DeviceEx, device, native_vector_width_short, u32);
impl_native_method_and_nif!(DeviceEx, device, native_vector_width_int, u32);
impl_native_method_and_nif!(DeviceEx, device, native_vector_width_long, u32);
impl_native_method_and_nif!(DeviceEx, device, native_vector_width_float, u32);
impl_native_method_and_nif!(DeviceEx, device, native_vector_width_double, u32);
impl_native_method_and_nif!(DeviceEx, device, native_vector_width_half, u32);
impl_native_method_and_nif!(DeviceEx, device, partition_max_sub_devices, u32);
impl_native_method_and_nif!(DeviceEx, device, preferred_vector_width_char, u32);
impl_native_method_and_nif!(DeviceEx, device, preferred_vector_width_short, u32);
impl_native_method_and_nif!(DeviceEx, device, preferred_vector_width_int, u32);
impl_native_method_and_nif!(DeviceEx, device, preferred_vector_width_long, u32);
impl_native_method_and_nif!(DeviceEx, device, preferred_vector_width_float, u32);
impl_native_method_and_nif!(DeviceEx, device, preferred_vector_width_double, u32);
impl_native_method_and_nif!(DeviceEx, device, preferred_vector_width_half, u32);
impl_native_method_and_nif!(DeviceEx, device, vendor_id, u32);

impl_native_method_and_nif!(DeviceEx, device, available, bool);
impl_native_method_and_nif!(DeviceEx, device, compiler_available, bool);
impl_native_method_and_nif!(DeviceEx, device, endian_little, bool);
impl_native_method_and_nif!(DeviceEx, device, error_correction_support, bool);
impl_native_method_and_nif!(DeviceEx, device, host_unified_memory, bool);
impl_native_method_and_nif!(DeviceEx, device, image_support, bool);
impl_native_method_and_nif!(DeviceEx, device, linker_available, bool);
impl_native_method_and_nif!(DeviceEx, device, preferred_interop_user_sync, bool);

impl_native_method_and_nif!(DeviceEx, device, image2d_max_width, usize);
impl_native_method_and_nif!(DeviceEx, device, image2d_max_height, usize);
impl_native_method_and_nif!(DeviceEx, device, image3d_max_width, usize);
impl_native_method_and_nif!(DeviceEx, device, image3d_max_height, usize);
impl_native_method_and_nif!(DeviceEx, device, image3d_max_depth, usize);
impl_native_method_and_nif!(DeviceEx, device, image_max_buffer_size, usize);
impl_native_method_and_nif!(DeviceEx, device, image_max_array_size, usize);
impl_native_method_and_nif!(DeviceEx, device, max_parameter_size, usize);
impl_native_method_and_nif!(DeviceEx, device, max_work_group_size, usize);
impl_native_method_and_nif!(DeviceEx, device, printf_buffer_size, usize);
impl_native_method_and_nif!(DeviceEx, device, profiling_timer_resolution, usize);

impl_native_method_and_nif!(DeviceEx, device, max_work_item_sizes, Vec<usize>);

use opencl_core::device::flags::{
    DeviceAffinityDomain,
    // DeviceType,
    // DeviceMemCacheType,
    // DeviceLocalMemType,
};

#[derive(NifUnitEnum, Clone)]
pub enum DeviceAffinityDomainEx {
    Numa,
    L4Cache,
    L3Cache,
    L2Cache,
    L1Cache,
    NextPartitionable,
}

const DEVICE_AFFINITY_DOMAIN_EX_MAPPING: [(DeviceAffinityDomain, DeviceAffinityDomainEx); 6] = [
    (DeviceAffinityDomain::NUMA, DeviceAffinityDomainEx::Numa),
    (DeviceAffinityDomain::L4_CACHE, DeviceAffinityDomainEx::L4Cache),
    (DeviceAffinityDomain::L3_CACHE, DeviceAffinityDomainEx::L3Cache),
    (DeviceAffinityDomain::L2_CACHE, DeviceAffinityDomainEx::L2Cache),
    (DeviceAffinityDomain::L1_CACHE, DeviceAffinityDomainEx::L1Cache),
    (DeviceAffinityDomain::NEXT_PARTITIONABLE, DeviceAffinityDomainEx::NextPartitionable),
];

impl_bitflag_ex_for!(DeviceAffinityDomainEx, DeviceAffinityDomain, DEVICE_AFFINITY_DOMAIN_EX_MAPPING);


impl_native_method_into_bitflag_and_nif!(DeviceEx, device, partition_affinity_domain, DeviceAffinityDomainEx);

// impl DeviceAffinityDomainEx {
//     fn list(flag: DeviceAffinityDomain) -> Vec<DeviceAffinityDomainEx> {
//         bit_flag_to_ex(flag, DEVICE_AFFINITY_DOMAIN_EX_MAPPING)
//         // use DeviceAffinityDomainEx as Ex;
//         // use DeviceAffinityDomain as Cl;
        
//         // let mut output: Vec<Ex> = Vec::new();
//         // if aff.contains(Cl::NUMA) {
//         //     output.push(Ex::Numa)
//         // }
//         // if aff.contains(Cl::L4_CACHE) {
//         //     output.push(Ex::L4Cache)
//         // }
//         // if aff.contains(Cl::L3_CACHE) {
//         //     output.push(Ex::L3Cache)
//         // }
//         // if aff.contains(Cl::L2_CACHE) {
//         //     output.push(Ex::L2Cache)
//         // }
//         // if aff.contains(Cl::L1_CACHE) {
//         //     output.push(Ex::L1Cache)
//         // }
//         // if aff.contains(Cl::NEXT_PARTITIONABLE) {
//         //     output.push(Ex::NextPartitionable)
//         // }
     
//     }
// }


// impl DeviceEx {
//     pub fn partition_affinity_domain(&self) -> OutputEx<Vec<DeviceAffinityDomainEx>> {
//         let aff: DeviceAffinityDomain = self
//             .native()
//             .partition_affinity_domain()
//             .map_err(|e| {
//                 let ee: ErrorEx = e.into();
//                 ee
//             })?;
//         Ok(DeviceAffinityDomainEx::list(aff))
//     }
// }

// // impl Encoder for DeviceType {
// //     fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
// //         1.encode(env)
// //     }
// // }

// // impl Encoder for DeviceMemCacheType {
// //     fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
// //         1.encode(env)
// //     }
// // }

// // impl Encoder for DeviceLocalMemType {
// //     fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
// //         1.encode(env)
// //     }
// // }
//         // const NONE = 0;
//         // const NUMA = 1;
//         // const L4_CACHE = 2;
//         // const L3_CACHE = 4;
//         // const L2_CACHE = 8;
//         // const L1_CACHE = 16;
//         // const NEXT_PARTITIONABLE = 32;





// impl_native_method!(DeviceEx, device_type, DeviceType);
// impl_native_method!(DeviceEx, global_mem_cache_type, DeviceMemCacheType);
// impl_native_method!(DeviceEx, local_mem_type, DeviceLocalMemType);

// // device Platform
// impl_native_method_into_other!(DeviceEx, platform, PlatformEx);

