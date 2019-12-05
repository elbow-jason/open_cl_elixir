defmodule OpenCL.NifNotLoadedError do
  defexception message: "nif not loaded"

  @doc """
  This function raise an `OpenCLNifNotLoadedError`.

  In this function use `apply/3` to remove the type information
  from our function call so dialyzer won't complain.

  Yes, it's a hack, but it's *our* hack.
  """
  @spec err() :: any()
  def err, do: apply(__MODULE__, :do_raise_err, [])

  # needed for apply to trick dialyzer
  @doc false
  def do_raise_err, do: raise(OpenCL.NifNotLoadedError)
end
