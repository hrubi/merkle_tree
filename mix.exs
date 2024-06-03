defmodule MerkleTree.MixProject do
  use Mix.Project

  def project do
    [
      app: :merkle_tree,
      version: "0.1.0",
      elixir: "~> 1.16",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.33.0", runtime: false},
      {:benchee, "~> 1.3", only: :dev}
    ]
  end
end
