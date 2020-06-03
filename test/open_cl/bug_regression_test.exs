defmodule OpenCL.BugRegressionTest do
  use ExUnit.Case, async: true
  use OpenCL.SessionsCase
  import OpenCL.Test.Repeat

  alias OpenCL.Session
  alias OpenCL.Array
  alias OpenCL.Buffer

  test "12_JAN_2020 - A buffer that IS used in a kernel is thread-safe", %{sessions: sessions} do
    for session <- sessions do
      count = 500
      assert {:ok, array} = Array.filled_with({:char, 0}, count)

      with_concurrency 10 do
        n_times 200 do
          assert {:ok, buffer} = Session.create_buffer_from_data(session, array)
          :ok = Session.execute_kernel(session, "add_num_uchar", count, [buffer, {:uchar, 1}])
          {:ok, array2} = Session.read_buffer(session, buffer)
          Array.to_list(array2)
        end
      end
    end
  end

  test "6_JAN_2020 - A buffer that is not used in a kernel is thread-safe", %{sessions: sessions} do
    for session <- sessions do
      count = 500
      assert {:ok, array} = Array.filled_with({:char, 0}, count)
      assert %Array{} = array
      assert {:ok, buffer} = Session.create_buffer_from_data(session, array)

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
        n_times 10 do
          assert {:ok, arr} = Array.new({:char, [1]})
          assert {:ok, buffer} = Session.create_buffer_from_data(session, arr)
          assert :ok = Session.execute_kernel(session, "add_num_uchar", 1, [buffer, {:uchar, 1}])
        end
      end
    end
  end
end
