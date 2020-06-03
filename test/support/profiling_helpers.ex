defmodule OpenCL.ProfilingHelpers do
  require Logger

  @time_unit "μ"

  defguardp is_pos_int(p) when is_integer(p) and p > 0

  def run(session, name, n_sequential, n_concurrency, func) when is_function(func, 1) do
    {took, _} =
      :timer.tc(fn ->
        do_run(n_sequential, n_concurrency, fn -> func.(session) end)
      end)

    report(session, name, n_sequential, n_concurrency, took)
    :ok
  end

  defp do_run(n_sequential, 1, func) when is_pos_int(n_sequential) do
    with_sequential(n_sequential, func)
  end

  defp do_run(n_sequential, n_concurrency, func)
       when is_pos_int(n_sequential) and is_pos_int(n_concurrency) do
    1..n_concurrency
    |> Enum.map(fn _ ->
      Task.async(fn -> with_sequential(n_sequential, func) end)
    end)
    |> Enum.each(fn task -> Task.await(task) end)
  end

  defp with_sequential(1, func) do
    _ = func.()
    :ok
  end

  defp with_sequential(n, func) do
    Enum.each(1..n, fn _ -> func.() end)
  end

  defp report(session, name, n_sequential, n_concurrency, took) do
    total_runs = n_sequential * n_concurrency
    time_per_run = trunc(Float.round(took / total_runs))
    time_unit = "#{@time_unit}sec"
    runs_per_sec = Float.round(1_000_000 / (took / total_runs))

    Logger.debug("""
    Report: #{name} ---
        session: #{inspect(session)}
        n_sequential: #{n_sequential}
        n_concurrency: #{n_concurrency}
        total_time: #{took}#{time_unit}
        approx_time_per_call: #{time_per_run}#{time_unit}
        approx_calls_per_sec: #{runs_per_sec}
    """)
  end
end
