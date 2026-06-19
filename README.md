# tree-sitter-language-pack

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <a href="https://github.com/kreuzberg-dev/alef">
    <img src="https://img.shields.io/badge/Bindings-alef%20%D7%90-007ec6" alt="Bindings">
  </a>
  <!-- Language Bindings -->
  <a href="https://crates.io/crates/tree-sitter-language-pack">
    <img src="https://img.shields.io/crates/v/tree-sitter-language-pack?label=Rust&color=007ec6" alt="Rust">
  </a>
  <a href="https://pypi.org/project/tree-sitter-language-pack/">
    <img src="https://img.shields.io/pypi/v/tree-sitter-language-pack?label=Python&color=007ec6" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg-dev/tree-sitter-language-pack">
    <img src="https://img.shields.io/npm/v/@kreuzberg-dev/tree-sitter-language-pack?label=Node.js&color=007ec6" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg-dev/tree-sitter-language-pack-wasm">
    <img src="https://img.shields.io/npm/v/@kreuzberg-dev/tree-sitter-language-pack-wasm?label=WASM&color=007ec6" alt="WASM">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg.treesitterlanguagepack/tree-sitter-language-pack">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg.treesitterlanguagepack/tree-sitter-language-pack?label=Java&color=007ec6" alt="Java">
  </a>
  <a href="https://pkg.go.dev/github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go">
    <img src="https://img.shields.io/github/v/tag/kreuzberg-dev/tree-sitter-language-pack?label=Go&color=007ec6" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/TreeSitterLanguagePack/">
    <img src="https://img.shields.io/nuget/v/TreeSitterLanguagePack?label=C%23&color=007ec6" alt="C#">
  </a>
  <a href="https://packagist.org/packages/kreuzberg-dev/tree-sitter-language-pack">
    <img src="https://img.shields.io/packagist/v/kreuzberg-dev/tree-sitter-language-pack?label=PHP&color=007ec6" alt="PHP">
  </a>
  <a href="https://rubygems.org/gems/tree_sitter_language_pack">
    <img src="https://img.shields.io/gem/v/tree_sitter_language_pack?label=Ruby&color=007ec6" alt="Ruby">
  </a>
  <a href="https://hex.pm/packages/tree_sitter_language_pack">
    <img src="https://img.shields.io/hexpm/v/tree_sitter_language_pack?label=Elixir&color=007ec6" alt="Elixir">
  </a>
  <a href="https://pub.dev/packages/tree_sitter_language_pack">
    <img src="https://img.shields.io/pub/v/tree_sitter_language_pack?label=Dart&color=007ec6" alt="Dart">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg.tslp.android/tree-sitter-language-pack-android">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg.tslp.android/tree-sitter-language-pack-android?label=Kotlin&color=007ec6" alt="Kotlin">
  </a>
  <a href="https://github.com/kreuzberg-dev/tree-sitter-language-pack/tree/main/packages/swift">
    <img src="https://img.shields.io/badge/Swift-SPM-007ec6" alt="Swift">
  </a>
  <a href="https://github.com/kreuzberg-dev/tree-sitter-language-pack/tree/main/packages/zig">
    <img src="https://img.shields.io/badge/Zig-package-007ec6" alt="Zig">
  </a>
  <a href="https://github.com/kreuzberg-dev/tree-sitter-language-pack/releases">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C FFI">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/kreuzberg-dev/tree-sitter-language-pack/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-007ec6" alt="License">
  </a>
  <a href="https://docs.tree-sitter-language-pack.kreuzberg.dev">
    <img src="https://img.shields.io/badge/Docs-tree--sitter--language--pack-007ec6" alt="Documentation">
  </a>
</div>

<div align="center" style="margin: 24px 0 0;">
  <a href="https://kreuzberg.dev">
    <img alt="tree-sitter-language-pack" src="https://github.com/user-attachments/assets/478a83da-237b-446b-b3a8-e564c13e00a8" />
  </a>
</div>

<div align="center" style="display: flex; flex-wrap: wrap; gap: 12px; justify-content: center; margin: 28px 0 24px;">
  <a href="https://discord.gg/xt9WY3GnKR">
    <img height="22" src="https://img.shields.io/badge/Discord-Chat-007ec6?logo=discord&logoColor=white" alt="Join Discord">
  </a>
</div>

A comprehensive collection of tree-sitter language parsers with polyglot bindings

## Overview

**tree-sitter-language-pack** bundles 306 tree-sitter language parsers into a single package with native bindings across 15 languages. Ship syntax analysis in your application without managing individual parser dependencies.

## Architecture

```text
tree-sitter-language-pack/
├── crates/
│   ├── ts-pack-core/       # Rust core library
│   ├── ts-pack-python/     # Python (maturin/PyO3) bindings
│   ├── ts-pack-node/       # Node.js (NAPI-RS) bindings
│   ├── ts-pack-java/       # Java (Panama FFI) bindings
│   ├── ts-pack-elixir/     # Elixir (Rustler NIF) bindings
│   ├── ts-pack-ffi/        # C-compatible FFI library
│   └── ts-pack-cli/        # CLI tool
├── packages/
│   └── go/v1/              # Go (cgo) bindings
├── grammars/               # Tree-sitter grammar sources
└── scripts/                # Build and maintenance scripts
```

## Quick Start

### Rust

```sh
cargo add tree-sitter-language-pack
```

See [Rust README](crates/ts-pack-core/README.md) for full documentation.

### Python

```sh
pip install tree-sitter-language-pack
```

```sh
uv add tree-sitter-language-pack
```

See [Python README](packages/python/README.md) for full documentation.

### Node.js

```sh
npm install @kreuzberg-dev/tree-sitter-language-pack
```

```sh
pnpm add @kreuzberg-dev/tree-sitter-language-pack
```

```sh
yarn add @kreuzberg-dev/tree-sitter-language-pack
```

See [Node.js README](crates/ts-pack-core-node/README.md) for full documentation.

### Go

```sh
go get github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go
```

See [Go README](packages/go/README.md) for full documentation.

### Java

```xml
<dependency>
  <groupId>dev.kreuzberg.treesitterlanguagepack</groupId>
  <artifactId>tree-sitter-language-pack</artifactId>
  <version>1.8.0</version>
</dependency>
```

```groovy
implementation("dev.kreuzberg.treesitterlanguagepack:tree-sitter-language-pack:1.8.0")
```

See [Java README](packages/java/README.md) for full documentation.

### Elixir

```elixir
{:tree_sitter_language_pack, "~> 1.0"}
```

See [Elixir README](packages/elixir/README.md) for full documentation.

### Ruby

```sh
gem install tree_sitter_language_pack
```

See [Ruby README](packages/ruby/README.md) for full documentation.

### WebAssembly

```sh
npm install @kreuzberg-dev/tree-sitter-language-pack-wasm
```

```sh
pnpm add @kreuzberg-dev/tree-sitter-language-pack-wasm
```

```sh
yarn add @kreuzberg-dev/tree-sitter-language-pack-wasm
```

See [WebAssembly README](crates/ts-pack-core-wasm/README.md) for full documentation.

### PHP

```sh
composer require kreuzberg-dev/tree-sitter-language-pack
```

See [PHP README](packages/php/README.md) for full documentation.

### .NET (C#)

```sh
dotnet add package TreeSitterLanguagePack
```

See [.NET (C#) README](packages/csharp/README.md) for full documentation.

### C/C++ (FFI)

Build from source as part of this workspace.

See [C/C++ (FFI) README](crates/ts-pack-core-ffi/README.md) for full documentation.

### CLI

```sh
cargo install ts-pack-cli
```

See [CLI README](crates/ts-pack-cli/README.md) for full documentation.

## Core API

All bindings expose a unified `process()` function for extracting structured intelligence from source code:

| Language | Function                                             |
| -------- | ---------------------------------------------------- |
| Rust     | `ts_pack_core::process(source, &config)`             |
| Python   | `process(source, ProcessConfig(...))`                |
| Node.js  | `process(source, { language: '...' })`               |
| Go       | `registry.Process(source, config)`                   |
| Java     | `registry.process(source, configJson)`               |
| Ruby     | `TreeSitterLanguagePack.process(source, configJson)` |
| Elixir   | `TreeSitterLanguagePack.process(source, configJson)` |
| WASM     | `process(source, { language: '...' })`               |
| C FFI    | `ts_pack_process(registry, source, len, configJson)` |

The `process()` function returns structured analysis including functions, classes, imports, comments, and optionally chunked source segments.

## Features

| Feature                    | Description                                                                |
| -------------------------- | -------------------------------------------------------------------------- |
| **306 Languages**          | Pre-compiled parsers for 306 programming languages                         |
| **Code Intelligence**       | Extract functions, classes, imports, docstrings, symbols from source       |
| **Data Extraction**         | Hierarchical key-value trees from 17 config/data formats (JSON, YAML, TOML, properties, XML, CSV, …) |
| **On-Demand Downloads**    | Parsers are downloaded on-demand and cached locally for fast reuse         |
| **Selective Installation** | Download only the languages you need; unused parsers never downloaded      |
| **Polyglot Bindings**      | Native bindings across 15 languages — Rust, Python, Node.js, WebAssembly, Go, Java, C#, PHP, Ruby, Elixir, Dart, Kotlin, Swift, Zig, and C/C++ |
| **Automatic Caching**      | Downloaded parsers cached in platform-specific directories for offline use |
| **CLI Tool**               | `ts-pack download` to pre-download parsers for offline/CI/Docker use       |

## Supported Languages

This pack includes 306 languages. See the [full language list](docs/languages.md) for all supported grammars with extensions and repository links.

## Package READMEs

- [Rust](crates/ts-pack-core/README.md) -- Rust core library providing access to 306 tree-sitter parsers with on-demand download and caching support.
- [Python](packages/python/README.md) -- Python bindings for tree-sitter-language-pack, providing access to 306 pre-compiled tree-sitter parsers with on-demand downloads.
- [Node.js](crates/ts-pack-core-node/README.md) -- Node.js NAPI bindings for tree-sitter-language-pack with on-demand parser downloads.
- [Go](packages/go/README.md) -- Go bindings for tree-sitter-language-pack with on-demand parser caching.
- [Java](packages/java/README.md) -- Java bindings for tree-sitter-language-pack with on-demand parser downloads (JDK 22+).
- [Kotlin (Android + JVM)](packages/kotlin-android/README.md) -- Kotlin Android AAR and host-JVM bindings with on-demand parser downloads. JNI is first-class for gradle test execution in CI/CD without Android emulator.
- [Elixir](packages/elixir/README.md) -- Elixir bindings for tree-sitter-language-pack with on-demand parser downloads.
- [Ruby](packages/ruby/README.md) -- Ruby bindings for tree-sitter-language-pack with on-demand parser downloads.
- [WebAssembly](crates/ts-pack-core-wasm/README.md) -- WebAssembly bindings for tree-sitter-language-pack. Includes a curated subset of 30 languages optimized for browser and edge runtimes. For all 306 languages, use native bindings (Python, Node.js, etc.).
- [PHP](packages/php/README.md) -- PHP extension via ext-php-rs with on-demand parser downloads.
- [.NET (C#)](packages/csharp/README.md) -- .NET P/Invoke bindings with on-demand parser downloads.
- [Dart / Flutter](packages/dart/README.md) -- Dart and Flutter bindings via flutter_rust_bridge with isolate-safe Future APIs.
- [Swift](packages/swift/README.md) -- Swift bindings via swift-bridge for macOS, iOS, and Linux with async/await.
- [Zig](packages/zig/README.md) -- Zig bindings over the C FFI with idiomatic error sets and explicit ownership.
- [C/C++ (FFI)](crates/ts-pack-core-ffi/README.md) -- C-compatible FFI bindings for tree-sitter-language-pack. Use from any language with C interop.
- [CLI](crates/ts-pack-cli/README.md) -- Command-line tool for managing tree-sitter language parsers with download utilities.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Join our [Discord community](https://discord.gg/xt9WY3GnKR) for questions and discussion.

## Part of Kreuzberg.dev

- [Kreuzberg](https://github.com/kreuzberg-dev/kreuzberg) — document intelligence: text, tables, metadata from 90+ formats with optional OCR.
- [Kreuzberg Cloud](https://github.com/kreuzberg-dev/kreuzberg-cloud) — managed extraction API with SDKs, dashboards, and observability.
- [kreuzcrawl](https://github.com/kreuzberg-dev/kreuzcrawl) — web crawling and scraping with HTML→Markdown and headless-Chrome fallback.
- [html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown) — fast, lossless HTML→Markdown engine.
- [liter-llm](https://github.com/kreuzberg-dev/liter-llm) — universal LLM API client with native bindings for 14 languages and 143 providers.
- [alef](https://github.com/kreuzberg-dev/alef) — the polyglot binding generator that produces all per-language bindings.
- [Discord](https://discord.gg/xt9WY3GnKR) — community, roadmap, announcements.

## License

MIT -- see [LICENSE](LICENSE) for details.

All included tree-sitter grammars are permissively licensed (MIT, Apache-2.0, BSD, ISC, or similar). Copyleft licenses (GPL, AGPL, LGPL, MPL) are not accepted. See [CONTRIBUTING.md](CONTRIBUTING.md) for grammar inclusion criteria.
