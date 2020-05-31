defmodule OpenCL.SourceHelpers do
  use OpenCL.T

  def zero(t) when T.is_int_type(t), do: "0"
  def zero(t) when T.is_float_type(t), do: "0.0"

  def one(t) when T.is_int_type(t), do: "1"
  def one(t) when T.is_float_type(t), do: "1.0"

  def to_arg(name, :buffer, type), do: "__global #{type} *#{name}"

  def buffer_add_one(type) do
    name = "add_num_#{type}"

    src = """
    __kernel void #{name}(
      __global #{type} *data,
      const #{type} num
    ) {
        data[get_global_id(0)] += num;
    }
    """

    {name, [{type, :buffer}, {type, :number}], src}
  end

  def full do
    OpenCL.T.number_types()
    |> Enum.flat_map(fn t ->
      {_name, _, add_one_src} = buffer_add_one(t)
      [add_one_src]
    end)
    |> Enum.join("\n")
  end
end
