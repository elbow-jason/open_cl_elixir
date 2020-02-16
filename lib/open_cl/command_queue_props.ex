defmodule OpenCL.CommandQueueProps do
  @type prop ::
          :out_of_order_execution
          | :profiling_enabled
          | :on_device
          | :on_device_default

  @type t :: [prop]

  @props [
    :out_of_order_execution,
    :profiling_enabled,
    :on_device,
    :on_device_default
  ]

  defguard is_prop(p) when p in @props

  def build(opts), do: Enum.filter(opts, fn p -> p in @props end)

  def errors(opts) do
    case opts -- @props do
      [] ->
        []
      got ->
        [command_queue_properties: "Allowed flags are #{inspect(@props)}. Got: #{inspect(got)}"]
    end
  end

  def default, do: []
end
