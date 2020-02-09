defmodule OpenCL.Buffer do
  use OpenCL.NativeStruct
  alias OpenCL.Array
  alias OpenCL.Session

  def build(%Session{} = session, number_type, len_or_data, opts \\ []) do
    Session.create_buffer(session, number_type, len_or_data, opts)
  end

  # defdelegate to_array(buf), to: Native, as: :buffer_to_array
  # defdelegate reference_count(buf), to: Native, as: :buffer_reference_count
end

