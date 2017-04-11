defmodule PrimalEx.Mixfile do
  use Mix.Project

  def project do
    [app: :primal_ex,
     version: "0.1.2",
     elixir: "~> 1.4",
     build_embedded: Mix.env == :prod,
     start_permanent: Mix.env == :prod,
     compilers: [:rustler] ++ Mix.compilers(),
     rustler_crates: rustler_crates(),
     description: description(),
     package: package(),
     deps: deps()]
  end

  # Configuration for the OTP application
  #
  # Type "mix help compile.app" for more information
  def application do
    # Specify extra applications you'll use from Erlang/Elixir
    [extra_applications: [:logger]]
  end

  # Dependencies can be Hex packages:
  #
  #   {:my_dep, "~> 0.3.0"}
  #
  # Or git/path repositories:
  #
  #   {:my_dep, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
  #
  # Type "mix help deps" for more examples and options
  defp deps do
    [
      {:benchfella, "~> 0.3.0", only: :dev},
      {:rustler, "~> 0.9.0"},
      {:ex_doc, ">= 0.0.0", only: :dev}
    ]
  end

  defp rustler_crates do
    [native_prime: [
      path: "native/primalex_nativeprime",
      mode: (if Mix.env == :prod, do: :release, else: :debug),
    ]]
  end
  defp description do
    """
    A prime number library for Elixir, providing NIF bindings for the Primal Rust library.
    """
  end
  defp package do
    [
      name: :primal_ex,
      files: ["lib", "native", "priv", "mix.exs", "README*", "LICENSE*"],
      maintainers: ["Dan Dresselhaus"],
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/ddresselhaus/primal_ex"}
    ]
  end
end
