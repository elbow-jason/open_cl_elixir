defmodule OpenCL.ArrayTest do
  use ExUnit.Case, async: true
  use OpenCL.Test.ArrayHelpers

  @integer_types ~w(uchar char ushort short uint int ulong long size_t)a
  # @integer_types ~w(u8 i8 u16 i16 u32 i32)a # u64 i64 usize isize)a
  @float_types ~w(float double)a
  # @float_types ~w()a
  @all_types @integer_types ++ @float_types

  describe "new/1" do
    test "preserves order in conjuction with to_list" do
      content = [1, 2, 3]
      assert {:ok, array} = Array.new({:uchar, content})
      assert Array.to_list(array) == content
    end
  end

  describe "type/2" do
    for t <- @integer_types do
      ArrayHelpers.test_number_type(t, [1, 2, 3])
    end

    for t <- @float_types do
      ArrayHelpers.test_number_type(t, [1.0, 2.0, 3.0])
    end
  end

  describe "new/2" do
    for t <- @integer_types do
      ArrayHelpers.test_new(t, [1, 2, 3])
    end

    for t <- @float_types do
      ArrayHelpers.test_new(t, [1.0, 2.0, 3.0])
    end
  end

  describe "filled_with/1" do
    test "returns an array filled with `n` cound of `val`" do
      assert {:ok, array} = Array.filled_with({:uchar, 3}, 10)
      content = Array.to_list(array)
      assert length(content) == 10

      Enum.each(content, fn item ->
        assert item == 3
      end)
    end

    for t <- @integer_types do
      ArrayHelpers.test_filled_with(t, 2, 3)
    end

    for t <- @float_types do
      ArrayHelpers.test_filled_with(t, 2.0, 3)
    end
  end

  describe "to_list/1" do
    test "returns the contents of the array as a list" do
      assert {:ok, array} = Array.new({:char, [1, 2, 3]})
      assert Array.to_list(array) == [1, 2, 3]
    end

    for t <- @integer_types do
      ArrayHelpers.test_to_list(t, [1, 2, 3])
    end

    for t <- @float_types do
      ArrayHelpers.test_to_list(t, [1.0, 2.0, 3.0])
    end
  end

  describe "length/1" do
    test "matches the length of the content" do
      content = [1, 2, 3]
      assert {:ok, array} = Array.new({:char, content})
      assert Array.length(array) == length(content)
    end

    for t <- @integer_types do
      ArrayHelpers.test_length(t, [1, 2, 3])
    end

    for t <- @float_types do
      ArrayHelpers.test_length(t, [1.0, 2.0, 3.0])
    end
  end

  describe "push/2" do
    for t <- @integer_types do
      ArrayHelpers.test_push(t, [1, 2, 3], 4)
    end

    for t <- @float_types do
      ArrayHelpers.test_push(t, [1.0, 2.0, 3.0], 4.0)
    end
  end

  describe "extend/2" do
    for t <- @integer_types do
      ArrayHelpers.test_extend(t, [1, 2, 3], [4, 5])
    end

    for t <- @float_types do
      ArrayHelpers.test_extend(t, [1.0, 2.0, 3.0], [4.0, 5.0])
    end
  end

  describe "type_cast/2" do
    for left <- @all_types do
      for right <- @all_types do
        ArrayHelpers.test_type_cast(left, right)
      end
    end
  end
end
