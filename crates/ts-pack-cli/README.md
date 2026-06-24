# tree-sitter-language-pack — CLI

[![Bindings](https://img.shields.io/badge/Bindings-alef%20%D7%90-007ec6)](https://github.com/xberg-io/alef)

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Language Bindings -->  <a href="https://crates.io/crates/tree-sitter-language-pack">
    <img src="https://img.shields.io/crates/v/tree-sitter-language-pack?label=Rust&color=007ec6" alt="Rust">
  </a>  <a href="https://pypi.org/project/tree-sitter-language-pack/">
    <img src="https://img.shields.io/pypi/v/tree-sitter-language-pack?label=Python&color=007ec6" alt="Python">
  </a>  <a href="https://www.npmjs.com/package/@kreuzberg/tree-sitter-language-pack">
    <img src="https://img.shields.io/npm/v/@kreuzberg/tree-sitter-language-pack?label=Node.js&color=007ec6" alt="Node">
  </a>  <a href="https://www.npmjs.com/package/@kreuzberg/tree-sitter-language-pack-wasm">
    <img src="https://img.shields.io/npm/v/@kreuzberg/tree-sitter-language-pack-wasm?label=WASM&color=007ec6" alt="Wasm">
  </a>  <a href="https://central.sonatype.com/artifact/dev.xberg-io/tree-sitter-language-pack">
    <img src="https://img.shields.io/maven-central/v/dev.xberg-io/tree-sitter-language-pack?label=Java&color=007ec6" alt="Java">
  </a>  <a href="https://github.com/xberg-io/tree-sitter-language-pack/tree/main/packages/go">
    <img src="https://img.shields.io/github/v/tag/xberg-io/tree-sitter-language-pack?label=Go&color=007ec6" alt="Go">
  </a>  <a href="https://www.nuget.org/packages/TreeSitterLanguagePack">
    <img src="https://img.shields.io/nuget/v/TreeSitterLanguagePack?label=C%23&color=007ec6" alt="Csharp">
  </a>  <a href="https://packagist.org/packages/xberg-io/tree-sitter-language-pack">
    <img src="https://img.shields.io/packagist/v/xberg-io/tree-sitter-language-pack?label=PHP&color=007ec6" alt="Php">
  </a>  <a href="https://rubygems.org/gems/tree_sitter_language_pack">
    <img src="https://img.shields.io/gem/v/tree_sitter_language_pack?label=Ruby&color=007ec6" alt="Ruby">
  </a>  <a href="https://hex.pm/packages/tree_sitter_language_pack">
    <img src="https://img.shields.io/hexpm/v/tree_sitter_language_pack?label=Elixir&color=007ec6" alt="Elixir">
  </a>  <a href="https://github.com/xberg-io/tree-sitter-language-pack/tree/main/crates/ts-pack-core-ffi">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="Ffi">
  </a>
  <!-- Project Info -->
  <a href="https://github.com/xberg-io/tree-sitter-language-pack/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/xberg-io/tree-sitter-language-pack/ci-rust.yaml?branch=main&label=CI&color=007ec6" alt="CI">
  </a>
  <a href="https://github.com/xberg-io/tree-sitter-language-pack/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-007ec6.svg" alt="License">
  </a>
  <a href="https://github.com/xberg-io/homebrew-tap">
    <img src="https://img.shields.io/badge/homebrew-ts--pack-007ec6?logo=homebrew" alt="Homebrew">
  </a>
  <a href="https://docs.tree-sitter-language-pack.xberg.io">
    <img src="https://img.shields.io/badge/Docs-tree--sitter--language--pack-007ec6" alt="Docs">
  </a>
</div>

<div align="center" style="margin: 24px 0 0;">
  <a href="https://xberg.io">
    <img alt="tree-sitter-language-pack" src="https://github.com/user-attachments/assets/478a83da-237b-446b-b3a8-e564c13e00a8" />
  </a>
</div>

<div align="center" style="display: flex; flex-wrap: wrap; gap: 12px; justify-content: center; margin: 28px 0 24px;">
  <a href="https://discord.gg/xt9WY3GnKR">
    <img height="22" src="https://img.shields.io/badge/Discord-Chat-007ec6?logo=discord&logoColor=white" alt="Join Discord">
  </a>
</div>

Command-line tool for managing tree-sitter language parsers with download utilities.

## Installation

```sh
cargo install ts-pack-cli
```

Or install via Homebrew:

Homebrew 6.0+ requires explicit trust for third-party taps. Trust once, then install:

```sh
brew trust xberg-io/tap
brew install xberg-io/tap/ts-pack
```

## Quick Start

```sh
# Initialize a language-pack.toml config
ts-pack init

# List available languages
ts-pack list

# Download specific languages for offline use
ts-pack download python rust javascript

# Check download status
ts-pack status

# Add languages to your config
ts-pack add python rust javascript

# Clone parser sources
ts-pack clone
```

## API Reference

### Commands

- `ts-pack init` -- initialize a `language-pack.toml` config in the current directory
- `ts-pack list` -- list all available language names
- `ts-pack download <lang>...` -- download specific parsers for offline use
- `ts-pack status` -- show download status of all configured languages
- `ts-pack add <lang>...` -- add languages to `language-pack.toml`
- `ts-pack clone` -- clone parser source repositories (for development)
- `ts-pack parse <file>` -- parse a file and print the syntax tree
- `ts-pack process <file>` -- extract code intelligence from a file

### Global Flags

- `--config <path>` -- path to `language-pack.toml` (default: current directory)
- `--verbose` / `-v` -- enable verbose output
- `--help` -- show help for any command

## MCP Server

The CLI includes an MCP server for integration with AI agents and IDEs. Start it with:

```sh
ts-pack mcp
```

The server runs over stdio by default. For HTTP transport (remote agents or team setups):

```sh
ts-pack mcp --transport http --host 127.0.0.1 --port 8011
```

### Options

- `--transport <stdio|http>` -- transport mode (default: `stdio`)
- `--host <addr>` -- bind address for HTTP transport (default: `127.0.0.1`)
- `--port <u16>` -- port for HTTP transport (default: `8011`)
- `--config <path>` -- optional language-pack.toml config file

### Tools

The MCP server exposes 8 tools:

- **`parse`** — render syntax tree as S-expression or JSON
- **`process`** — extract code intelligence (structure, imports, exports, symbols, docstrings, comments, diagnostics, chunking)
- **`detect_language`** — identify language from file path or source code
- **`list_languages`** — enumerate available, downloaded, or manifest languages with optional filtering
- **`info`** — get status of a specific language (available, downloaded, etc.)
- **`download`** — fetch parsers (list, groups, all, or fresh)
- **`cache_dir`** — retrieve the local cache directory
- **`clean_cache`** — delete cached parsers

### Resources

- `ts-pack://languages` — available language catalog
- `ts-pack://languages/downloaded` — user-downloaded parsers
- `ts-pack://language/{name}` — per-language status (template resource)

### Prompt

- **`analyze-code`** — analyze source code with language detection and code intelligence extraction

For detailed setup and IDE configuration, see the [MCP Server guide](https://docs.tree-sitter-language-pack.xberg.io/guides/mcp-server/).

For full documentation, see [xberg.io](https://docs.tree-sitter-language-pack.xberg.io).

## License

MIT -- see [LICENSE](https://github.com/xberg-io/tree-sitter-language-pack/blob/main/LICENSE) for details.

---

Part of [tree-sitter-language-pack](https://github.com/xberg-io/tree-sitter-language-pack) -- A comprehensive collection of tree-sitter language parsers with polyglot bindings.

## Part of Kreuzberg, Inc

- [Kreuzberg](https://docs.xberg.io) — document intelligence: text, tables, metadata from 90+ formats with optional OCR.
- [Kreuzberg Cloud](https://docs.kreuzberg.cloud) — managed extraction API with SDKs, dashboards, and observability.
- [kreuzcrawl](https://docs.kreuzcrawl.xberg.io) — web crawling and scraping with HTML→Markdown and headless-Chrome fallback.
- [html-to-markdown](https://docs.html-to-markdown.xberg.io) — fast, lossless HTML→Markdown engine.
- [liter-llm](https://docs.liter-llm.xberg.io) — universal LLM API client with native bindings for 14 languages and 143 providers.
- [Discord](https://discord.gg/xt9WY3GnKR) — community, roadmap, announcements.
