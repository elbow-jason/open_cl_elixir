defmodule OpenCL.SessionTest do
  use ExUnit.Case
  use OpenCL.SessionsCase

  alias OpenCL.Session
  alias OpenCL.Array

  test "session concurrency", %{sessions: sessions} do
    assert kernel_name = "add_one_int"
    1..10
    |> Enum.map(fn _ ->
      Task.async(fn ->
        work_dims = 1000
        array = Array.filled_with(:char, 0, work_dims)

        Enum.each(sessions, fn session ->
          assert {:ok, buffer} = Session.create_buffer(session, array)

          Enum.each(1..10, fn _ ->
            :ok = Session.execute_kernel(session, kernel_name, work_dims, [buffer])
          end)
        end)
      end)
    end)
    |> Enum.map(fn task -> Task.await(task) end)
  end
end
