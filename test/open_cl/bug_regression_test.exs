# defmodule OpenCL.BugRegressionTest do
#   use ExUnit.Case, async: true
#   use OpenCL.SessionsCase
#   import OpenCL.Test.Repeat

#   alias OpenCL.Session
#   alias OpenCL.Array
#   alias OpenCL.Platform
#   alias OpenCL.DeviceBuffer
#   alias OpenCL.Device

#   @src_thing """
#   __kernel void thing(uchar a) {
#     // noop
#   }
#   """

#   @src_add_one_u8 """
#   __kernel void add_one_u8(__global uchar *i) {
#     *i += 1;
#   }
#   """

#   @tag [src: @src_add_one_u8, kernel_name: "add_one_u8"]
#   test "12_JAN_2020 - A buffer that IS used in a kernel is thread-safe", %{sessions: sessions, kernel_name: kernel_name} do
#     assert kernel_name == "add_one_u8"

#     for session <- sessions do
#       count = 500
#       array = Array.filled_with(:u8, 0, count)
#       assert %Array{} = array
#       assert %DeviceBuffer{} = buffer = DeviceBuffer.build(session, count, :u8, array, :read_write)

#       with_concurrency 1 do
#         n_times 200 do
#           :ok = Session.kernel_execute_sync(session, kernel_name, count, [buffer])
#           _result = buffer |> DeviceBuffer.to_array() |> Array.to_list()
#         end
#       end
#     end
#     :timer.sleep(5000)
#   end

#   test "6_JAN_2020 - A buffer that is not used in a kernel is thread-safe", %{sessions: sessions} do
#     for session <- sessions do
#       count = 500
#       array = Array.filled_with(:u8, 0, count)
#       assert %Array{} = array
#       assert %DeviceBuffer{} = buffer = DeviceBuffer.build(session, count, :u8, array, :read_write)

#       with_concurrency 100 do
#         n_times 100 do
#           _rc = DeviceBuffer.reference_count(buffer)
#         end
#       end
#     end
#   end

#   @tag [src: @src_thing, kernel_name: "thing"]
#   test "6_JAN_2020 - A calling kernels is concurrency-safe", %{sessions: sessions, kernel_name: kernel_name} do
#     assert kernel_name == "thing"
#     for session <- sessions do
#       with_concurrency 1 do
#         n_times 10 do
#           :ok = Session.kernel_execute_sync(session, kernel_name, 1, [1])
#         end
#       end
#     end
#   end
# end
