defmodule OpenCL.T do
  alias OpenCL.Array
  alias OpenCL.Buffer
  alias OpenCL.Platform

  defmacro __using__(_) do
    quote do
      require OpenCL.T
      alias OpenCL.T
    end
  end

  @type len :: non_neg_integer()
  defguard is_len(len) when is_integer(len) and len > 0

  @type num_list :: {number_type(), [number(), ...]}

  @type array :: Array.t()
  @type buffer :: Buffer.t()
  @type platform :: Platform.t()

  @type buffer_builder :: len() | num_list() | array()

  @type resource_ref :: reference()

  @type output(item) :: item | {:error, String.t()} | :invalid_variant

  @type side_effect_output :: :ok | {:error, String.t()} | :invalid_variant | {}

  @type float_type :: :float | :double
  @type signed_int_type :: :char | :short | :int | :long
  @type unsigned_int_type :: :uchar | :ushort | :uint | :ulong | :size_t
  @type int_type :: signed_int_type() | unsigned_int_type()
  @type number_type :: float_type() | signed_int_type() | unsigned_int_type()
  @type number_typed_float :: {float_type(), float()}
  @type number_typed_int :: {int_type(), integer()}
  @type number_typed_number :: number_typed_float() | number_typed_int()
  @type number_typed_list_of_float() :: {float_type(), [float()]}
  @type number_typed_list_of_int() :: {int_type(), [integer()]}
  @type number_typed_list :: number_typed_list_of_float() | number_typed_list_of_int()

  @float_types [:float, :double]
  defguard is_float_type(x) when x in @float_types
  def float_types, do: @float_types


  @signed_int_types [:char, :short, :int, :long]
  defguard is_signed_int_type(t) when t in @signed_int_types
  def signed_int_types, do: @signed_int_types


  @unsigned_int_types [:uchar, :ushort, :uint, :ulong]
  defguard is_unsigned_int_type(t) when t in @unsigned_int_types
  def unsigned_int_types, do: @unsigned_int_types

  @int_types @unsigned_int_types ++ @signed_int_types
  def int_types, do: @int_types
  defguard is_int_type(t) when is_signed_int_type(t) or is_unsigned_int_type(t)

  @number_types @float_types ++ @int_types
  def number_types, do: @number_types
  defguard is_number_type(t) when is_signed_int_type(t) or is_unsigned_int_type(t) or is_float_type(t)
end
