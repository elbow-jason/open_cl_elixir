defmodule OpenCL.ArrayTest do
  use ExUnit.Case
  alias OpenCL.Array

  describe "new/1" do

    test "work for i8" do
      content = [-1, -2, -3, 0, 1, 2, 3]
      array = Array.new(:i8, content)
      assert Array.to_list(array) == content
    end

    test "work for u8" do
      content = [0, 1, 2, 3]
      array = Array.new(:u8, content)
      assert Array.to_list(array) == content
    end

    test "preserves order in conjuction with to_list" do
      content = [1, 2, 3]
      array = Array.new(:u8, content)
      assert Array.to_list(array) == content
    end

  end

  describe "filled_with/1" do
    test "returns an array filled with `n` cound of `val`" do
      array = Array.filled_with(:u8, 3, 10)
      content = Array.to_list(array)
      assert length(content) == 10
      Enum.each(content, fn item ->
        assert item == 3
      end)
    end
  # (number, count), to: Native, as: :array_new_filled_with
  end
  describe "to_list/1" do
    test "returns the contents of the array as a list" do
      array = Array.new(:u8, [1, 2, 3])
      assert Array.to_list(array) == [1, 2, 3]
    end
  end
  describe "length/1" do
    test "returns the length of the array" do
      data = [1, 2, 3]
      array = Array.new(:u8, data)
      assert length(data) == Array.length(array)
      assert Array.length(array) == 3
    end
  # (array), to: Native, as: :array_length
  end
  describe "push/1" do
    test "pushes items onto the array" do
      array = Array.new(:u8, [1, 2, 3])
      assert :ok = Array.push(array, 4)
      assert Array.to_list(array) == [1, 2, 3, 4]
    end
  end
  describe "extend/2" do
    test "mutates the extended array but does not change the extender" do
      array = Array.filled_with(:u8, 3, 2)
      # can't mutate a list so nothing to check here
      assert :ok = Array.extend(array, [4, 4])
      assert Array.to_list(array) == [3, 3, 4, 4]
      array2 = Array.filled_with(:u8, 5, 2)
      assert :ok = Array.extend(array, array2)
      array2_before = Array.to_list(array2)
      assert array2_before == [5, 5]
      assert Array.to_list(array) == [3, 3, 4, 4, 5, 5]
      array2_after = Array.to_list(array2)
      assert array2_after == [5, 5]
    end
    test "works for list of numbers" do
      array = Array.filled_with(:u8, 3, 2)
      assert :ok = Array.extend(array, [4, 4])
      assert Array.to_list(array) == [3, 3, 4, 4]
    end
    test "works for other Array" do
      array1 = Array.filled_with(:u8, 3, 2)
      array2 = Array.filled_with(:u8, 7, 2)
      assert :ok = Array.extend(array1, array2)
      assert Array.to_list(array1) == [3, 3, 7, 7]
    end
    test "works for itself (the same array because the Rust involves locking)" do
      # this will block forever if the RwLock contends for write-lock access on it's own read-lock.
      array = Array.filled_with(:u8, 3, 10)
      assert :ok = Array.extend(array, array)
      assert Array.length(array) == 20
    end
  end


  describe "number_type/1" do
    test "works for u8" do
      array = Array.filled_with(:u8, 1, 2)
      assert Array.type(array) == :u8
    end

    test "works for i8" do
      array = Array.filled_with(:i8, -1, 2)
      assert Array.type(array) == :i8
    end
  end

  @all_number_types ~w(u8 i8 u16 i16 u32 i32 f32 u64 i64 f64 usize isize)a

  describe "array_cast/2 with u8 array" do
    test "can be casted to other types" do
      array = Array.filled_with(:u8, 1, 2)
      assert Array.type(array) == :u8
      Enum.each(@all_number_types, fn number_type ->
        array2 = Array.type_cast(array, number_type)
        assert Array.type(array2) == number_type
      end)

    end

    # test "works for u8 to i8" do
    #   array = Array.filled_with(:u8, 1, 2)
    #   assert Array.type(array) == :u8
    #   array2 = Array.type_cast(array, :i8)
    #   assert Array.type(array2) == :i8
    # end

    # test "works for u8 to i16" do
    #   array = Array.filled_with(:u8, 1, 2)
    #   assert Array.type(array) == :u8
    #   array2 = Array.type_cast(array, :i16)
    #   assert Array.type(array2) == :i16
    # end

    # test "works for u8 to i32" do
    #   array = Array.filled_with(:u8, 1, 2)
    #   assert Array.type(array) == :u8
    #   array2 = Array.type_cast(array, :i32)
    #   assert Array.type(array2) == :i32
    # end

    # test "works for u8 to i64" do
    #   array = Array.filled_with(:u8, 1, 2)
    #   assert Array.type(array) == :u8
    #   array2 = Array.type_cast(array, :i64)
    #   assert Array.type(array2) == :i64
    # end

    # test "works for u8 to isize" do
    #   array = Array.filled_with(:u8, 1, 2)
    #   assert Array.type(array) == :u8
    #   array2 = Array.type_cast(array, :isize)
    #   assert Array.type(array2) == :isize
    # end

    # test "works for u8 to isize" do
    #   array = Array.filled_with(:u8, 1, 2)
    #   assert Array.type(array) == :u8
    #   array2 = Array.type_cast(array, :isize)
    #   assert Array.type(array2) == :isize
    # end
  end
end
