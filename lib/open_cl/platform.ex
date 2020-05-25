defmodule OpenCL.Platform do
  use OpenCL.NativeStruct
  alias OpenCL.Platform

  func_0(:default)
  func_0(:list_all)

  method(:name)
  method(:version)
  method(:profile)
  method(:vendor)
  method(:extensions)

  defdelegate list_all_devices(platform), to: Native, as: :platform_list_all_devices
  defdelegate list_default_devices(platform), to: Native, as: :platform_list_default_devices
  defdelegate list_cpu_devices(platform), to: Native, as: :platform_list_cpu_devices
  defdelegate list_gpu_devices(platform), to: Native, as: :platform_list_gpu_devices

  defdelegate list_accelerator_devices(platform),
    to: Native,
    as: :platform_list_accelerator_devices

  defdelegate list_custom_devices(platform), to: Native, as: :platform_list_custom_devices

  defimpl Inspect do
    @spec inspect(Platform.t(), any) :: String.t()
    def inspect(platform, _) do
      name = Platform.name(platform)
      version = Platform.version(platform)
      "#OpenCL.Platform<[name: #{name}, version: #{version}]>"
    end
  end
end

# alias OpenCL.Platform
