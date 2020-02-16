defmodule OpenCL.KernelOp do
  alias OpenCL.Buffer
  alias OpenCL.KernelOp
  alias OpenCL.CommandQueueOpts
  alias OpenCL.Work

  @type arg :: number() | Buffer.t()
  @type args :: [arg]
  @type arg_index :: non_neg_integer()
  @type name :: String.t()
  @type work :: Work.t()
  @type work_builder :: Work.builder()
  @type option :: {:command_queue_opts, CommandQueueOpts.t()}
  @type options :: [option]

  @type t :: %KernelOp{
          name: name(),
          args: args(),
          work: work(),
          returning: nil | arg_index(),
          command_queue_opts: CommandQueueOpts.t()
        }

  defstruct name: nil,
            work: nil,
            returning: nil,
            command_queue_opts: nil,
            args: []

  @spec build(name(), work_builder(), args, options) :: t()
  def build(name, work_builder, args, opts \\ []) when is_list(args) do
    %KernelOp{
      name: name,
      work: Work.build(work_builder),
      args: args,
      returning: Keyword.get(opts, :returning),
      command_queue_opts: resolve_command_queue_opts(opts)
    }
  end

  def get_return_value(%KernelOp{returning: nil}), do: nil

  def get_return_value(%KernelOp{returning: index, args: args}) when is_integer(index) do
    Enum.at(args, index)
  end

  def to_native(%KernelOp{work: work, command_queue_opts: cq_opts} = kernel_op) do
    %KernelOp{kernel_op | work: Work.to_native(work), command_queue_opts: CommandQueueOpts.to_native(cq_opts)}
  end

  def errors(%KernelOp{} = op) do
    List.flatten([
      name_errors(op),
      args_errors(op),
      work_errors(op),
      returning_errors(op),
      cq_opts_errors(op)
    ])
  end

  defp resolve_command_queue_opts(opts) do
    case Keyword.fetch(opts, :command_queue_opts) do
      {:ok, %CommandQueueOpts{} = cq_opts} ->
        cq_opts

      {:ok, not_cq_opts} ->
        raise "Invalid option :command_queue_opts value. Expected %OpenCL.CommandQueueOpts{}. Got: #{
                inspect(not_cq_opts)
              }"

      :error ->
        CommandQueueOpts.build(opts)
    end
  end

  defp work_errors(%KernelOp{work: work}) do
    Work.errors(work)
  end

  defp cq_opts_errors(%KernelOp{command_queue_opts: %CommandQueueOpts{} = cq_opts}) do
    CommandQueueOpts.errors(cq_opts)
  end

  defp cq_opts_errors(%KernelOp{command_queue_opts: nil}) do
    []
  end

  defp cq_opts_errors(%KernelOp{command_queue_opts: _}) do
    [command_queue_opts: "must be a CommandQueueOpts struct or nil"]
  end

  defp args_errors(%KernelOp{args: args}) when not is_list(args) do
    [args: "must be a list of args"]
  end

  defp args_errors(%KernelOp{args: args}) when is_list(args) do
    args
    |> Enum.with_index()
    |> Enum.flat_map(fn {arg, index} ->
      if is_arg?(arg) do
        []
      else
        [args: "invalid arg at index #{index}"]
      end
    end)
  end

  defp name_errors(%KernelOp{name: name}) when is_binary(name) do
    if String.printable?(name) do
      []
    else
      [name: "must be a valid string"]
    end
  end

  defp name_errors(_), do: [name: "must be a valid string"]

  @doc false
  def is_arg?(n) when is_number(n), do: true
  def is_arg?(%Buffer{}), do: true
  def is_arg?(_), do: false

  defp returning_errors(%KernelOp{returning: ret, args: args}) do
    case ret do
      nil -> []
      _ when not is_integer(ret) -> [returning: "must be a non-negative integer or nil"]
      _ when ret < 0 -> [returning: "must be a non-negative integer or nil"]
      _ when ret >= length(args) -> [returning: "index is out of bounds"]
      _ -> []
    end
  end
end
