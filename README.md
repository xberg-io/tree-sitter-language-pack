# tree-sitter-language-pack

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0">
	<a href="https://github.com/xberg-io/alef">
		<img src="https://img.shields.io/badge/Bindings-alef%20%D7%90-007ec6" alt="Bindings" />
	</a>
	<!-- Language Bindings -->
	<a href="https://crates.io/crates/tree-sitter-language-pack">
		<img src="https://img.shields.io/crates/v/tree-sitter-language-pack?label=Rust&color=007ec6" alt="Rust" />
	</a>
	<a href="https://pypi.org/project/tree-sitter-language-pack/">
		<img src="https://img.shields.io/pypi/v/tree-sitter-language-pack?label=Python&color=007ec6" alt="Python" />
	</a>
	<a href="https://www.npmjs.com/package/@kreuzberg/tree-sitter-language-pack">
		<img
			src="https://img.shields.io/npm/v/@kreuzberg/tree-sitter-language-pack?label=Node.js&color=007ec6"
			alt="Node.js"
		/>
	</a>
	<a href="https://www.npmjs.com/package/@kreuzberg/tree-sitter-language-pack-wasm">
		<img
			src="https://img.shields.io/npm/v/@kreuzberg/tree-sitter-language-pack-wasm?label=WASM&color=007ec6"
			alt="WASM"
		/>
	</a>
	<a href="https://central.sonatype.com/artifact/dev.kreuzberg.treesitterlanguagepack/tree-sitter-language-pack">
		<img
			src="https://img.shields.io/maven-central/v/dev.kreuzberg.treesitterlanguagepack/tree-sitter-language-pack?label=Java&color=007ec6"
			alt="Java"
		/>
	</a>
	<a href="https://pkg.go.dev/github.com/xberg-io/tree-sitter-language-pack/packages/go">
		<img
			src="https://img.shields.io/github/v/tag/xberg-io/tree-sitter-language-pack?label=Go&color=007ec6"
			alt="Go"
		/>
	</a>
	<a href="https://www.nuget.org/packages/TreeSitterLanguagePack/">
		<img src="https://img.shields.io/nuget/v/TreeSitterLanguagePack?label=C%23&color=007ec6" alt="C#" />
	</a>
	<a href="https://packagist.org/packages/xberg-io/tree-sitter-language-pack">
		<img
			src="https://img.shields.io/packagist/v/xberg-io/tree-sitter-language-pack?label=PHP&color=007ec6"
			alt="PHP"
		/>
	</a>
	<a href="https://rubygems.org/gems/tree_sitter_language_pack">
		<img src="https://img.shields.io/gem/v/tree_sitter_language_pack?label=Ruby&color=007ec6" alt="Ruby" />
	</a>
	<a href="https://hex.pm/packages/tree_sitter_language_pack">
		<img src="https://img.shields.io/hexpm/v/tree_sitter_language_pack?label=Elixir&color=007ec6" alt="Elixir" />
	</a>
	<a href="https://pub.dev/packages/tree_sitter_language_pack">
		<img src="https://img.shields.io/pub/v/tree_sitter_language_pack?label=Dart&color=007ec6" alt="Dart" />
	</a>
	<a href="https://central.sonatype.com/artifact/dev.kreuzberg.tslp.android/tree-sitter-language-pack-android">
		<img
			src="https://img.shields.io/maven-central/v/dev.kreuzberg.tslp.android/tree-sitter-language-pack-android?label=Kotlin&color=007ec6"
			alt="Kotlin"
		/>
	</a>
	<a href="https://github.com/xberg-io/tree-sitter-language-pack/tree/main/packages/swift">
		<img src="https://img.shields.io/badge/Swift-SPM-007ec6" alt="Swift" />
	</a>
	<a href="https://github.com/xberg-io/tree-sitter-language-pack/tree/main/packages/zig">
		<img src="https://img.shields.io/badge/Zig-package-007ec6" alt="Zig" />
	</a>
	<a href="https://github.com/xberg-io/tree-sitter-language-pack/releases">
		<img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C FFI" />
	</a>

	<!-- Project Info -->
	<a href="https://github.com/xberg-io/tree-sitter-language-pack/blob/main/LICENSE">
		<img src="https://img.shields.io/badge/License-MIT-007ec6" alt="License" />
	</a>
	<a href="https://docs.tree-sitter-language-pack.xberg.io">
		<img src="https://img.shields.io/badge/Docs-tree--sitter--language--pack-007ec6" alt="Documentation" />
	</a>
</div>

<div align="center" style="display: flex; flex-wrap: wrap; gap: 12px; justify-content: center; margin: 28px 0 24px">
	<a href="https://discord.gg/xt9WY3GnKR">
		<img
			height="22"
			src="https://img.shields.io/badge/Discord-Chat-007ec6?logo=discord&logoColor=white"
			alt="Join Discord"
		/>
	</a>
</div>

A comprehensive collection of tree-sitter language parsers with polyglot bindings — 306 languages, one Rust core, downloaded on demand.

## What and Why?

[tree-sitter](https://tree-sitter.github.io/tree-sitter/) generates fast, incremental parsers from grammars for any programming language. As agentic tooling makes processing, inspecting, and analyzing code ever more critical, tree-sitter-language-pack bundles the most comprehensive collection of grammars available behind a single API.

The core is written in Rust with polyglot bindings for 15 other languages, plus a CLI and MCP server for working with code from the shell. Grammars are built into multi-platform binaries and downloaded on demand, so the package stays small while offering 300+ parsers.

### Features

| Feature | Description |
| ------- | ----------- |
| **306 languages** | Pre-compiled parsers at ABI 14 (backwards compatible with tree-sitter 0.21–0.26) |
| **Code intelligence** | Extract functions, classes, imports, docstrings, and symbols from source |
| **Data extraction** | Hierarchical key-value trees from 17 config/data formats (JSON, YAML, TOML, XML, CSV, …) |
| **Host-native language API** | `get_language()` returns native `Language` objects in Python, Node.js, Go, Java, C#, Kotlin, Swift, Zig, and C |
| **On-demand downloads** | Parsers are fetched on first use and cached locally for fast, offline reuse |
| **Selective installation** | Download only the languages you need; unused parsers are never downloaded |
| **Polyglot bindings** | Native bindings across 15 languages plus a C ABI for everything else |
| **CLI & MCP server** | `ts-pack download` to pre-fetch parsers; MCP integration for AI agents |

### Supported Languages

This pack includes 306 languages. See the [full language list](docs/languages.md) for every supported grammar with extensions and repository links.

<div align="center">
  <a href="https://github.com/xberg-io/tree-sitter-language-pack/stargazers">
    <img src="docs/assets/star.gif" alt="Star tree-sitter-language-pack on GitHub" width="640">
  </a>
</div>

<p align="center"><strong>⭐ Star this repo to show your support — it helps others discover tree-sitter-language-pack.</strong></p>

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
npm install @kreuzberg/tree-sitter-language-pack
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

Available on Maven Central as `dev.kreuzberg.treesitterlanguagepack:tree-sitter-language-pack`. See [Java README](packages/java/README.md) for the dependency snippet and current version.

</details>

<details>
<summary><strong>C#</strong></summary>

```sh
dotnet add package TreeSitterLanguagePack
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
npm install @kreuzberg/tree-sitter-language-pack-wasm
```

See [WebAssembly README](crates/ts-pack-core-wasm/README.md) for full documentation.

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
brew install xberg-io/tap/ts-pack
```

Or run without a persistent install (the proxy package fetches the native binary):

```sh
npx @kreuzberg/ts-pack-cli parse <file>
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
claude mcp add ts-pack -- ts-pack mcp
```

Or add to your Claude Desktop config at `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "ts-pack": {
      "command": "ts-pack",
      "args": ["mcp"]
    }
  }
}
```

The MCP server exposes 8 tools: `parse`, `process`, `detect_language`, `list_languages`, `info`, `download`, `cache_dir`, and `clean_cache`. It also provides resources for the available language catalog and a prompt for code analysis.

The marketplace plugin from [`xberg-io/plugins`](https://github.com/xberg-io/plugins) auto-registers the server — see [AI Coding Assistants](#ai-coding-assistants) below to install it instead of manual registration.

For detailed setup, transport options, and tool reference, see the [MCP Server guide](https://docs.tree-sitter-language-pack.xberg.io/guides/mcp-server/).

</details>

### AI Coding Assistants

Install the tree-sitter-language-pack plugin from the [`xberg-io/plugins`](https://github.com/xberg-io/plugins) marketplace. It ships the tree-sitter-language-pack agent skills (parse and extract code intelligence from 300+ languages) and works with every major coding agent — expand your harness below.

<details open>
<summary><strong>Claude Code</strong></summary>

```text
/plugin marketplace add xberg-io/plugins
/plugin install tree-sitter-language-pack@kreuzberg
```

</details>

<details>
<summary><strong>Codex CLI</strong></summary>

```text
/plugins add https://github.com/xberg-io/plugins
```

Then search for `tree-sitter-language-pack` and select **Install Plugin**.

</details>

<details>
<summary><strong>Cursor</strong></summary>

Settings → Plugins → Add from URL → `https://github.com/xberg-io/plugins`, then select **tree-sitter-language-pack**.

</details>

<details>
<summary><strong>Gemini CLI</strong></summary>

```text
gemini extensions install https://github.com/xberg-io/plugins
```

</details>

<details>
<summary><strong>Factory Droid</strong></summary>

```text
droid plugin marketplace add https://github.com/xberg-io/plugins
droid plugin install tree-sitter-language-pack@kreuzberg
```

</details>

<details>
<summary><strong>GitHub Copilot CLI</strong></summary>

```text
copilot plugin marketplace add https://github.com/xberg-io/plugins
copilot plugin install tree-sitter-language-pack@kreuzberg
```

</details>

<details>
<summary><strong>opencode</strong></summary>

Add the package to `opencode.json`:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "plugin": ["@kreuzberg/opencode-tree-sitter-language-pack"]
}
```

</details>

## Documentation

Full guides, the host-native language API, data extraction, the CLI and MCP server, and the complete language list live at **[docs.tree-sitter-language-pack.xberg.io](https://docs.tree-sitter-language-pack.xberg.io)**.

## Part of Kreuzberg.dev

- [Kreuzberg](https://github.com/xberg-io/kreuzberg) — document intelligence: text, tables, metadata from 91+ formats with optional OCR.
- [Xberg Enterprise](https://github.com/xberg-io/xberg-enterprise) — managed extraction API with SDKs, dashboards, and observability.
- [crawlberg](https://github.com/xberg-io/crawlberg) — web crawling and scraping with HTML→Markdown and headless-Chrome fallback.
- [html-to-markdown](https://github.com/xberg-io/html-to-markdown) — fast, lossless HTML→Markdown engine.
- [liter-llm](https://github.com/xberg-io/liter-llm) — universal LLM API client with native bindings for 14 languages and 143 providers.
- [tree-sitter-language-pack](https://github.com/xberg-io/tree-sitter-language-pack) — tree-sitter grammars and code-intelligence primitives.
- [alef](https://github.com/xberg-io/alef) — the polyglot binding generator that produces every per-language binding across the 5 polyglot repos.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Join our [Discord community](https://discord.gg/xt9WY3GnKR) for questions and discussion.

## License

MIT — see [LICENSE](LICENSE) for details.

All included tree-sitter grammars are permissively licensed (MIT, Apache-2.0, BSD, ISC, or similar). Copyleft licenses (GPL, AGPL, LGPL, MPL) are not accepted. See [CONTRIBUTING.md](CONTRIBUTING.md) for grammar inclusion criteria.
