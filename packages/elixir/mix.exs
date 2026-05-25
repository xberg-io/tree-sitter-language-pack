defmodule TreeSitterLanguagePack.MixProject do
  use Mix.Project

  def project do
    [
      app: :tree_sitter_language_pack,
      version: "1.9.0-rc.5",
      elixir: "~> 1.14",
      elixirc_paths: ["lib", Path.expand("../../packages/elixir/native/tree_sitter_language_pack_nif/src", __DIR__)],
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
        ~w(.formatter.exs mix.exs README* checksum-*.exs native/tree_sitter_language_pack_nif/Cargo.toml native/tree_sitter_language_pack_nif/Cargo.lock ../../packages/elixir/native/tree_sitter_language_pack_nif/src)
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.37.0", runtime: false},
      {:rustler_precompiled, "~> 0.9"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.40", only: :dev, runtime: false}
    ]
  end
end
