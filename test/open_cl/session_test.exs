# defmodule OpenCL.SessionTest do
#   use ExUnit.Case
#   use OpenCL.SessionsCase

#   alias OpenCL.Session

#   @add_one_src """
#   __kernel void test(__global int *i) {
#     *i += 1;
#   }
#   """

#   test "src" do
#     assert is_binary(@add_one_src) == true
#   end

#   test "session concurrency", %{sessions: sessions} do
#      1..1000
#       |> Enum.map(fn _ ->
#         Task.async(fn ->
#           Enum.each(sessions, fn sess ->
#             Enum.each(1..100, fn _ ->
#               Session.device_name(sess)
#               # :ok = Session.kernel_execute_sync(session, kernel_name, work_dims, [buffer])
#             end)
#           end)
#         end)
#       end)
#       |> Enum.map(fn task -> Task.await(task) end)
#   end

# end
