defmodule OpenCL.SessionsCase do
  use ExUnit.CaseTemplate

  alias OpenCL.Platform
  alias OpenCL.Device
  alias OpenCL.Session

  @src_add_one_u8 """
  __kernel void add_one_u8(__global uchar *nums) {
    int index = get_global_id(0);
    nums[index] += 1;
  }
  """

  setup ctx do
    {:ok, platforms} = Platform.list_all()

    [_ | _] =
      Enum.flat_map(platforms, fn p ->
        {:ok, devices} = Platform.list_all_devices(p)
        Enum.filter(devices, &Device.usable?/1)
      end)

    # Logger.debug("Running tests with #{inspect(devices, pretty: true)}")

    src = Map.get(ctx, :src, @src_add_one_u8)
    kernel_name = Map.get(ctx, :kernel_name, "add_one_u8")
    session = Session.create(src)
    {:ok, session: session, kernel_name: kernel_name}
  end
end
