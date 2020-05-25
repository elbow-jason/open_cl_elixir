defmodule OpenCL.Native do
  use Rustler, otp_app: :open_cl, crate: :open_cl_native
  use OpenCL.T
  #   alias OpenCL.Array
  #   alias OpenCL.Device
  #   alias OpenCL.Platform

  #   alias OpenCL.Buffer
  #   alias OpenCL.MemConfig
  #   alias OpenCL.KernelOp
  #   alias OpenCL.CommandQueueProps

  import OpenCL.NifNotLoadedError, only: [err: 0]

  # PLATFORM
  @spec platform_default :: T.platform()
  def platform_default, do: err()

  @spec platform_list_all :: T.output([T.platform()])
  def platform_list_all, do: err()

  @spec platform_self_name(T.platform()) :: T.output(String.t())
  def platform_self_name(_platform), do: err()

  @spec platform_self_version(T.platform()) :: T.output(String.t())
  def platform_self_version(_platform), do: err()

  @spec platform_self_profile(T.platform()) :: T.output(String.t())
  def platform_self_profile(_platform), do: err()

  @spec platform_self_vendor(T.platform()) :: T.output(String.t())
  def platform_self_vendor(_platform), do: err()

  @spec platform_self_extensions(T.platform()) :: T.output(String.t())
  def platform_self_extensions(_platform), do: err()

  @spec platform_list_all_devices(T.platform()) :: T.output([Device.t()])
  def platform_list_all_devices(_platform), do: err()

  @spec platform_list_default_devices(T.platform()) :: T.output([Device.t()])
  def platform_list_default_devices(_platform), do: err()

  @spec platform_list_cpu_devices(T.platform()) :: T.output([Device.t()])
  def platform_list_cpu_devices(_platform), do: err()

  @spec platform_list_gpu_devices(T.platform()) :: T.output([Device.t()])
  def platform_list_gpu_devices(_platform), do: err()

  @spec platform_list_accelerator_devices(T.platform()) :: T.output([Device.t()])
  def platform_list_accelerator_devices(_platform), do: err()

  @spec platform_list_custom_devices(T.platform()) :: T.output([Device.t()])
  def platform_list_custom_devices(_platform), do: err()

  @spec platform_self_cpu_devices(T.platform()) :: T.output([Device.t()])
  def platform_self_cpu_devices(_platform), do: err()

  @spec platform_self_gpu_devices(T.platform()) :: T.output([Device.t()])
  def platform_self_gpu_devices(_platform), do: err()

  @spec platform_self_accelerator_devices(T.platform()) :: T.output([Device.t()])
  def platform_self_accelerator_devices(_platform), do: err()

  @spec platform_self_custom_devices(T.platform()) :: T.output([Device.t()])
  def platform_self_custom_devices(_platform), do: err()

  # DEVICE
  @spec device_default :: Device.t()
  def device_default, do: err()

  # device string
  @spec device_self_name(Device.t()) :: T.output(String.t())
  def device_self_name(_device), do: err()

  @spec device_self_version(Device.t()) :: T.output(String.t())
  def device_self_version(_device), do: err()

  @spec device_self_opencl_c_version(Device.t()) :: T.output(String.t())
  def device_self_opencl_c_version(_device), do: err()

  @spec device_self_profile(Device.t()) :: T.output(String.t())
  def device_self_profile(_device), do: err()

  @spec device_self_vendor(Device.t()) :: T.output(String.t())
  def device_self_vendor(_device), do: err()

  @spec device_self_driver_version(Device.t()) :: T.output(String.t())
  def device_self_driver_version(_device), do: err()

  # device u32
  @spec device_self_address_bits(Device.t()) :: non_neg_integer()
  def device_self_address_bits(_device), do: err()

  @spec device_self_global_mem_cacheline_size(Device.t()) :: non_neg_integer()
  def device_self_global_mem_cacheline_size(_device), do: err()

  @spec device_self_max_clock_frequency(Device.t()) :: non_neg_integer()
  def device_self_max_clock_frequency(_device), do: err()

  @spec device_self_max_compute_units(Device.t()) :: non_neg_integer()
  def device_self_max_compute_units(_device), do: err()

  @spec device_self_max_constant_args(Device.t()) :: non_neg_integer()
  def device_self_max_constant_args(_device), do: err()

  @spec device_self_max_read_image_args(Device.t()) :: non_neg_integer()
  def device_self_max_read_image_args(_device), do: err()

  @spec device_self_max_samplers(Device.t()) :: non_neg_integer()
  def device_self_max_samplers(_device), do: err()

  @spec device_self_max_work_item_dimensions(Device.t()) :: non_neg_integer()
  def device_self_max_work_item_dimensions(_device), do: err()

  @spec device_self_max_write_image_args(Device.t()) :: non_neg_integer()
  def device_self_max_write_image_args(_device), do: err()

  @spec device_self_mem_base_addr_align(Device.t()) :: non_neg_integer()
  def device_self_mem_base_addr_align(_device), do: err()

  @spec device_self_min_data_type_align_size(Device.t()) :: non_neg_integer()
  def device_self_min_data_type_align_size(_device), do: err()

  @spec device_self_native_vector_width_char(Device.t()) :: non_neg_integer()
  def device_self_native_vector_width_char(_device), do: err()

  @spec device_self_native_vector_width_short(Device.t()) :: non_neg_integer()
  def device_self_native_vector_width_short(_device), do: err()

  @spec device_self_native_vector_width_int(Device.t()) :: non_neg_integer()
  def device_self_native_vector_width_int(_device), do: err()

  @spec device_self_native_vector_width_long(Device.t()) :: non_neg_integer()
  def device_self_native_vector_width_long(_device), do: err()

  @spec device_self_native_vector_width_float(Device.t()) :: non_neg_integer()
  def device_self_native_vector_width_float(_device), do: err()

  @spec device_self_native_vector_width_double(Device.t()) :: non_neg_integer()
  def device_self_native_vector_width_double(_device), do: err()

  @spec device_self_native_vector_width_half(Device.t()) :: non_neg_integer()
  def device_self_native_vector_width_half(_device), do: err()

  @spec device_self_partition_max_sub_devices(Device.t()) :: non_neg_integer()
  def device_self_partition_max_sub_devices(_device), do: err()

  @spec device_self_preferred_vector_width_char(Device.t()) :: non_neg_integer()
  def device_self_preferred_vector_width_char(_device), do: err()

  @spec device_self_preferred_vector_width_short(Device.t()) :: non_neg_integer()
  def device_self_preferred_vector_width_short(_device), do: err()

  @spec device_self_preferred_vector_width_int(Device.t()) :: non_neg_integer()
  def device_self_preferred_vector_width_int(_device), do: err()

  @spec device_self_preferred_vector_width_long(Device.t()) :: non_neg_integer()
  def device_self_preferred_vector_width_long(_device), do: err()

  @spec device_self_preferred_vector_width_float(Device.t()) :: non_neg_integer()
  def device_self_preferred_vector_width_float(_device), do: err()

  @spec device_self_preferred_vector_width_double(Device.t()) :: non_neg_integer()
  def device_self_preferred_vector_width_double(_device), do: err()

  @spec device_self_preferred_vector_width_half(Device.t()) :: non_neg_integer()
  def device_self_preferred_vector_width_half(_device), do: err()

  @spec device_self_vendor_id(Device.t()) :: non_neg_integer()
  def device_self_vendor_id(_device), do: err()

  # device bool
  @spec device_self_available(Device.t()) :: boolean()
  def device_self_available(_device), do: err()

  @spec device_self_compiler_available(Device.t()) :: boolean()
  def device_self_compiler_available(_device), do: err()

  @spec device_self_endian_little(Device.t()) :: boolean()
  def device_self_endian_little(_device), do: err()

  @spec device_self_error_correction_support(Device.t()) :: boolean()
  def device_self_error_correction_support(_device), do: err()

  @spec device_self_host_unified_memory(Device.t()) :: boolean()
  def device_self_host_unified_memory(_device), do: err()

  @spec device_self_image_support(Device.t()) :: boolean()
  def device_self_image_support(_device), do: err()

  @spec device_self_linker_available(Device.t()) :: boolean()
  def device_self_linker_available(_device), do: err()

  @spec device_self_preferred_interop_user_sync(Device.t()) :: boolean()
  def device_self_preferred_interop_user_sync(_device), do: err()

  # device usize
  @spec device_self_image2d_max_width(Device.t()) :: non_neg_integer()
  def device_self_image2d_max_width(_device), do: err()

  @spec device_self_image2d_max_height(Device.t()) :: non_neg_integer()
  def device_self_image2d_max_height(_device), do: err()

  @spec device_self_image3d_max_width(Device.t()) :: non_neg_integer()
  def device_self_image3d_max_width(_device), do: err()

  @spec device_self_image3d_max_height(Device.t()) :: non_neg_integer()
  def device_self_image3d_max_height(_device), do: err()

  @spec device_self_image3d_max_depth(Device.t()) :: non_neg_integer()
  def device_self_image3d_max_depth(_device), do: err()

  @spec device_self_image_max_buffer_size(Device.t()) :: non_neg_integer()
  def device_self_image_max_buffer_size(_device), do: err()

  @spec device_self_image_max_array_size(Device.t()) :: non_neg_integer()
  def device_self_image_max_array_size(_device), do: err()

  @spec device_self_max_parameter_size(Device.t()) :: non_neg_integer()
  def device_self_max_parameter_size(_device), do: err()

  @spec device_self_max_work_group_size(Device.t()) :: non_neg_integer()
  def device_self_max_work_group_size(_device), do: err()

  @spec device_self_printf_buffer_size(Device.t()) :: non_neg_integer()
  def device_self_printf_buffer_size(_device), do: err()

  @spec device_self_profiling_timer_resolution(Device.t()) :: non_neg_integer()
  def device_self_profiling_timer_resolution(_device), do: err()

  # TODO: Fix device_self_max_work_item_sizes typespec
  @spec device_self_max_work_item_sizes(Device.t()) :: [non_neg_integer()]
  def device_self_max_work_item_sizes(_device), do: err()

  @type partition_affinity_domain ::
          :numa
          | :l4_cache
          | :l3_cache
          | :l2_cache
          | :l1_cache
          | :next_partitionable

  @spec device_self_partition_affinity_domain(Device.t()) :: [partition_affinity_domain()]
  def device_self_partition_affinity_domain(_device), do: err()

  @spec device_self_device_type(Device.t()) :: integer()
  def device_self_device_type(_device), do: err()

  @spec device_self_global_mem_cache_type(Device.t()) :: integer()
  def device_self_global_mem_cache_type(_device), do: err()

  @spec device_self_local_mem_type(Device.t()) :: integer()
  def device_self_local_mem_type(_device), do: err()

  # device
  @spec device_self_platform(Device.t()) :: T.platform()
  def device_self_platform(_device), do: err()

  # SESSION

  @spec session_create_with_devices([Device.t()], String.t(), CommandQueueProps.t()) ::
          T.output(Session.t())
  def session_create_with_devices(_devices, _src, _props), do: err()

  @spec session_create(String.t(), CommandQueueProps.t()) :: T.output(Session.t())
  def session_create(_src, _props), do: err()

  @spec session_self_create_copy(Session.t()) :: T.output(Session.t())
  def session_self_create_copy(_session), do: err()

  @spec session_self_device(Session.t()) :: [Device.t()]
  def session_self_device(_session), do: err()

  @type len :: non_neg_integer()

  @spec session_self_create_buffer_from_list(
          Session.t(),
          T.number_type(),
          T.num_list(),
          MemConfig.t() | nil
        ) :: T.output(Buffer.t())
  def session_self_create_buffer_from_list(_session, _type, _list_of_numbers, _config), do: err()

  @spec session_self_create_buffer_from_array(Session.t(), T.array(), MemConfig.t() | nil) ::
          T.output(Buffer.t())
  def session_self_create_buffer_from_array(_session, _array, _config), do: err()

  @spec session_self_create_buffer_with_length(
          Session.t(),
          T.number_type(),
          T.len(),
          MemConfig.t() | nil
        ) :: T.output(Buffer.t())
  def session_self_create_buffer_with_length(_session, _type, _len, _config), do: err()

  @spec session_self_write_array_to_buffer(
          Session.t(),
          Buffer.t(),
          T.array(),
          CommandQueueOpts.native()
        ) ::
          T.output({})
  def session_self_write_array_to_buffer(_session, _buffer, _array, _cq_opts), do: err()

  @spec session_self_read_buffer(Session.t(), Buffer.t(), CommandQueueOpts.native()) ::
          T.output(Array.t())
  def session_self_read_buffer(_session, _buffer, _cq_opts), do: err()

  @spec session_self_execute_kernel_operation(Session.t(), KernelOp.t()) :: T.output({})
  def session_self_execute_kernel_operation(_session, _kernel_op), do: err()

  #   @spec session_self_device_name(Session.t()) :: T.output(String.t())
  #   def session_self_device_name(_session), do: err()

  #   @spec session_self_device_opencl_c_version(Session.t()) :: T.output(String.t())
  #   def session_self_device_opencl_c_version(_session), do: err()

  #   @spec session_self_device_profile(Session.t()) :: T.output(String.t())
  #   def session_self_device_profile(_session), do: err()

  #   @spec session_self_device_vendor(Session.t()) :: T.output(String.t())
  #   def session_self_device_vendor(_session), do: err()

  #   @spec session_self_device_version(Session.t()) :: T.output(String.t())
  #   def session_self_device_version(_session), do: err()

  #   @spec session_self_device_driver_version(Session.t()) :: T.output(String.t())
  #   def session_self_device_driver_version(_session), do: err()

  #   @spec session_self_device_address_bits(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_address_bits(_session), do: err()

  #   @spec session_self_device_global_mem_cacheline_size(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_global_mem_cacheline_size(_session), do: err()

  #   @spec session_self_device_max_clock_frequency(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_max_clock_frequency(_session), do: err()

  #   @spec session_self_device_max_compute_units(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_max_compute_units(_session), do: err()

  #   @spec session_self_device_max_constant_args(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_max_constant_args(_session), do: err()

  #   @spec session_self_device_max_read_image_args(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_max_read_image_args(_session), do: err()

  #   @spec session_self_device_max_samplers(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_max_samplers(_session), do: err()

  #   @spec session_self_device_max_work_item_dimensions(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_max_work_item_dimensions(_session), do: err()

  #   @spec session_self_device_max_write_image_args(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_max_write_image_args(_session), do: err()

  #   @spec session_self_device_mem_base_addr_align(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_mem_base_addr_align(_session), do: err()

  #   @spec session_self_device_min_data_type_align_size(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_min_data_type_align_size(_session), do: err()

  #   @spec session_self_device_native_vector_width_char(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_native_vector_width_char(_session), do: err()

  #   @spec session_self_device_native_vector_width_short(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_native_vector_width_short(_session), do: err()

  #   @spec session_self_device_native_vector_width_int(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_native_vector_width_int(_session), do: err()

  #   @spec session_self_device_native_vector_width_long(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_native_vector_width_long(_session), do: err()

  #   @spec session_self_device_native_vector_width_float(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_native_vector_width_float(_session), do: err()

  #   @spec session_self_device_native_vector_width_double(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_native_vector_width_double(_session), do: err()

  #   @spec session_self_device_native_vector_width_half(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_native_vector_width_half(_session), do: err()

  #   @spec session_self_device_partition_max_sub_devices(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_partition_max_sub_devices(_session), do: err()

  #   @spec session_self_device_preferred_vector_width_char(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_preferred_vector_width_char(_session), do: err()

  #   @spec session_self_device_preferred_vector_width_short(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_preferred_vector_width_short(_session), do: err()

  #   @spec session_self_device_preferred_vector_width_int(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_preferred_vector_width_int(_session), do: err()

  #   @spec session_self_device_preferred_vector_width_long(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_preferred_vector_width_long(_session), do: err()

  #   @spec session_self_device_preferred_vector_width_float(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_preferred_vector_width_float(_session), do: err()

  #   @spec session_self_device_preferred_vector_width_double(Session.t()) ::
  #           T.output(non_neg_integer())
  #   def session_self_device_preferred_vector_width_double(_session), do: err()

  #   @spec session_self_device_preferred_vector_width_half(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_preferred_vector_width_half(_session), do: err()

  #   @spec session_self_device_vendor_id(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_vendor_id(_session), do: err()

  #   @spec session_self_device_available(Session.t()) :: T.output(boolean())
  #   def session_self_device_available(_session), do: err()

  #   @spec session_self_device_compiler_available(Session.t()) :: T.output(boolean())
  #   def session_self_device_compiler_available(_session), do: err()

  #   @spec session_self_device_endian_little(Session.t()) :: T.output(boolean())
  #   def session_self_device_endian_little(_session), do: err()

  #   @spec session_self_device_error_correction_support(Session.t()) :: T.output(boolean())
  #   def session_self_device_error_correction_support(_session), do: err()

  #   @spec session_self_device_host_unified_memory(Session.t()) :: T.output(boolean())
  #   def session_self_device_host_unified_memory(_session), do: err()

  #   @spec session_self_device_image_support(Session.t()) :: T.output(boolean())
  #   def session_self_device_image_support(_session), do: err()

  #   @spec session_self_device_linker_available(Session.t()) :: T.output(boolean())
  #   def session_self_device_linker_available(_session), do: err()

  #   @spec session_self_device_preferred_interop_user_sync(Session.t()) :: T.output(boolean())
  #   def session_self_device_preferred_interop_user_sync(_session), do: err()

  #   @spec session_self_device_image2d_max_width(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_image2d_max_width(_session), do: err()

  #   @spec session_self_device_image2d_max_height(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_image2d_max_height(_session), do: err()

  #   @spec session_self_device_image3d_max_width(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_image3d_max_width(_session), do: err()

  #   @spec session_self_device_image3d_max_height(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_image3d_max_height(_session), do: err()

  #   @spec session_self_device_image3d_max_depth(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_image3d_max_depth(_session), do: err()

  #   @spec session_self_device_image_max_buffer_size(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_image_max_buffer_size(_session), do: err()

  #   @spec session_self_device_image_max_array_size(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_image_max_array_size(_session), do: err()

  #   @spec session_self_device_max_parameter_size(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_max_parameter_size(_session), do: err()

  #   @spec session_self_device_max_work_group_size(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_max_work_group_size(_session), do: err()

  #   @spec session_self_device_printf_buffer_size(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_printf_buffer_size(_session), do: err()

  #   @spec session_self_device_profiling_timer_resolution(Session.t()) :: T.output(non_neg_integer())
  #   def session_self_device_profiling_timer_resolution(_session), do: err()

  #   @spec session_self_device_max_work_item_sizes(Session.t()) :: T.output([non_neg_integer()])
  #   def session_self_device_max_work_item_sizes(_session), do: err()

  #   @type dim :: non_neg_integer()
  #   @type(dims :: dim() | {dim()} | {dim(), dim()}, {dim(), dim(), dim()})

  # ARRAY
  @spec array_new(T.number_type(), [number(), ...]) :: T.array()
  def array_new(_number_type, _numbers), do: err()

  @spec array_new_filled_with(T.number_type(), number(), non_neg_integer()) :: T.array()
  def array_new_filled_with(_number_type, _number, _count), do: err()

  def array_data(_array), do: err()

  def array_length(_array), do: err()

  def array_push(_array, _number), do: err()

  def array_extend_from_list(_array, _list_of_numbers), do: err()

  def array_extend_from_array(_array, _other), do: err()

  def array_number_type(_array), do: err()

  def array_cast(_array, _number_type), do: err()

  #   @type buffer_access :: :read_only | :write_only | :read_write

  def buffer_self_length(_buffer), do: err()

  def buffer_self_number_type(_buffer), do: err()

  def buffer_self_mem_config(_buffer), do: err()

  def buffer_self_reference_count(_buffer), do: err()

  def buffer_self_available_devices(_buffer), do: err()
end
