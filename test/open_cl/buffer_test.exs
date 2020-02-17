defmodule OpenCL.BufferTest do
  use ExUnit.Case
  alias OpenCL.Buffer
  alias OpenCL.Session

  doctest Buffer

  use OpenCL.SessionsCase

  @integer_types ~w(u8 i8 u16 i16 u32 i32 u64 i64 usize isize)a
  # @integer_types ~w(u8 i8 u16 i16 u32 i32)a # u64 i64 usize isize)a
  @float_types ~w(f32 f64)a

  describe "buffer creation" do
    for t <- @integer_types ++ @float_types do
      @number_type t
      test "#{t} buffer can be created from with a length", %{sessions: sessions} do
        for session <- sessions do
          assert {:ok, buffer} = Session.create_buffer(session, @number_type, 10)
          assert Buffer.number_type(buffer) == @number_type
        end
      end
    end
  end
end
