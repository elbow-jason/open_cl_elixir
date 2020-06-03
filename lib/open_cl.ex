defmodule OpenCL do
  @moduledoc """
  Documentation for OpenCL.
  """

  @spec add_one_uchar_src :: binary()
  def add_one_uchar_src do
    """
    __kernel void add_one_uchar(__global uchar *data) {
        data[get_global_id(0)] += 1;
    }
    """
  end

  def simple_demo do
    src = add_one_uchar_src()
    IO.puts("with src:n\#{src}")
    IO.puts("create session...")
    {:ok, [session | _]} = OpenCL.Session.create(src)
    IO.puts("session created: #{inspect(session)}\n")
    IO.puts("create an array of uchar...")
    # put a zero on the end to keep the charlist from printing
    {:ok, arr1} = OpenCL.Array.filled_with({:uchar, 8}, 100) |> OpenCL.Array.push(0)
    IO.puts("created uchar array: #{inspect(arr1)}")
    IO.puts("uchar data: #{inspect(OpenCL.Array.to_list(arr1))}\n")

    # Array buffer
    IO.puts("create a buffer from the array...")
    {:ok, buffer1} = OpenCL.Session.create_buffer_from_data(session, arr1)
    IO.puts("created buffer from array: #{inspect(buffer1)}\n")

    IO.puts("create a buffer from a list...")
    nums = Enum.to_list(0..255)
    IO.puts("the list: #{inspect(nums)}")
    {:ok, buffer2} = OpenCL.Session.create_buffer_from_data(session, {:uchar, nums})
    IO.puts("created buffer from list: #{inspect(buffer2)}\n")

    IO.puts("create a buffer from a length (non_neg_integer)...")
    len = 148
    IO.puts("the length: #{inspect(len)}")
    {:ok, buffer3} = OpenCL.Session.create_buffer_with_length(session, :uchar, len)
    IO.puts("created buffer from length: #{inspect(buffer3)}\n")

    IO.puts("execute a kernel...")

    kernel_name = "add_one_uchar"
    kernel_work = OpenCL.Buffer.length(buffer1)
    kernel_op = OpenCL.KernelOp.build(kernel_name, kernel_work, [buffer1])
    IO.puts("build a kernel operation: #{inspect(kernel_op)}")

    {micros, op_result} = :timer.tc(fn ->
      OpenCL.Session.execute_kernel_op(session, kernel_op)
    end)

    ops_per_sec = div(1_000_000, micros)
    IO.puts("kernel operation took #{micros} microseconds (estimated #{ops_per_sec} ops/sec)")
    IO.puts("kernel operation execution result: #{inspect(op_result)}\n")

    IO.puts("after the successful op read the buffer into an array...")
    {:ok, arr2} = OpenCL.Session.read_buffer(session, buffer1)
    IO.puts("the data is moved from the buffer to the array: #{inspect(arr2)}")
    num_list2 = OpenCL.Array.to_list(arr2)
    IO.puts("the data is the array into an elixir list: #{inspect(num_list2)}")
    arr1_len = OpenCL.Array.length(arr1)
    arr2_len = OpenCL.Array.length(arr2)
    IO.puts("the new array and the old array are the same length: #{arr1_len == arr2_len} = #{arr1_len} == #{arr2_len}")
    data1 = OpenCL.Array.to_list(arr1)
    data2 = OpenCL.Array.to_list(arr2)
    added_one? =
      data1
      |> Enum.zip(data2)
      |> Enum.all?(fn {d1, d2} -> d1 + 1 == d2 end)
    IO.puts("the new array is the old array with 1 added: #{added_one?}")
  end
end
