defmodule OpenCL.Platform do
  use OpenCL.NativeStruct
  alias OpenCL.Platform

  func_0(:default)
  func_0(:count)
  func_0(:list_all)

  method(:all_devices)
  method(:cpu_devices)
  method(:gpu_devices)
  method(:accelerator_devices)
  method(:custom_devices)
  method(:name)
  method(:version)
  method(:profile)
  method(:vendor)
  method(:extensions)

  defimpl Inspect do
    @spec inspect(Platform.t(), any) :: String.t()
    def inspect(platform, _) do
      {:ok, name} = Platform.name(platform)
      {:ok, version} = Platform.version(platform)
      "#OpenCL.Platform<[name: #{name}, version: #{version}]>"
    end
  end
end

# alias OpenCL.Platform


