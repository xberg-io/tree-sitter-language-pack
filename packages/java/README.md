# Java

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0">
	<a href="https://github.com/kreuzberg-dev/alef">
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
	<a href="https://pkg.go.dev/github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go">
		<img
			src="https://img.shields.io/github/v/tag/kreuzberg-dev/tree-sitter-language-pack?label=Go&color=007ec6"
			alt="Go"
		/>
	</a>
	<a href="https://www.nuget.org/packages/TreeSitterLanguagePack/">
		<img src="https://img.shields.io/nuget/v/TreeSitterLanguagePack?label=C%23&color=007ec6" alt="C#" />
	</a>
	<a href="https://packagist.org/packages/kreuzberg-dev/tree-sitter-language-pack">
		<img
			src="https://img.shields.io/packagist/v/kreuzberg-dev/tree-sitter-language-pack?label=PHP&color=007ec6"
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
	<a href="https://github.com/kreuzberg-dev/tree-sitter-language-pack/tree/main/packages/swift">
		<img src="https://img.shields.io/badge/Swift-SPM-007ec6" alt="Swift" />
	</a>
	<a href="https://github.com/kreuzberg-dev/tree-sitter-language-pack/tree/main/packages/zig">
		<img src="https://img.shields.io/badge/Zig-package-007ec6" alt="Zig" />
	</a>
	<a href="https://github.com/kreuzberg-dev/tree-sitter-language-pack/releases">
		<img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C FFI" />
	</a>

	<!-- Project Info -->
	<a href="https://github.com/kreuzberg-dev/tree-sitter-language-pack/blob/main/LICENSE">
		<img src="https://img.shields.io/badge/License-MIT-007ec6" alt="License" />
	</a>
	<a href="https://docs.tree-sitter-language-pack.kreuzberg.dev">
		<img src="https://img.shields.io/badge/Docs-tree--sitter--language--pack-007ec6" alt="Documentation" />
	</a>
</div>

<div align="center" style="margin: 24px 0 0">
	<a href="https://kreuzberg.dev">
		<img alt="tree-sitter-language-pack" src="https:&#x2f;&#x2f;github.com&#x2f;user-attachments&#x2f;assets&#x2f;478a83da-237b-446b-b3a8-e564c13e00a8" />
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

Pre-compiled tree-sitter grammars for 306 programming languages with Java bindings via Panama FFM.

## What This Package Provides

- **Parser access** — load a tree-sitter language parser by name without wiring individual grammar crates or packages.
- **Code intelligence primitives** — parse trees, functions, classes, imports, exports, symbols, docstrings, diagnostics, and syntax-aware chunks.
- **Shared cache model** — parsers are fetched and cached once, then reused by every call in the process.
- **Same catalog as every binding** — Rust, Python, Node.js, Go, Java, PHP, Ruby, .NET, Elixir, WASM, Dart, Kotlin Android, Swift, Zig, and C FFI use the same grammar set.
- **Java package** — Panama FFM binding for direct parser calls.

## Installation

```xml
<dependency>
  <groupId>dev.kreuzberg</groupId>
  <artifactId>tree-sitter-language-pack</artifactId>
  <version>1.9.0-rc.22</version>
</dependency>
```

## Quick Start

See the [language guide](https://docs.tree-sitter-language-pack.kreuzberg.dev) for `java`-specific usage.

## Features

- **300+ languages** — pre-compiled tree-sitter grammars covering every major programming language and many minor ones.
- **On-demand download + cache** — parsers fetched at first use; subsequent runs hit the local cache.
- **Code intelligence** — extract functions, classes, imports, exports, symbols, docstrings, and diagnostics with one API.
- **Syntax-aware chunking** — semantic chunks for RAG/LLM pipelines.
- **Polyglot bindings** — Rust core with native bindings for Python, TypeScript, Go, Java, C#, Ruby, PHP, Elixir, and WebAssembly via [alef](https://github.com/kreuzberg-dev/alef).

## Documentation

- **[Documentation](https://docs.tree-sitter-language-pack.kreuzberg.dev)** -- Full docs and API reference
- **[GitHub Repository](https://github.com/kreuzberg-dev/tree-sitter-language-pack)** -- Source, issues, and discussions

## Part of Kreuzberg.dev

- [Kreuzberg](https://github.com/kreuzberg-dev/kreuzberg) — document intelligence: text, tables, metadata from 90+ formats with optional OCR.
- [Kreuzberg Cloud](https://github.com/kreuzberg-dev/kreuzberg-cloud) — managed extraction API with SDKs, dashboards, and observability.
- [kreuzcrawl](https://github.com/kreuzberg-dev/kreuzcrawl) — web crawling and scraping with HTML→Markdown and headless-Chrome fallback.
- [html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown) — fast, lossless HTML→Markdown engine.
- [liter-llm](https://github.com/kreuzberg-dev/liter-llm) — universal LLM API client with native bindings for 14 languages and 143 providers.
- [alef](https://github.com/kreuzberg-dev/alef) — the polyglot binding generator that produces this README and all per-language bindings.
- [Discord](https://discord.gg/xt9WY3GnKR) — community, roadmap, announcements.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](https://github.com/kreuzberg-dev/tree-sitter-language-pack/blob/main/CONTRIBUTING.md) for guidelines.

Join our [Discord community](https://discord.gg/xt9WY3GnKR) for questions and discussion.

## License

MIT -- see [LICENSE](https://github.com/kreuzberg-dev/tree-sitter-language-pack/blob/main/LICENSE) for details.
