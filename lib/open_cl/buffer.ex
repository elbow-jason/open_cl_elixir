defmodule OpenCL.Buffer do
  use OpenCL.NativeStruct

  method(:length)
  method(:number_type)
  method(:mem_config)
  method(:reference_count)
  method(:available_devices)

  defimpl Inspect do
    alias OpenCL.Buffer
    def inspect(buffer, _) do
      nt = Buffer.number_type(buffer)
      len = Buffer.length(buffer)
      mem_cfg = inspect(Buffer.mem_config(buffer))
      devices = inspect(Buffer.available_devices(buffer))
      "#OpenCL.Buffer<[number_type: #{nt}, length: #{len}, mem_config: #{mem_cfg}, available_devices: #{devices}]>"
    end
  end
end
