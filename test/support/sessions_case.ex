defmodule OpenCL.SessionsCase do
  use ExUnit.CaseTemplate

  alias OpenCL.Platform
  alias OpenCL.Device
  alias OpenCL.Session
  alias OpenCL.SourceHelpers

  # @src_add_one_u8 """
  # __kernel void add_one_u8(__global uchar *nums) {
  #   nums[get_global_id(0)] += 1
  # }
  # """

  setup do
    {:ok, platforms} = Platform.list_all()

    [_ | _] =
      Enum.flat_map(platforms, fn p ->
        {:ok, devices} = Platform.list_all_devices(p)
        Enum.filter(devices, &Device.usable?/1)
      end)
    src = SourceHelpers.full()
    assert {:ok, sessions} = Session.create(src)
    sessions = Enum.take(sessions, 1)
    {:ok, sessions: sessions}
  end
end
