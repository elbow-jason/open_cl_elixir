defmodule OpenCL.SessionsCase do
  use ExUnit.CaseTemplate

  alias OpenCL.Platform
  alias OpenCL.Session
  alias OpenCL.SourceHelpers

  setup do
    platforms = Platform.list_all()
    assert length(platforms) > 0
    devices =
      Enum.flat_map(platforms, fn p ->
        Platform.list_all_devices(p)
      end)
    assert length(devices) > 0

    src = SourceHelpers.full()
    {:ok, sessions} = Session.create(src)
    sessions = Enum.take(sessions, 1)
    {:ok, sessions: sessions}
  end
end
