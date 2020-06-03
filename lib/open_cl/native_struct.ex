defmodule OpenCL.NativeStruct do
  # TODO: Change NativeStruct to ResourceStruct
  defmacro __using__(_) do
    quote do
      alias OpenCL.Native

      @namespace __MODULE__
                 |> Module.split()
                 |> List.last()
                 |> Macro.underscore()
                 |> String.to_atom()

      @type t :: %__MODULE__{
              __native__: Native.resource_ref()
            }

      defstruct [:__native__]

      import OpenCL.NativeStruct, only: [method: 1, func_0: 1]
    end
  end

  defmacro func_0(name) do
    quote do
      defdelegate unquote(name)(), to: OpenCL.Native, as: :"#{@namespace}_#{unquote(name)}"
    end
  end

  defmacro method(name) do
    quote do
      defdelegate unquote(name)(ex_struct),
        to: OpenCL.Native,
        as: :"#{@namespace}_self_#{unquote(name)}"
    end
  end
end
