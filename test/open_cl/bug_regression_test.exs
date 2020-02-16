defmodule OpenCL.BugRegressionTest do
  use ExUnit.Case, async: true
  use OpenCL.SessionsCase
  import OpenCL.Test.Repeat

  alias OpenCL.Session
  alias OpenCL.Array
  alias OpenCL.Platform
  alias OpenCL.Buffer
  alias OpenCL.Device

  @src_thing """
  __kernel void thing(uchar a) {
    // noop
  }
  """

  @src_add_one_u8 """
  __kernel void add_one_u8(__global uchar *nums) {
    nums[get_global_id(0)] += 1;
  }
  """

  test "12_JAN_2020 - A buffer that IS used in a kernel is thread-safe", %{sessions: sessions} do
    for session <- sessions do
      count = 500
      array = Array.filled_with(:u8, 0, count)
      assert {:ok, buffer} = Session.create_buffer(session, :u8, array)

      with_concurrency 10 do
        assert {:ok, session} = Session.create_copy(session)
        n_times 200 do
          :ok = Session.execute_kernel(session, "add_one_u8", count, [buffer])
          {:ok, array2} = Session.read_buffer(session, buffer)
          Array.to_list(array2)
        end
      end
    end
    :timer.sleep(5000)
  end

  test "6_JAN_2020 - A buffer that is not used in a kernel is thread-safe", %{sessions: sessions} do
    for session <- sessions do
      count = 500
      array = Array.filled_with(:u8, 0, count)
      assert %Array{} = array
      assert {:ok, buffer} = Session.create_buffer(session, :u8, array)

      with_concurrency 100 do
        n_times 100 do
          _rc = Buffer.length(buffer)
        end
      end
    end
  end


  test "6_JAN_2020 - A calling kernels is concurrency-safe", %{sessions: sessions} do
    for session <- sessions do
      with_concurrency 10 do
        assert {:ok, session} = Session.create_copy(session)
        n_times 10 do
          arr = Array.new(:u8, [1])
          assert {:ok, buffer} = Session.create_buffer(session, :u8, arr)
          assert :ok = Session.execute_kernel(session, "add_one_u8", 1, [buffer])
        end
      end
    end
  end
end
