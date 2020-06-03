defmodule OpenCL.MemConfig do
  alias OpenCL.MemConfig

  @type t :: %MemConfig{
          kernel_access: kernel_access | nil,
          host_access: host_access | nil,
          mem_allocation: mem_allocation | nil
        }

  defstruct kernel_access: nil,
            host_access: nil,
            mem_allocation: nil

  @type kernel_access :: :read_only | :write_only | :read_write

  @kernel_access [
    :read_only,
    :write_only,
    :read_write
  ]

  @type host_access :: :read_only | :write_only | :no_access | :read_write

  @host_access [
    :read_only,
    :write_only,
    :no_access,
    :read_write
  ]

  @type mem_allocation ::
          :keep_in_place | :alloc_on_device | :copy_to_device | :force_copy_to_device

  @mem_allocation [
    :keep_in_place,
    :alloc_on_device,
    :copy_to_device,
    :force_copy_to_device
  ]

  def build(opts) do
    %{
      kernel_access: Keyword.get(opts, :kernel_access),
      host_access: Keyword.get(opts, :host_access),
      mem_allocation: Keyword.get(opts, :mem_allocation)
    }
  end

  def to_native(%{kernel_access: nil, host_access: nil, mem_allocation: nil}), do: nil
  def to_native(%{} = cfg), do: Map.from_struct(cfg)

  def errors(%{} = mc) do
    validate_field(mc, :kernel_access, @kernel_access) ++
      validate_field(mc, :host_access, @host_access) ++
      validate_field(mc, :mem_allocation, @mem_allocation)
  end

  defp validate_field(mc, field, permitted) do
    val = Map.fetch!(mc, field)

    cond do
      val in permitted -> []
      val == nil -> []
      true -> [{field, "is invalid"}]
    end
  end
end
