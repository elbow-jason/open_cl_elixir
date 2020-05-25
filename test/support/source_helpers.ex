defmodule OpenCL.SourceHelpers do
  alias OpenCL.Buffer
  def to_type(x), do: to_string(x)

  def to_arg(name, :buffer, type), do: "__global #{to_type(type)} *#{name}"

  def buffer_add_one(type) do
    name = "add_one_#{type}"

    src = """
    __kernel void #{name}(__global #{to_type(type)} *data) {
        data[get_global_id(0)] += 1;
    }
    """

    {name, [{:buffer, type}], src}
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
