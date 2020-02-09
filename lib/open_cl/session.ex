defmodule OpenCL.Session do
  use OpenCL.NativeStruct
  alias OpenCL.Array
  alias OpenCL.Device
  alias OpenCL.MemConfig
  alias OpenCL.Session

  @spec create(String.t()) :: Native.output(t())
  def create(src), do: Native.session_create(src)

  @spec create(String.t(), Device.t() | [Device.t()]) :: Native.output(t())
  def create(src, device_or_devices) do
    device_or_devices
    |> List.wrap()
    |> Native.session_create_with_devices(src)
  end

  method(:devices)

  def create_buffer(%Session{} = session, type, len_or_data) do
    create_buffer(%Session{} = session, type, len_or_data, [])
  end
  def create_buffer(%Session{} = session, type, len_or_data, opts \\ []) do
    with(
      {:number_type, []} <- {:number_type, validate_number_type(type)},
      {:len_or_data, []} <- {:len_or_data, validate_len_or_data(len_or_data)},
      {:opts_kwlist, true} <- {:opts_kwlist, Keyword.keyword?(opts)},
      {:ok, %MemConfig{} = mem_config} <- MemConfig.build(opts)
    ) do
      Native.session_self_create_buffer(session, type, len_or_data, Map.from_struct(mem_config))
    else
      {:opts_kwlist, false} -> {:error, [opts: "must be a keyword list"]}
      {:number_type, err} -> {:error, err}
      {:len_or_data, err} -> {:error, err}
      {:error, _} = err -> err
    end
  end


  defp validate_len_or_data(len) when is_integer(len) and len > 0, do: []
  defp validate_len_or_data(data) when is_list(data), do: []
  defp validate_len_or_data(%Array{}), do: []
  defp validate_len_or_data(_), do: [len_or_data: "invalid value"]


  @number_types [
    :u8,
    :i8,
    :u16,
    :i16,
    :u32,
    :i32,
    :f32,
    :u64,
    :i64,
    :f64,
  ]

  defp validate_number_type(t) when t in @number_types, do: []
  defp validate_number_type(_), do: [number_type: "is invalid"]

  # def kernel_execute_sync(session, name, dims, args) when is_list(args) do
  #   Native.kernel_execute_sync(session, name, dims, args)
  # end

  defimpl Inspect do
    @spec inspect(Session.t(), any) :: String.t()
    def inspect(session, _) do
      devices = Session.devices(session)
      "#OpenCL.Session<[devices: #{inspect(devices)}]>"
    end
  end
end
