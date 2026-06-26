{% if language == "rust" %}

```bash
cargo add tree-sitter-language-pack
```

{% elif language == "python" %}

```bash
pip install tree-sitter-language-pack
```

{% elif language in ["typescript", "node"] %}

```bash
npm install @xberg-io/tree-sitter-language-pack
```

{% elif language == "wasm" %}

```bash
npm install @xberg-io/tree-sitter-language-pack-wasm
```

{% elif language == "ruby" %}

```bash
gem install tree_sitter_language_pack
```

{% elif language == "php" %}

```bash
composer require xberg-io/tree-sitter-language-pack
```

{% elif language == "go" %}

```bash
go get github.com/xberg-io/tree-sitter-language-pack/packages/go
```

{% elif language == "java" %}

```xml
<dependency>
  <groupId>io.xberg</groupId>
  <artifactId>tree-sitter-language-pack</artifactId>
  <version>{{ version }}</version>
</dependency>
```

{% elif language == "csharp" %}

```bash
dotnet add package TreeSitterLanguagePack
```

{% elif language == "elixir" %}
Add to `mix.exs`:

```elixir
defp deps do
  [
    {:tree_sitter_language_pack, "~> {{ version }}"}
  ]
end
```

{% elif language == "ffi" %}
Download the prebuilt static/dynamic library from the [GitHub releases page](https://github.com/xberg-io/tree-sitter-language-pack/releases) or build from source:

```bash
git clone https://github.com/xberg-io/tree-sitter-language-pack
cargo build --release -p tree-sitter-language-pack-ffi
```

{% endif %}
