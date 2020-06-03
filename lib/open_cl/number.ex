defmodule OpenCL.Number do
  use OpenCL.T

  defmodule NumberError do
    defexception [:number_type, :value, :reason, :index]

    def message(%NumberError{} = err) do
      IO.iodata_to_binary([
        render_reason(err),
        render_value(err),
        render_number_type(err),
        render_index(err),
      ])

    end
    defp render_reason(%NumberError{reason: reason}) do
      ["reason: ", inspect(reason), "\n"]
    end

    defp render_value(%NumberError{value: value}) do
      ["value: ", inspect(value), "\n"]
    end

    defp render_number_type(%NumberError{number_type: number_type}) do
      ["number_type: ", inspect(number_type), "\n"]
    end

    def render_index(%NumberError{index: nil}) do
      []
    end

    def render_index(%NumberError{index: i}) when is_integer(i) do
      [to_string(i)]
    end

  end

  defguard is_positive(n) when is_number(n) and n > 0
  defguard is_non_neg(n) when is_number(n) and n >= 0
  defguard is_negative(n) when is_number(n) and n < 0

  defguardp is_in_range(n, low, high) when is_number(n) and n >= low and n <= high

  def zero(t) when T.is_int_type(t), do: 0
  def zero(t) when T.is_float_type(t), do: 0.0

  @min_char -128
  @max_char 127

  defguard is_char_castable(n) when is_in_range(n, @min_char, @max_char)
  defguard is_char(n) when is_integer(n) and is_char_castable(n)

  @max_uchar 255

  defguard is_uchar_castable(n) when is_in_range(n, 0, @max_uchar)
  defguard is_uchar(n) when is_integer(n) and is_uchar_castable(n)

  @min_short -32_768
  @max_short 32_767

  defguard is_short_castable(n) when is_in_range(n, @min_short, @max_short)
  defguard is_short(n) when is_integer(n) and is_short_castable(n)

  @max_ushort 65_535

  defguard is_ushort_castable(n) when is_in_range(n, 0, @max_ushort)
  defguard is_ushort(n) when is_integer(n) and n in 0..@max_ushort

  @min_int -2_147_483_648
  @max_int 2_147_483_647

  defguard is_int_castable(n) when is_in_range(n, @min_int, @max_int)
  defguard is_int(n) when is_integer(n) and is_int_castable(n)

  @max_uint 4_294_967_295

  defguard is_uint_castable(n) when is_in_range(n, 0, @max_uint)
  defguard is_uint(n) when is_integer(n) and is_uint_castable(n)

  @min_long -9_223_372_036_854_775_808
  @max_long 9_223_372_036_854_775_807

  defguard is_long_castable(n) when is_in_range(n, @min_long, @max_long)
  defguard is_long(n) when is_integer(n) and is_long_castable(n)

  @max_ulong 18_446_744_073_709_551_615

  defguard is_ulong_castable(n) when is_in_range(n, 0, @max_ulong)
  defguard is_ulong(n) when is_integer(n) and is_ulong_castable(n)

  @max_size_t (case :erlang.system_info(:wordsize) do
    8 -> @max_ulong
    4 -> @max_uint
  end)

  defguard is_size_t_castable(n) when is_in_range(n, 0, @max_size_t)
  defguard is_size_t(n) when is_integer(n) and is_size_t_castable(n)

  @min_float -3.40282347e+38
  @max_float 3.40282347e+38

  defguard is_float32_castable(n) when is_in_range(n, @min_float, @max_float)
  defguard is_float32(n) when is_float(n) and is_float32_castable(n)

  @min_double -1.7976931348623157e+308
  @max_double 1.7976931348623157e+308

  defguard is_double_castable(n) when is_in_range(n, @min_double, @max_double)
  defguard is_double(n) when is_float(n)

  def max(:char), do: @max_char
  def max(:uchar), do: @max_uchar
  def max(:short), do: @max_short
  def max(:ushort), do: @max_ushort
  def max(:int), do: @max_int
  def max(:uint), do: @max_uint
  def max(:long), do: @max_long
  def max(:ulong), do: @max_ulong
  def max(:size_t), do: @max_ulong
  def max(:float), do: @max_float
  def max(:double), do: @max_double

  def min(:char), do: @min_char
  def min(:short), do: @min_short
  def min(:int), do: @min_int
  def min(:long), do: @min_long
  def min(:float), do: @min_long
  def min(:double), do: @min_long
  def min(t) when T.is_unsigned_int_type(t), do: 0
  # char
  def cast(:char, n) when is_char(n), do: {:ok, {:char, n}}
  def cast(:char, n) when is_float(n) and is_char_castable(n), do: {:ok, {:char, float_to_integer(n)}}
  # uchar
  def cast(:uchar, n) when is_uchar(n), do: {:ok, {:uchar, n}}
  def cast(:uchar, n) when is_float(n) and is_uchar_castable(n), do: {:ok, {:uchar, float_to_integer(n)}}
  # short
  def cast(:short, n) when is_short(n), do: {:ok, {:short, n}}
  def cast(:short, n) when is_float(n) and is_short_castable(n), do: {:ok, {:short, float_to_integer(n)}}
  # ushort
  def cast(:ushort, n) when is_ushort(n), do: {:ok, {:ushort, n}}
  def cast(:ushort, n) when is_float(n) and is_ushort_castable(n), do: {:ok, {:ushort, float_to_integer(n)}}
  # int
  def cast(:int, n) when is_int(n), do: {:ok, {:int, n}}
  def cast(:int, n) when is_float(n) and is_int_castable(n), do: {:ok, {:int, float_to_integer(n)}}
  # uint
  def cast(:uint, n) when is_uint(n), do: {:ok, {:uint, n}}
  def cast(:uint, n) when is_float(n) and is_uint_castable(n), do: {:ok, {:uint, float_to_integer(n)}}
  # long
  def cast(:long, n) when is_long(n), do: {:ok, {:long, n}}
  def cast(:long, n) when is_float(n) and is_long_castable(n), do: {:ok, {:long, float_to_integer(n)}}
  # ulong
  def cast(:ulong, n) when is_ulong(n), do: {:ok, {:ulong, n}}
  def cast(:ulong, n) when is_float(n) and is_ulong_castable(n), do: {:ok, {:ulong, float_to_integer(n)}}
  # size_t
  def cast(:size_t, n) when is_size_t(n), do: {:ok, {:size_t, n}}
  def cast(:size_t, n) when is_float(n) and is_size_t_castable(n), do: {:ok, {:size_t, float_to_integer(n)}}
  # float
  def cast(:float, n) when is_float32(n), do: {:ok, {:float, n}}
  def cast(:float, n) when is_integer(n) and is_float32_castable(n), do: {:ok, {:float, integer_to_float(n)}}
  # double
  def cast(:double, n) when is_double(n), do: {:ok, {:double, n}}
  def cast(:double, n) when is_integer(n) and is_double_castable(n), do: {:ok, {:double, integer_to_float(n)}}
  # everything else

  def cast(t, nums) when T.is_number_type(t) and is_list(nums) do
    cast_list(t, nums, [])
  end

  def cast(t, n), do: {:error, {:not_castable, {t, n}}}

  def cast!(t, n) do
    case cast(t, n) do
      {:ok, casted} -> casted
      {:error, _} -> raise "Failed to cast #{inspect(t)} from number #{inspect(n)}"
    end
  end

  def is_castable?(:char, n), do: is_char_castable(n)
  def is_castable?(:uchar, n), do: is_uchar_castable(n)
  def is_castable?(:short, n), do: is_short_castable(n)
  def is_castable?(:ushort, n), do: is_ushort_castable(n)
  def is_castable?(:int, n), do: is_int_castable(n)
  def is_castable?(:uint, n), do: is_uint_castable(n)
  def is_castable?(:long, n), do: is_long_castable(n)
  def is_castable?(:ulong, n), do: is_ulong_castable(n)
  def is_castable?(:size_t, n), do: is_size_t_castable(n)
  def is_castable?(:float, n), do: is_float32_castable(n)
  def is_castable?(:double, n), do: is_double_castable(n)
  def is_castable?(_, _), do: false

  defp cast_list(t, [], acc) do
    {:ok, {t, Enum.reverse(acc)}}
  end
  defp cast_list(t, [head | tail], acc) do
    case cast(t, head) do
      {:ok, {^t, casted}} -> cast_list(t, tail, [casted | acc])
      {:error, _} = err -> err
    end
  end


  defp float_to_integer(n) when is_float(n) do
    n |> Float.round() |> trunc()
  end

  defp integer_to_float(n) when is_integer(n) do
    n * 1.0
  end

  def check({t, n}), do: do_check(t, n)

  defp do_check(t, n) when T.is_number_type(t) and is_number(n) do
    if is_castable?(t, n) do
      :ok
    else
      err = %NumberError{
        number_type: t,
        value: n,
        reason: :invalid_number_value,
      }
      {:error, err}
    end
  end
  defp do_check(t, nums) when is_list(nums) do
    nums
    |> Enum.with_index()
    |> Enum.reduce([], fn
      {num, index}, acc when is_number(num) ->
        case do_check(t, num) do
          :ok ->
            acc
          {:error, %NumberError{} = err} ->
            [%NumberError{err | index: index} | acc]
        end
    end)
    |> case do
      [] ->
        :ok
      [_ | _] = errors ->
        {:error, Enum.reverse(errors)}
    end
  end

  defp do_check(t, n) when not is_number(n) do
    err = %NumberError{
      number_type: t,
      value: n,
      reason: :not_a_number,
    }
    {:error, err}
  end

  defp do_check(t, n) when not T.is_number_type(t) do
    err = %NumberError{
      number_type: t,
      value: n,
      reason: :invalid_number_type,
    }
    {:error, err}
  end
end
