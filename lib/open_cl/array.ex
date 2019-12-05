defmodule OpenCL.Array do
  use OpenCL.NativeStruct
  alias OpenCL.Array

  defdelegate new(number_type, numbers), to: Native, as: :array_new

  defdelegate filled_with(number_type, number, count), to: Native, as: :array_new_filled_with

  defdelegate to_list(array), to: Native, as: :array_data

  defdelegate length(array), to: Native, as: :array_length

  defdelegate push(array, number), to: Native, as: :array_push

  def extend(array, %Array{} = other) do
    Native.array_extend_from_array(array, other)
  end

  def extend(array, numbers) when is_list(numbers) do
    Native.array_extend(array, numbers)
  end

  defdelegate type(array), to: Native, as: :array_number_type

  defdelegate type_cast(array, number_type), to: Native, as: :array_cast
end
