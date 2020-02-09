# defmodule OpenCL.Test.Repeat do

#   defmacro n_times(n, do: block) do
#     quote do
#       for _ <- 1..unquote(n) do
#         unquote(block)
#       end
#     end
#   end

#   defmacro with_concurrency(n, do: block) do
#     quote do
#       Enum.map(1..unquote(n), fn _ ->
#         Task.async(fn ->
#           unquote(block)
#         end)
#       end)
#       |> Enum.map(fn task -> Task.await(task) end)
#     end

#   end
# end
