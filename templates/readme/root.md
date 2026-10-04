<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://cdn.jsdelivr.net/gh/xberg-io/assets@v1/banner/readme-banner-dark.svg">
    <img alt="Xberg" width="420" src="https://cdn.jsdelivr.net/gh/xberg-io/assets@v1/banner/readme-banner-light.svg">
  </picture>
</p>

# tree-sitter-language-pack

{% include 'partials/badges.html' %}

Parse and understand source code in 372 languages, from the language you already work in — with one dependency and no grammars to compile.

## What and Why?

Add one package and you can parse any of 372 languages, walk their syntax trees, pull out functions, classes, imports, and symbols, and split code into chunks an LLM can use. It works the same whether you call it from Python, Node.js, Go, Java, C#, Ruby, PHP, Elixir, and eight more, or from the shell via the CLI and MCP server.

[tree-sitter](https://tree-sitter.github.io/tree-sitter/) gives fast, incremental parsers for individual languages, but wiring up hundreds of grammars — and reaching them from a non-C ecosystem — is the hard part. tree-sitter-language-pack does that work for you: it bundles the most comprehensive set of grammars available behind a single API, ships native bindings for 15 languages, and downloads each parser on first use so the install stays small.

Reach for it whenever you need to process, inspect, or analyze code — building developer tooling, feeding a RAG pipeline, or giving an agent structural understanding of a codebase.

### Features

| Feature | Description |
| ------- | ----------- |
| **372 languages** | Pre-compiled parsers at ABI 14 (backwards compatible with tree-sitter 0.21–0.26) |
| **Code intelligence** | Extract functions, classes, imports, docstrings, and symbols from source |
| **Data extraction** | Hierarchical key-value trees from 17 config/data formats (JSON, YAML, TOML, XML, CSV, …) |
| **Host-native language API** | `get_language()` returns native `Language` objects in Python, Node.js, Go, Java, C#, Kotlin, Swift, Zig, and C |
| **On-demand downloads** | Parsers are fetched on first use and cached locally for fast, offline reuse |
| **Prefetch & warming** | `prefetch()` loads (and downloads) every grammar you need up front, so hot loops only parse |
| **Bundled queries** | `highlights`, `injections`, `locals`, `tags`, `indents`, and `folds` `.scm` queries per language, with a process-wide compiled-query cache (Rust) |
| **Selective installation** | Download only the languages you need; unused parsers are never downloaded |
| **Polyglot bindings** | Native bindings across 15 languages, including a C ABI for everything else |
| **CLI & MCP server** | `ts-pack download` to pre-fetch parsers; MCP integration for AI agents |

### Supported Languages

This pack includes 372 languages. See the [full language list](https://docs.tree-sitter-language-pack.xberg.io/languages/) for every supported grammar with extensions and repository links.

### Grammars & Bundled Queries

Each grammar bundles a subset of the six standard tree-sitter query types. The table below shows which `.scm` queries ship with every grammar.

{% include 'partials/grammar_table.md' %}

<p align="center">
  <a href="https://github.com/xberg-io/tree-sitter-language-pack/stargazers"><strong>⭐ Star this repo to show your support — it helps others discover tree-sitter-language-pack.</strong></a>
</p>

## Quick Start

### Language Packages

<details open>
<summary><strong>Rust</strong></summary>

```sh
cargo add tree-sitter-language-pack
```

See [Rust README](crates/ts-pack-core/README.md) for full documentation.

</details>

<details>
<summary><strong>Python</strong></summary>

```sh
pip install tree-sitter-language-pack
```

See [Python README](packages/python/README.md) for full documentation.

</details>

<details>
<summary><strong>Node.js</strong></summary>

```sh
npm install @xberg-io/tree-sitter-language-pack
```

See [Node.js README](crates/ts-pack-core-node/README.md) for full documentation.

</details>

<details>
<summary><strong>Go</strong></summary>

```sh
go get github.com/xberg-io/tree-sitter-language-pack/packages/go
```

See [Go README](packages/go/README.md) for full documentation.

</details>

<details>
<summary><strong>Java</strong></summary>

Available on Maven Central as `io.xberg.treesitterlanguagepack:tree-sitter-language-pack`. See [Java README](packages/java/README.md) for the dependency snippet and current version.

</details>

<details>
<summary><strong>C#</strong></summary>

```sh
dotnet add package XbergIo.TreeSitterLanguagePack
```

See [.NET README](packages/csharp/README.md) for full documentation.

</details>

<details>
<summary><strong>Ruby</strong></summary>

```sh
gem install tree_sitter_language_pack
```

See [Ruby README](packages/ruby/README.md) for full documentation.

</details>

<details>
<summary><strong>PHP</strong></summary>

```sh
composer require xberg-io/tree-sitter-language-pack
```

See [PHP README](packages/php/README.md) for full documentation.

</details>

<details>
<summary><strong>Elixir</strong></summary>

Add `{:tree_sitter_language_pack, "~> 1.0"}` to your `mix.exs` dependencies. See [Elixir README](packages/elixir/README.md) for full documentation.

</details>

<details>
<summary><strong>WebAssembly</strong></summary>

```sh
npm install @xberg-io/tree-sitter-language-pack-wasm
```

See [WebAssembly README](crates/ts-pack-core-wasm/README.md) for full documentation.

</details>

<details>
<summary><strong>Dart / Flutter</strong></summary>

```sh
dart pub add tree_sitter_language_pack
```

See [Dart README](packages/dart/README.md) for full documentation.

</details>

<details>
<summary><strong>Kotlin (Android)</strong></summary>

Available on Maven Central as `io.xberg.tslp.android:tree-sitter-language-pack-android`. See [Kotlin Android README](packages/kotlin-android/README.md) for the dependency snippet and current version.

</details>

<details>
<summary><strong>Swift</strong></summary>

Available via Swift Package Manager. See [Swift README](packages/swift/README.md) for the SwiftPM package URL and current version.

</details>

<details>
<summary><strong>Zig</strong></summary>

```sh
zig fetch --save <release tarball url>
```

See [Zig README](packages/zig/README.md) for full documentation.

</details>

<details>
<summary><strong>C/C++ (FFI)</strong></summary>

Build from source as part of this workspace. See [FFI README](crates/ts-pack-core-ffi/README.md) for full documentation.

</details>

<details>
<summary><strong>CLI</strong></summary>

```sh
cargo install ts-pack-cli
```

```sh
# Or install the prebuilt binary via cargo-binstall:
cargo binstall ts-pack-cli
```

```sh
brew trust xberg-io/tap
brew install xberg-io/tap/ts-pack
```

Windows users can install the same binary through [Scoop](https://scoop.sh):

```powershell
scoop bucket add xberg https://github.com/xberg-io/scoop-bucket
scoop install ts-pack
```

Or run without a persistent install (the proxy package fetches the native binary):

```sh
npx @xberg-io/ts-pack-cli parse <file>
uvx --from ts-pack-cli ts-pack parse <file>
```

See [CLI README](crates/ts-pack-cli/README.md) for full documentation.

</details>

<details>
<summary><strong>MCP Server</strong></summary>

The CLI bundles an MCP server for integration with AI agents. Start it with:

```sh
ts-pack mcp
```

The server runs over stdio by default. For HTTP transport:

```sh
ts-pack mcp --transport http --host 127.0.0.1 --port 8011
```

Register with Claude Code:

```sh
claude mcp add tree-sitter-language-pack -- ts-pack mcp --transport stdio
```

Or add to your Claude Desktop config at `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "tree-sitter-language-pack": {
      "command": "ts-pack",
      "args": ["mcp", "--transport", "stdio"]
    }
  }
}
```

The MCP server exposes 8 tools: `parse`, `process`, `detect_language`, `list_languages`, `info`, `download`, `cache_dir`, and `clean_cache`. It also provides resources for the available language catalog and a prompt for code analysis.

The marketplace plugin from [`xberg-io/tree-sitter-language-pack`](https://github.com/xberg-io/tree-sitter-language-pack) auto-registers the server — see [AI Coding Assistants](#ai-coding-assistants) below to install it instead of manual registration.

For detailed setup, transport options, and tool reference, see the [MCP Server guide](https://docs.tree-sitter-language-pack.xberg.io/guides/mcp-server/).

</details>

### AI Coding Assistants

Install the tree-sitter-language-pack plugin from [`xberg-io/tree-sitter-language-pack`](https://github.com/xberg-io/tree-sitter-language-pack). It ships the tree-sitter-language-pack agent skills (parse and extract code intelligence from 372 languages) and works with every major coding agent — expand your harness below.

<details open>
<summary><strong>Claude Code</strong></summary>

```text
/plugin marketplace add xberg-io/tree-sitter-language-pack
/plugin install tree-sitter-language-pack@tree-sitter-language-pack
```

</details>

<details>
<summary><strong>Codex CLI</strong></summary>

```text
/plugins add https://github.com/xberg-io/tree-sitter-language-pack
```

Then search for `tree-sitter-language-pack` and select **Install Plugin**.

</details>

<details>
<summary><strong>Cursor</strong></summary>

Settings → Plugins → Add from URL → `https://github.com/xberg-io/tree-sitter-language-pack`, then select **tree-sitter-language-pack**.

</details>

<details>
<summary><strong>Gemini CLI</strong></summary>

```text
gemini extensions install https://github.com/xberg-io/tree-sitter-language-pack
```

</details>

<details>
<summary><strong>Factory Droid</strong></summary>

```text
droid plugin marketplace add https://github.com/xberg-io/tree-sitter-language-pack
droid plugin install tree-sitter-language-pack@tree-sitter-language-pack
```

</details>

<details>
<summary><strong>GitHub Copilot CLI</strong></summary>

```text
copilot plugin marketplace add https://github.com/xberg-io/tree-sitter-language-pack
copilot plugin install tree-sitter-language-pack@tree-sitter-language-pack
```

</details>

<details>
<summary><strong>opencode</strong></summary>

Add the package to `opencode.json`:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "plugin": ["@xberg-io/opencode-tree-sitter-language-pack"]
}
```

</details>

## Documentation

Full guides, the host-native language API, data extraction, the CLI and MCP server, and the complete language list live at **[docs.tree-sitter-language-pack.xberg.io](https://docs.tree-sitter-language-pack.xberg.io)**.

## Part of Xberg

- [Xberg](https://github.com/xberg-io/xberg) — document intelligence: text, tables, metadata from 101 formats with optional OCR.
- [Xberg Enterprise](https://github.com/xberg-io/xberg-enterprise) — managed extraction API with SDKs, dashboards, and observability.
- [crawlberg](https://github.com/xberg-io/crawlberg) — web crawling and scraping with HTML→Markdown and headless-Chrome fallback.
- [html-to-markdown](https://github.com/xberg-io/html-to-markdown) — fast, lossless HTML→Markdown engine.
- [liter-llm](https://github.com/xberg-io/liter-llm) — universal LLM API client with native bindings for 14 languages and 165 providers.
- [tree-sitter-language-pack](https://github.com/xberg-io/tree-sitter-language-pack) — tree-sitter grammars and code-intelligence primitives.
- [alef](https://github.com/xberg-io/alef) — the polyglot binding generator that produces every per-language binding across the 5 polyglot repos.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Join our [Discord community](https://discord.gg/xt9WY3GnKR) for questions and discussion.

## License

MIT — see [LICENSE](LICENSE) for details.

All included tree-sitter grammars are permissively licensed (MIT, Apache-2.0, BSD, ISC, or similar). Copyleft licenses (GPL, AGPL, LGPL, MPL) are not accepted. See [CONTRIBUTING.md](CONTRIBUTING.md) for grammar inclusion criteria.
