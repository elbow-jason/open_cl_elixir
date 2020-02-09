defmodule OpenCLTest do
  use ExUnit.Case, async: true
  use OpenCL.SessionsCase

  doctest OpenCL

  require Logger

  alias OpenCL.Session
  alias OpenCL.Array
  alias OpenCL.Platform
  alias OpenCL.Buffer
  alias OpenCL.Device

  def now_in_microsec do
    DateTime.utc_now()
    |> DateTime.to_unix(:microsecond)
  end

  @src_add_one_u8 """
  __kernel void add_one_u8(__global uchar *i) {
    *i += 1;
  }
  """

  setup ctx do
    src = Map.get(ctx, :src, @src_add_one_u8)
    {:ok, sess} = Session.create(src)
    {:ok, session: sess}
  end

  test "all together now", %{session: session} do

    assert %Array{} = array = Array.new(:u8, [1, 1, 1])
    assert {:ok, buffer} = Buffer.build(session, :u8, array)

    kernel_name = "add_one_u8"
    work_dims = 3
    :ok = Session.kernel_execute_sync(session, kernel_name, work_dims, [buffer])
    assert %Array{} = array = Buffer.to_array(buffer)
    assert Array.to_list(array) == [2, 2, 2]

  end

  test "works on 500k", %{session: session} do

    count = 500_000
    assert %Array{} = array = Array.filled_with(:u8, 0, count)

    assert {:ok, buffer} = Buffer.build(session, :u8, array)

    kernel_name = "add_one_u8"
    work_dims = 500_000
    start_time = now_in_microsec()
    :ok = Session.kernel_execute_sync(session, kernel_name, work_dims, [buffer])
    stop_time = now_in_microsec()

    took_microsec = stop_time - start_time
    Logger.debug("#{inspect(session)} - 500k u8 add_one took #{took_microsec}usec")
    assert %Array{} = array = Buffer.to_array(buffer)
    ones = Array.to_list(array)

    Enum.each(ones, fn one ->
      assert one == 1
    end)

    assert length(ones) == 500_000

  end

  test "works on 500k repeatedly", %{session: session} do
    count = 500_000
    assert %Array{} = array = Array.filled_with(:u8, 0, count)

    assert {:ok, buffer} = Buffer.build(session, :u8, array)

    kernel_name = "add_one_u8"
    work_dims = 500_000
    start_time = now_in_microsec()

    Enum.each(1..200, fn _ ->
      :ok = Session.kernel_execute_sync(session, kernel_name, work_dims, [buffer])
    end)

    stop_time = now_in_microsec()
    took_microsec = stop_time - start_time

    Logger.debug(
      "#{inspect(session)} - 500k u8 add_one x200 took #{took_microsec}usec (#{
        trunc(Float.round(took_microsec / 200))
      } per call) (#{Float.round(1_000_000 / (took_microsec / 200))} calls per sec)"
    )

    %Array{} = array2 = Buffer.to_array(buffer)
    nums = Array.to_list(array2)

    Enum.each(nums, fn num ->
      assert num == 200
    end)

    assert length(nums) == 500_000
  end

  test "works on 3x3 matrix", %{session: session} do

    count = 3 * 3
    array = Array.filled_with(:u8, 0, count)

    assert {:ok, buffer} = Buffer.build(session, :u8, array)

    kernel_name = "add_one_u8"
    work_dims = {3, 3}
    start_time = now_in_microsec()

    Enum.each(1..200, fn _ ->
      :ok = Session.kernel_execute_sync(session, kernel_name, work_dims, [buffer])
    end)

    stop_time = now_in_microsec()
    took_microsec = stop_time - start_time

    Logger.debug(
      "#{inspect(session)} - 3x3 u8 add_one x200 took #{took_microsec}usec (#{
        trunc(Float.round(took_microsec / 200))
      } per call) (#{Float.round(1_000_000 / (took_microsec / 200))} calls per sec)"
    )

    %Array{} = array2 = Buffer.to_array(buffer)
    nums = Array.to_list(array2)
    assert nums == [200, 200, 200, 0, 0, 0, 0, 0, 0]
    assert length(nums) == 9
  end

  test "parallelism works", %{session: session, kernel_name: kernel_name} do

    count = 500
    array = Array.filled_with(:u8, 0, count)
    assert %Array{} = array
    {:ok, buffer} = Buffer.build(session, :u8, array)
    assert %Buffer{} = buffer
    work_dims = 500
    start_time = now_in_microsec()

    1..100
    |> Enum.map(fn _ ->
      Task.async(fn ->
        Enum.each(1..100, fn _ ->
          :ok = Session.kernel_execute_sync(session, kernel_name, work_dims, [buffer])
        end)
      end)
    end)
    |> Enum.map(fn task -> Task.await(task) end)

    total_runs = 100 * 100
    expected_value = rem(total_runs, 256)

    stop_time = now_in_microsec()
    took_microsec = stop_time - start_time

    Logger.debug(
      "#{inspect(session)} - 500 u8 add_one x100p100 took #{took_microsec}usec (#{
        trunc(Float.round(took_microsec / 200))
      } per call) (#{Float.round(1_000_000 / (took_microsec / 200))} calls per sec)"
    )

    %Array{} = array2 = Buffer.to_array(buffer)
    nums = Array.to_list(array2)

    Enum.each(nums, fn num ->
      assert num == expected_value
    end)

    assert length(nums) == count
  end
end
