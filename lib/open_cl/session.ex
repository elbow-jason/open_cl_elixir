defmodule OpenCL.Session do
  use OpenCL.NativeStruct
  alias OpenCL.Device
  alias OpenCL.Session

  @spec create_with_src(Device.t(), String.t()) :: Native.output(t())
  def create_with_src(%Device{} = device, src), do: Native.session_create_with_src(device, src)

  method(:device)
  method(:device_name)
  method(:device_opencl_c_version)
  method(:device_profile)
  method(:device_vendor)
  method(:device_version)
  method(:device_driver_version)
  method(:device_address_bits)
  method(:device_global_mem_cacheline_size)
  method(:device_max_clock_frequency)
  method(:device_max_compute_units)
  method(:device_max_constant_args)
  method(:device_max_read_image_args)
  method(:device_max_samplers)
  method(:device_max_work_item_dimensions)
  method(:device_max_write_image_args)
  method(:device_mem_base_addr_align)
  method(:device_min_data_type_align_size)
  method(:device_native_vector_width_char)
  method(:device_native_vector_width_short)
  method(:device_native_vector_width_int)
  method(:device_native_vector_width_long)
  method(:device_native_vector_width_float)
  method(:device_native_vector_width_double)
  method(:device_native_vector_width_half)
  method(:device_partition_max_sub_devices)
  method(:device_preferred_vector_width_char)
  method(:device_preferred_vector_width_short)
  method(:device_preferred_vector_width_int)
  method(:device_preferred_vector_width_long)
  method(:device_preferred_vector_width_float)
  method(:device_preferred_vector_width_double)
  method(:device_preferred_vector_width_half)
  method(:device_vendor_id)
  method(:device_available)
  method(:device_compiler_available)
  method(:device_endian_little)
  method(:device_error_correction_support)
  method(:device_host_unified_memory)
  method(:device_image_support)
  method(:device_linker_available)
  method(:device_preferred_interop_user_sync)
  method(:device_image2d_max_width)
  method(:device_image2d_max_height)
  method(:device_image3d_max_width)
  method(:device_image3d_max_height)
  method(:device_image3d_max_depth)
  method(:device_image_max_buffer_size)
  method(:device_image_max_array_size)
  method(:device_max_parameter_size)
  method(:device_max_work_group_size)
  method(:device_printf_buffer_size)
  method(:device_profiling_timer_resolution)
  method(:device_max_work_item_sizes)


  defimpl Inspect do
    @spec inspect(Session.t(), any) :: String.t()
    def inspect(session, _) do
      {:ok, name} = Session.device_name(session)
      {:ok, vendor} = Session.device_vendor(session)
      "#OpenCL.Session<[name: #{name}, vendor: #{vendor}]>"
    end
  end
end
