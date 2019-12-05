defmodule OpenCL do
  @moduledoc """
  Documentation for OpenCL.
  """

  @spec add_one_src :: binary()
  def add_one_src do
    """
    __kernel void test(__global int *i) {
      *i += 1;
    }
    """
  end
end
