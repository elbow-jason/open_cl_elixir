defmodule OpenCLTest do
  use ExUnit.Case, async: true
  use OpenCL.SessionsCase

  doctest OpenCL

  require Logger

  alias OpenCL.Session
  alias OpenCL.Array
  alias OpenCL.SourceHelpers
  alias OpenCL.ProfilingHelpers

  setup do
    src = SourceHelpers.full()
    {:ok, sessions} = Session.create(src)
    sessions = Enum.take(sessions, 1)
    {:ok, sessions: sessions}
  end

  test "all together now", %{sessions: sessions} do
    for session <- sessions do
      assert {:ok, array} = Array.new({:uchar, [1, 1, 1]})
      assert {:ok, buffer} = Session.create_buffer_from_data(session, array)
      work_dims = 3
      :ok = Session.execute_kernel(session, "add_num_uchar", work_dims, [buffer, {:uchar, 1}])
      assert {:ok, array} = Session.read_buffer(session, buffer)
      assert Array.to_list(array) == [2, 2, 2]
    end
  end

  test "works on 500k", %{sessions: sessions} do
    for session <- sessions do
      count = 500_000
      assert {:ok, %Array{} = array} = Array.filled_with({:uchar, 0}, count)

      assert {:ok, buffer} = Session.create_buffer_from_data(session, array)
      name = "add_num_uchar with 500k items"

      ProfilingHelpers.run(session, name, 1, 1, fn sess ->
        assert :ok = Session.execute_kernel(sess, "add_num_uchar", count, [buffer, {:uchar, 1}])
      end)

      assert {:ok, array} = Session.read_buffer(session, buffer)
      ones = Array.to_list(array)

      assert length(ones) == 500_000
      assert_all_equal(ones, 1)
    end
  end

  test "works on 500k repeatedly", %{sessions: sessions} do
    for session <- sessions do
      count = 500_000
      assert {:ok, %Array{} = array} = Array.filled_with({:uchar, 0}, count)

      assert {:ok, buffer} = Session.create_buffer_from_data(session, array)
      name = "add_num_uchar with 500k items"

      ProfilingHelpers.run(session, name, 200, 1, fn sess ->
        assert :ok = Session.execute_kernel(sess, "add_num_uchar", count, [buffer, {:uchar, 1}])
      end)

      assert {:ok, array2} = Session.read_buffer(session, buffer)
      nums = Array.to_list(array2)

      assert length(nums) == 500_000
      assert_all_equal(nums, 200)
    end
  end

  @tag skip: true
  test "works on 3x3 matrix", %{sessions: sessions} do
    # This test needs a kernel that works on 3 dims
    for session <- sessions do
      count = 3 * 3
      assert {:ok, array} = Array.filled_with({:char, 0}, count)

      assert {:ok, buffer} = Session.create_buffer_from_data(session, array)

      work_dims = {3, 3}

      name = "add_num_uchar with 3 x 3 work"

      ProfilingHelpers.run(session, name, 200, 1, fn sess ->
        assert :ok = Session.execute_kernel(sess, "add_num_uchar", work_dims, [buffer, {:uchar, 1}])
      end)

      assert {:ok, array2} = Session.read_buffer(session, buffer)
      nums = Array.to_list(array2)
      # add_one_uchar only uses get_global_id(0) which means
      # that only the first dimension is worked on
      assert nums == [200, 200, 200, 0, 0, 0, 0, 0, 0]
      assert length(nums) == 9
    end
  end

  test "parallelism works", %{sessions: sessions} do
    for session <- sessions do
      count = 500
      assert {:ok, array} = Array.filled_with({:char, 0}, count)
      assert {:ok, buffer} = Session.create_buffer_from_data(session, array)
      work_dims = 500

      name = "add_num_uchar with 500 items"

      ProfilingHelpers.run(session, name, 100, 100, fn sess ->
        assert :ok = Session.execute_kernel(sess, "add_num_uchar", work_dims, [buffer, {:uchar, 1}])
      end)

      total_runs = 100 * 100
      expected_value = rem(total_runs, 256)

      assert {:ok, array2} = Session.read_buffer(session, buffer)
      nums = Array.to_list(array2)

      Enum.each(nums, fn num ->
        assert num == expected_value
      end)

      assert length(nums) == count
      assert_all_equal(nums, expected_value)
    end
  end

  # def now_in_microsec do
  #   DateTime.utc_now()
  #   |> DateTime.to_unix(:microsecond)
  # end

  def assert_all_equal(nums, expected) do
    nums
    |> Enum.with_index()
    |> Enum.each(fn {num, i} ->
      assert num == expected, "Number at index #{i} was not #{expected} got #{num}"
    end)
  end
end
