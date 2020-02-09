# defmodule OpenCL.Test.ArrayHelpers do
#   alias OpenCL.Array

#   defmacro __using__(_) do
#     quote do
#       require OpenCL.Test.ArrayHelpers
#       alias OpenCL.Test.ArrayHelpers
#       alias OpenCL.Array
#     end
#   end

#   defmacro test_type_cast(from_type, to_type) do
#     quote bind_quoted: [from_type: from_type, to_type: to_type] do
#       alias OpenCL.Array
#       alias OpenCL.Test.ArrayHelpers

#       test "can convert from #{inspect(from_type)} to #{inspect(to_type)}" do
#         from_type = unquote(from_type)
#         to_type = unquote(to_type)
#         data = ArrayHelpers.convert(from_type, [1, 2, 3])
#         expected = ArrayHelpers.convert(to_type, [1, 2, 3])

#         array = Array.new(from_type, data)
#         casted_array = Array.type_cast(array, to_type)
#         assert Array.type(casted_array) == to_type
#         assert Array.to_list(casted_array) == expected
#       end
#     end
#   end

#   defmacro test_push(number_type, data, num) do
#     quote bind_quoted: [number_type: number_type, data: data, num: num] do
#       alias OpenCL.Array
#       alias OpenCL.Test.ArrayHelpers

#       test "pushes items to the end of the Array for type #{number_type}" do
#         number_type = unquote(number_type)
#         data = unquote(data)
#         num = unquote(num)

#         array = Array.new(number_type, data)
#         assert :ok = Array.push(array, num)
#         assert Array.to_list(array) == ArrayHelpers.convert(number_type, data ++ [num])
#       end
#     end
#   end

#   defmacro test_number_type(number_type) do
#     quote bind_quoted: [number_type: number_type] do
#       alias OpenCL.Array

#       test "returns the number type atom #{inspect(number_type)} for #{number_type} Arrays" do
#         number_type = unquote(number_type)
#         array = Array.new(number_type, [1, 2, 3])
#         assert Array.type(array) == number_type
#       end
#     end
#   end

#   defmacro test_length(number_type, data) do
#     quote bind_quoted: [number_type: number_type, data: data] do
#       alias OpenCL.Array

#       test "returns the length of an Array for type #{number_type}" do
#         content = unquote(data)
#         array = Array.new(unquote(number_type), content)
#         assert length(content) == Array.length(array)
#       end
#     end
#   end

#   defmacro test_to_list(number_type, data) do
#     quote bind_quoted: [number_type: number_type, data: data] do
#       alias OpenCL.Array

#       test "can turn an Array to a list for type #{number_type}" do
#         array = Array.new(unquote(number_type), unquote(data))
#         assert Array.to_list(array) == unquote(data)
#       end
#     end
#   end

#   defmacro test_filled_with(number_type, filler, count) do
#     quote bind_quoted: [number_type: number_type, filler: filler, count: count] do
#       alias OpenCL.Array

#       test "can make a new array full of #{number_type}" do
#         count = unquote(count)
#         filler = unquote(filler)
#         array = Array.filled_with(unquote(number_type), filler, count)
#         content = Array.to_list(array)
#         assert Array.length(array) == count
#         assert length(content) == count
#         Enum.each(content, fn item -> assert item == filler end)
#       end
#     end
#   end

#   defmacro test_new(number_type, data) do
#     quote bind_quoted: [number_type: number_type, data: data] do
#       alias OpenCL.Array

#       test "can make a new Array of type #{number_type}" do
#         array = Array.new(unquote(number_type), unquote(data))
#         assert Array.to_list(array) == unquote(data)
#       end
#     end
#   end

#   defmacro test_extend(number_type, data, other) do
#     quote bind_quoted: [number_type: number_type, data: data, other: other] do
#       alias OpenCL.Array
#       alias OpenCL.Test.ArrayHelpers

#       test "works for #{number_type} with list of number" do
#         number_type = unquote(number_type)
#         data = unquote(data)
#         other = unquote(other)

#         array = Array.new(number_type, data)
#         assert :ok = Array.extend(array, other)
#         assert Array.to_list(array) == data ++ ArrayHelpers.to_list(other)
#       end

#       test "works for #{number_type} with array" do
#         number_type = unquote(number_type)
#         data = unquote(data)
#         other = unquote(other)

#         array = Array.new(number_type, data)
#         other_array = ArrayHelpers.to_array(number_type, other)
#         assert :ok = Array.extend(array, other_array)
#         assert Array.to_list(array) == data ++ ArrayHelpers.to_list(other_array)
#       end

#       test "with #{number_type} does not mutate the extender array" do
#         number_type = unquote(number_type)
#         data = unquote(data)
#         other = unquote(other)

#         array = Array.new(number_type, data)
#         other_array = ArrayHelpers.to_array(number_type, other)
#         assert :ok = Array.extend(array, other_array)
#         # array is unchanged
#         assert Array.to_list(array) == data ++ other
#       end

#       test "an array of type #{number_type} can extend itself" do
#         number_type = unquote(number_type)
#         data = unquote(data)

#         array = Array.new(number_type, data)
#         assert :ok = Array.extend(array, array)
#         assert Array.to_list(array) == data ++ data
#       end
#     end
#   end

#   def to_list(%Array{} = array), do: Array.to_list(array)
#   def to_list(data) when is_list(data), do: data

#   def to_array(number_type, data) when is_list(data), do: Array.new(number_type, data)
#   def to_array(number_type, %Array{} = array), do: Array.type_cast(number_type, array)

#   @float_types [:f32, :f64]
#   defguard is_float_t(x) when x in @float_types

#   def convert(t, data) when is_list(data) do
#     Enum.map(data, fn item -> convert(t, item) end)
#   end

#   def convert(t, data) when is_float_t(t) and is_float(data), do: data
#   def convert(t, data) when is_float_t(t) and is_integer(data), do: data * 1.0
#   def convert(t, data) when not is_float_t(t) and is_integer(data), do: data
#   def convert(t, data) when not is_float_t(t) and is_float(data), do: Float.round(data)
# end
