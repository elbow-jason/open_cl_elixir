defmodule OpenCL.Dims do
  @type dim_length :: non_neg_integer()

  @type t ::
          dim_length()
          | {dim_length()}
          | {dim_length(), dim_length()}
          | {dim_length(), dim_length(), dim_length()}

  defguard is_dim_length(x) when is_integer(x) and x >= 0

  def is_dims?(x) when is_dim_length(x), do: true
  def is_dims?({x}) when is_dim_length(x), do: true
  def is_dims?({x, y}) when is_dim_length(x) and is_dim_length(y), do: true

  def is_dims?({x, y, z}) when is_dim_length(x) and is_dim_length(y) and is_dim_length(z),
    do: true

  def is_dims?(_), do: false

  def errors(dims) do
    if is_dims?(dims) do
      []
    else
      [dims: "must be a non-negative integer or a 1D, 2D, or 3D tuple"]
    end
  end
end
