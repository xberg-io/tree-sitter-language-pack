defmodule Tree_sitter_language_pack.MixProject do
  use Mix.Project

  def project do
    [
      app: :tree_sitter_language_pack,
      version: "1.8.0-rc.25",
      elixir: "~> 1.14",
      rustler_crates: [tree_sitter_language_pack_nif: [mode: :release]],
      description: "Pre-compiled tree-sitter grammars for 305 programming languages",
      package: package(),
      deps: deps()
    ]
  end

  defp package do
    [
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/kreuzberg-dev/tree-sitter-language-pack"},
      files:
        ~w(lib .formatter.exs mix.exs README* checksum-*.exs) ++
          Path.wildcard("native/*/Cargo.toml") ++
          Path.wildcard("native/*/Cargo.lock") ++
          Path.wildcard("native/*/src/**/*") ++
          Path.wildcard("native/*/build.rs") ++
          Path.wildcard("native/*/.cargo/**/*")
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.37.0", optional: true, runtime: false},
      {:rustler_precompiled, "~> 0.9"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.40", only: :dev, runtime: false}
    ]
  end
end
