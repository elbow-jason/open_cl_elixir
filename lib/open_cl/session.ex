defmodule OpenCL.Session do
  use OpenCL.NativeStruct
  alias OpenCL.Array
  alias OpenCL.Buffer
  alias OpenCL.Device
  alias OpenCL.MemConfig
  alias OpenCL.CommandQueueOpts
  alias OpenCL.Session
  alias OpenCL.KernelOp
  alias OpenCL.Work

  @spec create(String.t()) :: Native.output(t())
  def create(src), do: Native.session_create(src)

  @spec create(String.t(), Device.t() | [Device.t()]) :: Native.output(t())
  def create(src, device_or_devices) do
    device_or_devices
    |> List.wrap()
    |> Native.session_create_with_devices(src)
  end

  method(:devices)

  def create_buffer(%Session{} = session, type, len_or_data, opts \\ []) do
    case Native.session_self_create_buffer(session, type, len_or_data, native_mem_config(opts)) do
      {:ok, %Buffer{}} = okay_buffer -> okay_buffer
      :invalid_variant -> {:error, create_buffer_errors(type, len_or_data, opts)}
      {:error, _} = error -> error
    end
  end

  def write_buffer(%Session{} = session, %Buffer{} = buffer, %Array{} = array, opts \\ []) do
    {queue_index, opts} = Keyword.pop(opts, :queue_index, 0)

    case Native.session_self_write_array_to_buffer(session, queue_index, buffer, array, native_cq_opts(opts)) do
      {:ok, {}} -> :ok
      :invalid_variant -> {:error, write_buffer_errors(opts)}
      {:error, _} = err -> err
    end
  end

  def read_buffer(%Session{} = session, %Buffer{} = buffer, opts \\ []) do
    {queue_index, opts} = Keyword.pop(opts, :queue_index, 0)
    case Native.session_self_read_buffer(session, queue_index, buffer, native_cq_opts(opts)) do
      {:ok, %Array{} = arr} -> {:ok, arr}
      :invalid_variant -> {:error, read_buffer_errors(opts)}
      {:error, _} = err -> err
    end
  end

  def execute_kernel(%Session{} = session, name, args, work_builder, opts \\ []) do
    {queue_index, opts} = Keyword.pop(opts, :queue_index, 0)
    work = Work.build(work_builder)
    kernel_op =
      name
      |> KernelOp.build(args, work, opts)
      # |> KernelOp.to_native()

    case Native.session_self_execute_kernel_operation(session, queue_index, kernel_op) do
      {:ok, {}} -> {:ok, KernelOp.get_return_value(kernel_op)}
      :invalid_variant -> {:error, execute_kernel_errors(kernel_op)}
      {:error, _} = err -> err
    end
  end

  defp native_cq_opts(opts) do
    opts
    |> CommandQueueOpts.build()
    |> CommandQueueOpts.to_native()
  end

  defp native_mem_config(opts) do
    opts
    |> MemConfig.build()
    |> MemConfig.to_native()
  end

  defp create_buffer_errors(type, len_or_data, opts) do
    mem_config = MemConfig.build(opts)

    len_or_data_errors(len_or_data)
    ++ number_type_errors(type)
    ++ MemConfig.errors(mem_config)
  end

  defp write_buffer_errors(opts) do
    command_queue_opts_error(opts)
  end

  defp read_buffer_errors(opts) do
    command_queue_opts_error(opts)
  end

  defp execute_kernel_errors(kernel_op) do
    KernelOp.errors(kernel_op)
  end

  defp command_queue_opts_error(opts) do
    opts
    |> CommandQueueOpts.build()
    |> CommandQueueOpts.errors()
  end

  defp len_or_data_errors(len) when is_integer(len) and len > 0, do: []
  defp len_or_data_errors(data) when is_list(data), do: []
  defp len_or_data_errors(%Array{}), do: []
  defp len_or_data_errors(_), do: [len_or_data: "invalid value"]


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

  defp number_type_errors(t) when t in @number_types, do: []
  defp number_type_errors(_), do: [number_type: "is invalid"]

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
