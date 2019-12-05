defmodule OpenCL.SessionTest do
  use ExUnit.Case
  @add_one_src """
    __kernel void test(__global int *i) {
      *i += 1;
    }
    """

  test "src" do
    assert is_binary(@add_one_src) == true
  end
end
