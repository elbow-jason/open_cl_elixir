ExUnit.start()

defmodule OpenCL.Testing do
  use ExUnit.Case

  alias OpenCL.Session
  alias OpenCL.DeviceBuffer
  alias OpenCL.Array
  alias OpenCL.Device
  alias OpenCL.Platform

  defmodule NativeKernel do
    defstruct name: nil,
              src: nil,
              type: :u8

    def build(name, src, type) do
      %NativeKernel{
        name: name,
        src: src,
        type: type
      }
    end

    def name(%NativeKernel{name: name}), do: name

    def src(%NativeKernel{src: src}), do: src

    def type(%NativeKernel{type: type}), do: type
  end

  defmodule Context do
    require Logger

    defstruct kernel: nil,
              initial_value: 0,
              shape: {3, 3},
              iterations: 100,
              started_at: nil,
              stopped_at: nil,
              expected: nil,
              session: nil,
              concurrency: 1

    @src_add_one_u8 """
    __kernel void add_one_u8(__global uchar *nums) {
      int index = get_global_id(0);
      nums[index] += 1;
    }
    """

    @kernels [
      NativeKernel.build("add_one_u8", @src_add_one_u8, :u8)
    ]

    defp kernel_named?(%NativeKernel{} = kernel, name), do: NativeKernel.name(kernel) == name

    defp has_name?(name), do: fn %NativeKernel{} = kernel -> kernel_named?(kernel, name) end

    def fetch_kernel!(name), do: Enum.fetch!(@kernels, has_name?(name))

    def get_kernel(name), do: Enum.find(@kernels, has_name?(name))

    def session(%Context{session: %Session{} = session}), do: session

    def kernel(%Context{kernel: %NativeKernel{} = kernel}), do: kernel

    def iterations(%Context{iterations: iters}), do: iters

    def type(%Context{} = tc) do
      tc
      |> kernel()
      |> NativeKernel.type()
    end

    def kernel_name(%Context{} = tc) do
      tc
      |> kernel()
      |> NativeKernel.name()
    end

    def initial_value(%Context{initial_value: val}), do: val

    def shape(%Context{shape: shape}), do: shape

    def device_name(%Context{} = tc) do
      {:ok, name} =
        tc
        |> session()
        |> Session.device_name()

      name
    end

    def identifier(%Context{} = tc) do
      d = device_name(tc)
      k = kernel_name(tc)
      t = type(tc)
      s = inspect(shape(tc))
      x = iterations(tc)
      "#{d}_#{k}_#{t}_#{s}_x#{x}"
    end

    def start(%Context{started_at: nil} = tc) do
      %Context{tc | started_at: DateTime.utc_now()}
    end

    def stop(%Context{stopped_at: nil} = tc) do
      %Context{tc | stopped_at: DateTime.utc_now()}
    end

    def took_usec(%Context{started_at: start, stopped_at: stop}) do
      DateTime.diff(stop, start, :microsecond)
    end

    def average_usec_per_op(%Context{} = tc) do
      Float.round(took_usec(tc) / iterations(tc), 1)
    end

    def estimated_ops_per_sec(%Context{} = tc) do
      Float.round(1_000_000 / took_usec(tc), 1)
    end

    def expected(%Context{expected: e}), do: e

    def all_contexts do
      [
        %Context{kernel: "add_one_u8", shape: 3, iterations: 10, expected: 10},
        %Context{kernel: "add_one_u8", shape: 100_000, iterations: 10},
        %Context{kernel: "add_one_u8", shape: {10_000, 10_000}, iterations: 10},
        %Context{kernel: "add_one_u8", shape: 100_000, iterations: 1, expected: 1},
        %Context{kernel: "add_one_u8", shape: 100, iterations: 1, expected: 1},
        %Context{kernel: "add_one_u8", shape: 120, iterations: 1, expected: 1}
      ]
    end

    def all_kernels, do: @kernels

    def all_devices do
      assert {:ok, platforms} = Platform.list_all()

      Enum.flat_map(platforms, fn p ->
        assert {:ok, devices} = Platform.all_devices(p)
        Enum.filter(devices, &Device.usable?/1)
      end)
    end

    def resolve_kernel(%NativeKernel{} = k), do: k
    def resolve_kernel(name) when is_binary(name), do: get_kernel(name)

    def build_all(cfg \\ %{}) do
      contexts = Map.get(cfg, :contexts) || all_contexts()
      kernels = Map.get(cfg, :kernels) || all_kernels()
      devices = Map.get(cfg, :devices) || all_devices()

      for kernel_cfg <- kernels do
        for ctx <- contexts do
          for device <- devices do
            %NativeKernel{} =
              kernel =
              resolve_kernel(ctx.kernel) || resolve_kernel(kernel_cfg) ||
                raise "NativeKernel not found"

            assert {:ok, session} = Session.create_with_src(device, kernel.src)

            %Context{
              ctx
              | kernel: kernel,
                session: session
            }
          end
        end
      end
      |> List.flatten()
    end

    def n_items(%Context{shape: s}) do
      case s do
        n when is_integer(n) -> n
        {x, y} -> x * y
      end
    end

    def build_array(%Context{} = tc) do
      Array.filled_with(
        type(tc),
        initial_value(tc),
        n_items(tc)
      )
    end

    def build_device_buffer(%Context{} = tc) do
      assert %DeviceBuffer{} =
               DeviceBuffer.build(
                 session(tc),
                 shape(tc),
                 type(tc),
                 build_array(tc),
                 :read_write
               )
    end

    defp render_report(tc) do
      """
      Ran #{identifier(tc)}
      took: #{took_usec(tc)}usec
      average_usec_per_op: #{average_usec_per_op(tc)}
      estimated_ops_per_sec: #{estimated_ops_per_sec(tc)}

      """
    end

    def report(%Context{} = tc) do
      Logger.debug(render_report(tc))
    end

    defp concurrency(%Context{concurrency: c}) when is_integer(c) and c > 0, do: c
    defp concurrency(%Context{}), do: 1

    defp run_once(tc, buffer) do
      assert :ok =
               Session.kernel_execute_sync(
                 session(tc),
                 kernel_name(tc),
                 shape(tc),
                 [buffer]
               )
    end

    def run(tc, buffer) do
      tc = start(tc)

      1..iterations(tc)
      |> Enum.each(fn _ ->
        run_once(tc, buffer)
      end)

      stop(tc)
    end

    def run_and_report(%Context{} = tc) do
      buffer = build_device_buffer(tc)
      Logger.debug("Starting Context #{identifier(tc)}")
      tc = run(tc, buffer)

      list =
        buffer
        |> DeviceBuffer.to_array()
        |> Array.to_list()

      assert length(list) == n_items(tc)
      expected_value = expected(tc)

      if expected_value do
        Enum.each(list, fn num ->
          assert num === expected_value, """
            Context #{identifier(tc)} failed -
              reason: expected value was incorrect
              expected_value: #{inspect(expected_value)}
              got_value: #{inspect(num)}
          """
        end)
      end

      report(tc)
    end
  end
end
