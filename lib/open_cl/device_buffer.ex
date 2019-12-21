defmodule OpenCL.DeviceBuffer do
  use OpenCL.NativeStruct
  alias OpenCL.Array
  alias OpenCL.Session

  def build(%Session{} = session, dims, number_type, %Array{} = array, kernel_access) do
    Native.buffer_build_from_array(session, dims, number_type, array, kernel_access)
  end

  defdelegate to_array(buf), to: Native, as: :buffer_to_array
end
