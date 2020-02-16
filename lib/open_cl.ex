defmodule OpenCL do
  @moduledoc """
  Documentation for OpenCL.
  """

  @spec add_one_u8_src :: binary()
  def add_one_u8_src do
    """
    __kernel void add_one_u8(__global uchar *data) {
        data[get_global_id(0)] += 1;
    }
    """
  end

  # def create_sessions(src)
end
