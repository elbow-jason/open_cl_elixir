defmodule OpenCL.CommandQueueOpts do
  alias OpenCL.CommandQueueOpts
  defstruct [
    is_blocking: true,
    offset: 0
  ]

  def build(opts) do
    %CommandQueueOpts{
      is_blocking: Keyword.get(opts, :is_blocking, true),
      offset: Keyword.get(opts, :offset, 0),
    }
  end

  def validate(%CommandQueueOpts{} = cq_opts) do
    case cq_opts do
      %CommandQueueOpts{is_blocking: b} when not is_boolean(b) ->
        {:error, [is_blocking: "must be a boolean"]}
      %CommandQueueOpts{offset: o} when not is_integer(o) ->
        {:error, [offset: "must be a non-negative integer"]}
      %CommandQueueOpts{offset: o} when o < 0 ->
        {:error, [offset: "must be a non-negative integer"]}
      _ ->
        {:ok, cq_opts}

    end
  end
end
