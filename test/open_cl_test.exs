defmodule OpenCLTest do
  use ExUnit.Case, async: true
  doctest OpenCL

  require Logger

  alias OpenCL.Session
  alias OpenCL.Array
  alias OpenCL.Platform
  alias OpenCL.DeviceBuffer
  alias OpenCL.Device

  def now_in_microsec do
    DateTime.utc_now()
    |> DateTime.to_unix(:microsecond)
  end

  setup [:init_sessions]

  test "all together now", %{sessions: sessions} do
    for session <- sessions do
      assert %Array{} = array = Array.new(:u8, [1, 1, 1])
      assert %DeviceBuffer{} = buffer = DeviceBuffer.build(session, 3, :u8, array, :read_write)

      kernel_name = "add_one_u8"
      work_dims = 3
      :ok = Session.kernel_execute_sync(session, kernel_name, work_dims, [buffer])
      assert %Array{} = array = DeviceBuffer.to_array(buffer)
      assert Array.to_list(array) == [2, 2, 2]
    end
  end

  test "works on 500k", %{sessions: sessions} do
    for session <- sessions do
      count = 500_000
      assert %Array{} = array = Array.filled_with(:u8, 0, count)
      assert %DeviceBuffer{} = buffer = DeviceBuffer.build(session, count, :u8, array, :read_write)
      kernel_name = "add_one_u8"
      work_dims = 500_000
      start_time = now_in_microsec()
      :ok = Session.kernel_execute_sync(session, kernel_name, work_dims, [buffer])
      stop_time = now_in_microsec()

      took_microsec = stop_time - start_time
      Logger.debug("#{inspect(session)} - 500k u8 add_one took #{took_microsec}usec")
      assert %Array{} = array = DeviceBuffer.to_array(buffer)
      ones = Array.to_list(array)
      Enum.each(ones, fn one ->
        assert one == 1
      end)
      assert length(ones) == 500_000
    end
  end

  test "works on 500k repeatedly", %{sessions: sessions} do
    for session <- sessions do
      count = 500_000
      assert %Array{} = array = Array.filled_with(:u8, 0, count)
      assert %DeviceBuffer{} = buffer = DeviceBuffer.build(session, count, :u8, array, :read_write)
      kernel_name = "add_one_u8"
      work_dims = 500_000
      start_time = now_in_microsec()
      Enum.each(1..200, fn _ ->
        :ok = Session.kernel_execute_sync(session, kernel_name, work_dims, [buffer])
      end)
      stop_time = now_in_microsec()
      took_microsec = stop_time - start_time
      Logger.debug("#{inspect(session)} - 500k u8 add_one x200 took #{took_microsec}usec (#{trunc(Float.round(took_microsec / 200))} per call) (#{Float.round(1_000_000 / (took_microsec / 200))} calls per sec)")
      %Array{} = array2 = DeviceBuffer.to_array(buffer)
      nums = Array.to_list(array2)
      Enum.each(nums, fn num ->
        assert num == 200
      end)
      assert length(nums) == 500_000
    end
  end

  test "works on 3x3 matrix", %{sessions: sessions} do
    for session <- sessions do
      count = 3 * 3
      assert %Array{} = array = Array.filled_with(:u8, 0, count)
      assert %DeviceBuffer{} = buffer = DeviceBuffer.build(session, count, :u8, array, :read_write)
      kernel_name = "add_one_u8"
      work_dims = {3, 3}
      start_time = now_in_microsec()
      Enum.each(1..200, fn _ ->
        :ok = Session.kernel_execute_sync(session, kernel_name, work_dims, [buffer])
      end)
      stop_time = now_in_microsec()
      took_microsec = stop_time - start_time
      Logger.debug("#{inspect(session)} - 3x3 u8 add_one x200 took #{took_microsec}usec (#{trunc(Float.round(took_microsec / 200))} per call) (#{Float.round(1_000_000 / (took_microsec / 200))} calls per sec)")
      %Array{} = array2 = DeviceBuffer.to_array(buffer)
      nums = Array.to_list(array2)
      assert nums == [200, 200, 200, 0, 0, 0, 0, 0, 0]
      assert length(nums) == 9
    end
  end

  test "parallelism works", %{sessions: sessions} do
    for session <- sessions do
      count = 500
      array = Array.filled_with(:u8, 0, count)
      assert %Array{} = array
      assert buffer = DeviceBuffer.build(session, count, :u8, array, :read_write)
      assert %DeviceBuffer{} = buffer
      kernel_name = "add_one_u8"
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
      Logger.debug("#{inspect(session)} - 500 u8 add_one x100p100 took #{took_microsec}usec (#{trunc(Float.round(took_microsec / 200))} per call) (#{Float.round(1_000_000 / (took_microsec / 200))} calls per sec)")
      %Array{} = array2 = DeviceBuffer.to_array(buffer)
      nums = Array.to_list(array2)
      Enum.each(nums, fn num ->
        assert num == expected_value
      end)
      assert length(nums) == count
    end
  end

  @src_add_one_u8 """
  __kernel void add_one_u8(__global uchar *nums) {
    int index = get_global_id(0);
    nums[index] += 1;
  }
  """

  def init_sessions(ctx) do
    assert {:ok, platforms} = Platform.list_all()
    assert [_ | _] = devices = Enum.flat_map(platforms, fn p ->
      assert {:ok, devices} = Platform.all_devices(p)
      Enum.filter(devices, &Device.usable?/1)
    end)

    Logger.debug("Running tests with #{inspect(devices, pretty: true)}")

    src = Map.get(ctx, :src, @src_add_one_u8)

    assert [_ | _] = sessions = Enum.map(devices, fn d ->
      assert {:ok, session} = Session.create_with_src(d, src)
      session
    end)

    {:ok, sessions: sessions}
  end
end
