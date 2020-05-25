defmodule OpenCL.Work do
  alias OpenCL.Work
  alias OpenCL.Dims

  @type t :: %Work{
          global_work_size: Dims.t(),
          global_work_offset: nil | Dims.t(),
          local_work_size: nil | Dims.t()
        }

  @type native :: %{
          global_work_size: Dims.t(),
          global_work_offset: nil | Dims.t(),
          local_work_size: nil | Dims.t()
        }

  @type option ::
          {:global_work_size, Dims.t()}
          | {:global_work_offset, Dims.t()}
          | {:local_work_size, Dims.t()}

  @type builder :: [option] | t() | Dims.t()

  defstruct [:global_work_size, :global_work_offset, :local_work_size]

  @spec build(builder()) :: t()
  def build(%Work{} = w), do: w
  def build(opts) when is_list(opts), do: from_opts(opts)

  def build(dimensional) do
    if Dims.is_dims?(dimensional) do
      from_gws(dimensional)
    else
      raise "Cannot build OpenCl.Work from #{inspect(dimensional)}"
    end
  end

  def from_gws(gws) do
    %Work{global_work_size: gws, global_work_offset: nil, local_work_size: nil}
  end

  @spec from_opts([option]) :: t()
  def from_opts(opts) when is_list(opts) do
    %Work{
      global_work_size: Keyword.get(opts, :global_work_size),
      global_work_offset: Keyword.get(opts, :global_work_offset),
      local_work_size: Keyword.get(opts, :local_work_size)
    }
  end

  def errors(%Work{} = work) do
    List.flatten([
      global_work_size_errors(work),
      global_work_offset_errors(work),
      local_work_size_errors(work)
    ])
  end

  defp global_work_size_errors(%Work{global_work_size: gws}) do
    if Dims.is_dims?(gws) do
      []
    else
      [global_work_size: "must be dimensional"]
    end
  end

  defp global_work_offset_errors(%Work{global_work_offset: gwo}) do
    cond do
      is_nil(gwo) -> []
      Dims.is_dims?(gwo) -> []
      true -> [global_work_offset: "must be dimensional or nil"]
    end
  end

  defp local_work_size_errors(%Work{local_work_size: lws}) do
    cond do
      is_nil(lws) -> []
      Dims.is_dims?(lws) -> []
      true -> [local_work_size: "must be dimensional or nil"]
    end
  end

  def to_native(%Work{global_work_size: nil}) do
    raise "OpenCL.Work :global_work_size must be dimensional"
  end

  def to_native(%Work{} = work) do
    Map.from_struct(work)
  end
end
