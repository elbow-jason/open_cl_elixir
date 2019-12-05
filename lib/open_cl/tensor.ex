defmodule OpenCL.Tensor do
  use OpenCL.NativeStruct
  alias OpenCL.Tensor

  @spec new(Native.dims(), [float]) :: Tensor.t()
  def new(dims, data), do: Native.tensor_new(dims, data)

  # @spec dims(t()) :: Native.dims()
  # def dims(tensor), do: Native.tensor_self_dims(tensor)

  # @spec append(Tensor.t(), Tensor.t() | [float]) :: Tensor.t()
  # def append(%Tensor{} = left, %Tensor{} = right) do
  #   Native.tensor_self_extend_tensor(left, right)
  # end

  # def append(%Tensor{} = tensor, list_of_floats) when is_list(list_of_floats) do
  #   Native.tensor_self_extend_vec(tensor, list_of_floats)
  # end

  # defimpl Inspect do
  #   @spec inspect(Tensor.t(), any) :: String.t()
  #   def inspect(tensor, _) do
  #     dims = Tensor.dims(tensor)

  #     "#OpenCL.Tensor<[type: f32, dims: #{inspect(dims)}]>"
  #   end
  # end
end

# alias OpenCL.Platform


