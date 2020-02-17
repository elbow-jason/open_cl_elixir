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
          # flags: ["-g"]
        ]
      ],
      elixirc_paths: elixirc_paths(Mix.env()),
      aliases: aliases(Mix.env())
    ]
  end

  def elixirc_paths(:test), do: ["lib", "test/support"]
  def elixirc_paths(_), do: ["lib"]

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

  defp deps do
    [
      {:rustler, "~> 0.21.0"},
    ]
  end

  @native_manifest_flag "--manifest-path native/open_cl_native/Cargo.toml"

  defp aliases(_) do
    [
      test: ["test.native", "test"],
      format: ["format.native", "format"],
      "test.native": ["cmd cargo test #{@native_manifest_flag} -- --nocapture"],
      "format.native": ["cmd cargo fmt #{@native_manifest_flag}"]
    ]
  end
end
