defmodule OpenCL.Array do
  use OpenCL.NativeStruct
  alias OpenCL.Array
  alias OpenCL.Native
  alias OpenCL.Number

  @spec new(Native.number_typed_list()) :: {:ok, t()} | {:error, any}
  def new(number_typed_list) do
    case Native.array_new(number_typed_list) do
      :invalid_variant ->
        # if this is not an error we've got a bug
        {:error, errors} = Number.check(number_typed_list)
        {:error, errors}
      %Array{} = array ->
        {:ok, array}
    end
  end

  def filled_with(typed_number, count) do
    case Native.array_new_filled_with(typed_number, count) do
      :invalid_variant ->
        # if this is not an error we've got a bug
        {:error, error} = Number.check(typed_number)
        {:error, error}
      %Array{} = array -> {:ok, array}
    end
  end

  defdelegate to_list(array), to: Native, as: :array_data

  defdelegate length(array), to: Native, as: :array_length

  def push(array, number) do
    case Native.array_push(array, number) do
      {} -> array
      err -> err
    end
  end
  def extend(array, %Array{} = other) do
    case Native.array_extend_from_array(array, other) do
      {:ok, {}} -> :ok
      {} -> :ok
      err -> err
    end
  end

  def extend(array, numbers) when is_list(numbers) do
    case Native.array_extend_from_list(array, numbers) do
      {:ok, {}} -> :ok
      {} -> :ok
      err -> err
    end
  end

  defdelegate type(array), to: Native, as: :array_number_type

  defdelegate type_cast(array, number_type), to: Native, as: :array_cast

  defimpl Inspect do
    alias OpenCL.Array
    def inspect(arr, _) do
      "#OpenCL.Array<[type: #{Array.type(arr)}, len: #{Array.length(arr)}]>"
    end
  end
end
