defmodule OpenCL.BufferTest do
  use ExUnit.Case
  alias OpenCL.Buffer
  alias OpenCL.Session

  doctest Buffer

  use OpenCL.SessionsCase

  describe "buffer creation" do
    for t <- OpenCL.T.int_types() ++ OpenCL.T.float_types() do
      @number_type t
      test "#{t} buffer can be created from with a length", %{sessions: sessions} do
        for session <- sessions do
          assert {:ok, buffer} = Session.create_buffer_with_length(session, @number_type, 10)
          assert Buffer.number_type(buffer) == @number_type
        end
      end
    end
  end
end
