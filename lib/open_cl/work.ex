defmodule OpenCL.Work do
  defguardp nn_int(x) when is_integer(x) and x >= 0

  def validate(work) do
    case work do
      x when nn_int(x) -> {:ok, x}
      {x} when nn_int(x) -> {:ok, {x}}
      {x, y} when nn_int(x) and nn_int(y) -> {:ok, {x, y}}
      {x, y, z} when nn_int(x) and nn_int(y) and nn_int(z) -> {:ok, {x, y, z}}
      {:error, [work: "is invalid"]}
    end
  end
end
