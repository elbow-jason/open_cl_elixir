defmodule OpenCL.StatusCodeError do
  alias OpenCL.StatusCodeError

  @type t :: %StatusCodeError{
    status_code: integer(),
    description: String.t()
  }

  defexception [status_code: 0, description: "Success"]

  @spec message(t()) :: String.t()
  def message(%StatusCodeError{status_code: status_code, description: description}) do
    "OpenCL FFI call failed with status code #{status_code} (#{description})"
  end
end
