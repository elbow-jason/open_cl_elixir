defmodule OpenCL.MixProject do
  use Mix.Project

  def project do
    [
      app: :open_cl,
      version: "0.1.0",
      elixir: "~> 1.8",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: [
        open_cl_native: [
          mode: rustc_mode(Mix.env())
        ]
      ]
    ]
  end

  def rustc_mode(:prod), do: :release
  def rustc_mode(:test), do: :debug
  def rustc_mode(:dev), do: :debug
  def rustc_mode(_), do: :release

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
      mod: {OpenCL.Application, []}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.21.0"}
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
    ]
  end
end
