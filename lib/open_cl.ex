defmodule OpenCL do
  @moduledoc """
  Documentation for OpenCL.
  """

  @spec add_one_src :: binary()
  def add_one_src do
    """
    __kernel void add_one(__global int *data) {
        data[get_global_id(0)] += 1;
    }
    """
  end
end
