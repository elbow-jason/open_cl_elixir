defmodule OpenCL.SessionTest do
  use ExUnit.Case
  use OpenCL.SessionsCase

  alias OpenCL.Session
  alias OpenCL.Array
  alias OpenCL.Number

  use OpenCL.T

  test "session concurrency", %{sessions: sessions} do
    assert kernel_name = "add_num_int"
    1..10
    |> Enum.map(fn _ ->
      Task.async(fn ->

        work_dims = 1000
        Enum.map(T.number_types(), fn number_type ->
          zero = Number.zero(number_type)
          assert zero in [0, 0.0]
          assert {:ok, array} = Array.filled_with({number_type, zero}, work_dims)
          Enum.each(sessions, fn session ->
            assert {:ok, buffer} = Session.create_buffer(session, array)

            Enum.each(1..10, fn raw_num ->
              assert {:ok, casted_num} = OpenCL.Number.cast(number_type, raw_num)
              :ok = Session.execute_kernel(session, kernel_name, work_dims, [buffer, {number_type, casted_num}])
            end)
          end)
        end)
      end)
    end)
    |> Enum.map(fn task -> Task.await(task) end)
  end
end
