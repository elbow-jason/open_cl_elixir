#![allow(dead_code)]


extern crate log;
#[macro_use]
extern crate failure;
extern crate ndarray;
extern crate num;
extern crate num_complex;

// use ndarray::prelude::*;

use rustler::{Env, Term};

mod atoms;

#[macro_use]
mod macros;
mod ex;
mod number;
mod traits;

pub use ex::*;
pub use number::*;

rustler::init! {
    "Elixir.OpenCL.Native",
    [
        // platform
        platform_ex::platform_default,
        platform_ex::platform_list_all,
        platform_ex::platform_self_name,
        platform_ex::platform_self_version,
        platform_ex::platform_self_profile,
        platform_ex::platform_self_vendor,
        platform_ex::platform_self_extensions,

        platform_ex::platform_list_all_devices,
        platform_ex::platform_list_default_devices,
        platform_ex::platform_list_cpu_devices,
        platform_ex::platform_list_gpu_devices,
        platform_ex::platform_list_accelerator_devices,
        platform_ex::platform_list_custom_devices,


        // device_ex::device_default,
        device_ex::device_self_is_usable,

        device_ex::device_self_name,
        device_ex::device_self_version,
        device_ex::device_self_opencl_c_version,
        device_ex::device_self_profile,
        device_ex::device_self_vendor,
        device_ex::device_self_driver_version,
        device_ex::device_self_address_bits,
        device_ex::device_self_global_mem_cacheline_size,
        device_ex::device_self_max_clock_frequency,
        device_ex::device_self_max_compute_units,
        device_ex::device_self_max_constant_args,
        device_ex::device_self_max_read_image_args,
        device_ex::device_self_max_samplers,
        device_ex::device_self_max_work_item_dimensions,
        device_ex::device_self_max_write_image_args,
        device_ex::device_self_mem_base_addr_align,
        device_ex::device_self_min_data_type_align_size,
        device_ex::device_self_native_vector_width_char,
        device_ex::device_self_native_vector_width_short,
        device_ex::device_self_native_vector_width_int,
        device_ex::device_self_native_vector_width_long,
        device_ex::device_self_native_vector_width_float,
        device_ex::device_self_native_vector_width_double,
        device_ex::device_self_native_vector_width_half,
        device_ex::device_self_partition_max_sub_devices,
        device_ex::device_self_preferred_vector_width_char,
        device_ex::device_self_preferred_vector_width_short,
        device_ex::device_self_preferred_vector_width_int,
        device_ex::device_self_preferred_vector_width_long,
        device_ex::device_self_preferred_vector_width_float,
        device_ex::device_self_preferred_vector_width_double,
        device_ex::device_self_preferred_vector_width_half,
        device_ex::device_self_vendor_id,
        device_ex::device_self_available,
        device_ex::device_self_compiler_available,
        device_ex::device_self_endian_little,
        device_ex::device_self_error_correction_support,
        device_ex::device_self_host_unified_memory,
        device_ex::device_self_image_support,
        device_ex::device_self_linker_available,
        device_ex::device_self_preferred_interop_user_sync,
        device_ex::device_self_image2d_max_width,
        device_ex::device_self_image2d_max_height,
        device_ex::device_self_image3d_max_width,
        device_ex::device_self_image3d_max_height,
        device_ex::device_self_image3d_max_depth,
        device_ex::device_self_image_max_buffer_size,
        device_ex::device_self_image_max_array_size,
        device_ex::device_self_max_parameter_size,
        device_ex::device_self_max_work_group_size,
        device_ex::device_self_printf_buffer_size,
        device_ex::device_self_profiling_timer_resolution,
        device_ex::device_self_max_work_item_sizes,
        // device_ex::device_self_partition_affinity_domain,

        session_ex::session_create,
        session_ex::session_create_with_devices,
        session_ex::session_self_device,
        session_ex::session_self_create_buffer,
        session_ex::session_self_write_array_to_buffer,
        session_ex::session_self_read_buffer,
        session_ex::session_self_execute_kernel_operation,
        session_ex::session_self_create_copy,
        // session_ex::session_self_device_name,
        // session_ex::session_self_device_opencl_c_version,
        // session_ex::session_self_device_profile,
        // session_ex::session_self_device_vendor,
        // session_ex::session_self_device_version,
        // session_ex::session_self_device_driver_version,
        // session_ex::session_self_device_address_bits,
        // session_ex::session_self_device_global_mem_cacheline_size,
        // session_ex::session_self_device_max_clock_frequency,
        // session_ex::session_self_device_max_compute_units,
        // session_ex::session_self_device_max_constant_args,
        // session_ex::session_self_device_max_read_image_args,
        // session_ex::session_self_device_max_samplers,
        // session_ex::session_self_device_max_work_item_dimensions,
        // session_ex::session_self_device_max_write_image_args,
        // session_ex::session_self_device_mem_base_addr_align,
        // session_ex::session_self_device_min_data_type_align_size,
        // session_ex::session_self_device_native_vector_width_char,
        // session_ex::session_self_device_native_vector_width_short,
        // session_ex::session_self_device_native_vector_width_int,
        // session_ex::session_self_device_native_vector_width_long,
        // session_ex::session_self_device_native_vector_width_float,
        // session_ex::session_self_device_native_vector_width_double,
        // session_ex::session_self_device_native_vector_width_half,
        // session_ex::session_self_device_partition_max_sub_devices,
        // session_ex::session_self_device_preferred_vector_width_char,
        // session_ex::session_self_device_preferred_vector_width_short,
        // session_ex::session_self_device_preferred_vector_width_int,
        // session_ex::session_self_device_preferred_vector_width_long,
        // session_ex::session_self_device_preferred_vector_width_float,
        // session_ex::session_self_device_preferred_vector_width_double,
        // session_ex::session_self_device_preferred_vector_width_half,
        // session_ex::session_self_device_vendor_id,
        // session_ex::session_self_device_available,
        // session_ex::session_self_device_compiler_available,
        // session_ex::session_self_device_endian_little,
        // session_ex::session_self_device_error_correction_support,
        // session_ex::session_self_device_host_unified_memory,
        // session_ex::session_self_device_image_support,
        // session_ex::session_self_device_linker_available,
        // session_ex::session_self_device_preferred_interop_user_sync,
        // session_ex::session_self_device_image2d_max_width,
        // session_ex::session_self_device_image2d_max_height,
        // session_ex::session_self_device_image3d_max_width,
        // session_ex::session_self_device_image3d_max_height,
        // session_ex::session_self_device_image3d_max_depth,
        // session_ex::session_self_device_image_max_buffer_size,
        // session_ex::session_self_device_image_max_array_size,
        // session_ex::session_self_device_max_parameter_size,
        // session_ex::session_self_device_max_work_group_size,
        // session_ex::session_self_device_printf_buffer_size,
        // session_ex::session_self_device_profiling_timer_resolution,
        // session_ex::session_self_device_max_work_item_sizes,

        // ARRAY
        array_ex::array_new,
        array_ex::array_new_filled_with,
        array_ex::array_push,
        array_ex::array_data,
        array_ex::array_length,
        array_ex::array_extend_from_list,
        array_ex::array_extend_from_array,
        array_ex::array_number_type,
        array_ex::array_cast,

        // DEVICE_BUFFER
        buffer_ex::buffer_length,
        // device_buffer_ex::buffer_to_array,
        // device_buffer_ex::buffer_reference_count,

    ],
    load = load
}

fn load<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    env_logger::init();

    // debug!("this is a debug {}", "message");
    // error!("this is printed by default");
    ex::define_resources(env);
    true
}
