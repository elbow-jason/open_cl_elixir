defmodule OpenCL.Session do
  use OpenCL.NativeStruct
  alias OpenCL.Array
  alias OpenCL.Buffer
  alias OpenCL.Device
  alias OpenCL.MemConfig
  alias OpenCL.CommandQueueOpts
  alias OpenCL.Session
  alias OpenCL.KernelOp
  alias OpenCL.CommandQueueProps

  use OpenCL.T

  @type create_option ::
          {:command_queue_properties, CommandQueueProps.t()}
          | {:devices, [Device.t()]}

  @type create_result :: {:ok, [t()]} | {:error, any()}

  @spec create(String.t(), [create_option]) :: create_result()
  def create(src, opts \\ []) do
    case Keyword.fetch(opts, :devices) do
      {:ok, devices} when is_list(devices) ->
        create_with_devices(src, devices, opts)

      :error ->
        src
        |> Native.session_create(get_props(opts))
        |> handle_create(opts)
    end
  end

  @spec create_with_devices(String.t(), [Device.t()], [create_option]) :: create_result()
  def create_with_devices(src, devices, opts) do
    devices
    |> Native.session_create_with_devices(src, get_props(opts))
    |> handle_create(opts)
  end

  defp handle_create(sessions, _opts) when is_list(sessions), do: {:ok, sessions}
  defp handle_create(:invalid_variant, opts), do: create_session_errors(opts)
  defp handle_create({:error, _} = err, _), do: err

  defp create_session_errors(opts) do
    CommandQueueProps.errors(get_props(opts))
  end

  defp get_props(opts) do
    Keyword.get(opts, :command_queue_properties, CommandQueueProps.default())
  end

  method(:device)
  method(:create_copy)

  def create_buffer(%Session{} = session, %Array{} = array) do
    create_buffer(session, array, [])
  end

  def create_buffer(%Session{} = session, type, len) when T.is_len(len) do
    create_buffer(session, type, len, [])
  end

  def create_buffer(%Session{} = session, type, nums) when T.is_number_type(type) and is_list(nums) do
    create_buffer(session, type, nums, [])
  end


  def create_buffer(%Session{} = session, %Array{} = array, opts) when is_list(opts) do
    session
    |> Native.session_self_create_buffer_from_array(array, native_mem_config(opts))
    |> case do
      :invalid_variant ->
        handle_create_buffer(:invalid_variant, Array.type(array), array, opts)

      ret ->
        handle_create_buffer(ret, nil, array, opts)
    end
  end

  def create_buffer(%Session{} = session, type, len, opts) when T.is_len(len) and is_list(opts) do
    session
    |> Native.session_self_create_buffer_with_length(type, len, native_mem_config(opts))
    |> handle_create_buffer(type, len, opts)
  end

  def create_buffer(%Session{} = session, type, num_list, opts) when is_list(num_list) do
    session
    |> Native.session_self_create_buffer_from_list(type, num_list, native_mem_config(opts))
    |> handle_create_buffer(type, num_list, opts)
  end

  defp handle_create_buffer(%Buffer{} = buffer, _, _, _), do: {:ok, buffer}
  defp handle_create_buffer({:error, _} = error, _, _, _), do: error
  defp handle_create_buffer(:invalid_variant, type, builder, opts) do
    {:error, create_buffer_errors(type, builder, opts)}
  end


  def write_buffer(%Session{} = session, %Buffer{} = buffer, %Array{} = array, opts \\ []) do
    case Native.session_self_write_array_to_buffer(session, buffer, array, native_cq_opts(opts)) do
      {} -> :ok
      :invalid_variant -> {:error, write_buffer_errors(opts)}
      {:error, _} = err -> err
    end
  end

  def read_buffer(%Session{} = session, %Buffer{} = buffer, opts \\ []) do
    case Native.session_self_read_buffer(session, buffer, native_cq_opts(opts)) do
      %Array{} = arr -> {:ok, arr}
      :invalid_variant -> {:error, read_buffer_errors(opts)}
      {:error, _} = err -> err
    end
  end

  @spec execute_kernel(
          Session.t(),
          KernelOp.name(),
          KernelOp.work_builder(),
          KernelOp.args(),
          KernelOp.options()
        ) :: :ok | {:error, binary | [any]} | {:ok, any}
  def execute_kernel(%Session{} = session, name, work_builder, args, opts \\ []) do
    kernel_op =
      name
      |> KernelOp.build(work_builder, args, opts)
      |> KernelOp.to_native()

    execute_kernel_op(session, kernel_op)
  end

  def execute_kernel_op(%Session{} = session, %KernelOp{} = kernel_op) do
    case Native.session_self_execute_kernel_operation(session, kernel_op) do
      {} -> returning(kernel_op)
      :invalid_variant -> {:error, execute_kernel_errors(kernel_op)}
      {:error, _} = err -> err
    end
  end

  defp returning(%KernelOp{} = kernel_op) do
    case KernelOp.get_return_value(kernel_op) do
      nil -> :ok
      ret -> {:ok, ret}
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

    len_or_data_errors(len_or_data) ++
      number_type_errors(type) ++
      MemConfig.errors(mem_config)
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

  defp number_type_errors(t) when T.is_number_type(t), do: []
  defp number_type_errors(_), do: [number_type: "is invalid"]

  defimpl Inspect do
    @spec inspect(Session.t(), any) :: String.t()
    def inspect(session, _) do
      device = Session.device(session)
      "#OpenCL.Session<[device: #{inspect(device)}]>"
    end
  end
end
