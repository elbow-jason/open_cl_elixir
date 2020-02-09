defmodule OpenCL.Device do
  use OpenCL.NativeStruct
  alias OpenCL.Device

  @spec usable?(t()) :: boolean
  defdelegate usable?(device), to: Native, as: :device_self_is_usable

  # strings
  method(:name)
  method(:opencl_c_version)
  method(:profile)
  method(:vendor)
  method(:version)
  method(:driver_version)

  # booleans
  method(:available)
  method(:compiler_available)
  method(:endian_little)
  method(:error_correction_support)
  method(:host_unified_memory)
  method(:image_support)
  method(:linker_available)
  method(:preferred_interop_user_sync)

  # u32
  method(:address_bits)
  method(:global_mem_cacheline_size)
  method(:max_clock_frequency)
  method(:max_compute_units)
  method(:max_constant_args)
  method(:max_read_image_args)
  method(:max_samplers)
  method(:max_work_item_dimensions)
  method(:max_write_image_args)
  method(:mem_base_addr_align)
  method(:min_data_type_align_size)
  method(:native_vector_width_char)
  method(:native_vector_width_short)
  method(:native_vector_width_int)
  method(:native_vector_width_long)
  method(:native_vector_width_float)
  method(:native_vector_width_double)
  method(:native_vector_width_half)
  method(:partition_max_sub_devices)
  method(:preferred_vector_width_char)
  method(:preferred_vector_width_short)
  method(:preferred_vector_width_int)
  method(:preferred_vector_width_long)
  method(:preferred_vector_width_float)
  method(:preferred_vector_width_double)
  method(:preferred_vector_width_half)
  method(:vendor_id)
  # list of sizes
  method(:max_work_item_sizes)

  # # flags
  method(:partition_affinity_domain)

  # @spec device_type(t()) :: integer()
  # def device_type(device), do: Native.device_self_device_type(device)

  # @spec global_mem_cache_type(t()) :: integer()
  # def global_mem_cache_type(device), do: Native.device_self_global_mem_cache_type(device)

  # @spec local_mem_type(t()) :: integer()
  # def local_mem_type(device), do: Native.device_self_local_mem_type(device)

  # # others
  # def platform(device), do: Native.device_self_platform(device)

  defimpl Inspect do
    @spec inspect(Device.t(), any) :: String.t()
    def inspect(device, _) do
      if Device.usable?(device) do
        {:ok, name} = Device.name(device)
        "#OpenCL.Device<[#{name}]>"
      else
        "#OpenCL.Device<[device is unusable]>"
      end
    end
  end
end
