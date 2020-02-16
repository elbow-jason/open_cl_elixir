defmodule OpenCL.CommandQueueOpts do
  alias OpenCL.CommandQueueOpts

  @type native ::
          nil
          | %{
              is_blocking: boolean() | nil,
              offset: non_neg_integer() | nil
            }

  defstruct is_blocking: nil, offset: nil

  def build(opts) do
    %CommandQueueOpts{
      is_blocking: Keyword.get(opts, :is_blocking),
      offset: Keyword.get(opts, :offset)
    }
  end

  def to_native(%CommandQueueOpts{is_blocking: nil, offset: nil}), do: nil
  def to_native(%CommandQueueOpts{} = cq_opts), do: Map.from_struct(cq_opts)

  def errors(%CommandQueueOpts{} = cq_opts) do
    is_blocking_errors(cq_opts) ++ offset_errors(cq_opts)
  end

  defp is_blocking_errors(%CommandQueueOpts{is_blocking: is_blocking}) do
    case is_blocking do
      nil -> []
      true -> []
      false -> []
      _ -> [is_blocking: "must be nil or boolean"]
    end
  end

  defp offset_errors(%CommandQueueOpts{offset: offset}) do
    case offset do
      _ when is_integer(offset) and offset <= 0 -> []
      _ -> [offset: "must be a non-negative integer"]
    end
  end
end
