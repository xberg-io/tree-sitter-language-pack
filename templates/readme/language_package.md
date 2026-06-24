# {{ name | replace("#", "\\#") }}

{% include 'partials/badges.html' %}

{{ description }}

## What This Package Provides

- **Parser access** — load a tree-sitter language parser by name without wiring individual grammar crates or packages.
- **Code intelligence primitives** — parse trees, functions, classes, imports, exports, symbols, docstrings, diagnostics, and syntax-aware chunks.
- **Shared cache model** — parsers are fetched and cached once, then reused by every call in the process.
- **Same catalog as every binding** — Rust, Python, Node.js, Go, Java, PHP, Ruby, .NET, Elixir, WASM, Dart, Kotlin Android, Swift, Zig, and C FFI use the same grammar set.
  {% if language == "typescript" %}
- **Node-first TypeScript API** — native NAPI package with typed parser and query helpers.
  {% elif language == "python" %}
- **Python package** — wheels for parser loading and code-intelligence pipelines.
  {% elif language == "rust" %}
- **Rust crate** — canonical API used by the other bindings and by Kreuzberg code intelligence.
  {% elif language == "go" %}
- **Go module** — cgo-backed access to the shared parser pack.
  {% elif language == "java" %}
- **Java package** — Panama FFM binding for direct parser calls.
  {% elif language == "php" %}
- **PHP extension** — typed PHP surface over the Rust parser pack.
  {% elif language == "ruby" %}
- **Ruby package** — Magnus-backed parser access with Ruby objects.
  {% elif language == "csharp" %}
- **.NET package** — P/Invoke binding for parser loading and analysis.
  {% elif language == "elixir" %}
- **BEAM package** — Rustler NIF binding for parser calls inside Elixir systems.
  {% elif language == "wasm" %}
- **WASM package** — browser and edge-compatible parser bundle.
  {% elif language == "ffi" %}
- **C ABI** — stable native surface for languages that bind through C.
  {% elif language == "kotlin_android" %}
- **Android AAR** — JNI-backed package for mobile parser workloads.
  {% elif language == "swift" %}
- **SwiftPM package** — swift-bridge API for Apple and Linux targets.
  {% elif language == "dart" %}
- **Dart package** — flutter_rust_bridge Future APIs for Dart and Flutter.
  {% elif language == "zig" %}
- **Zig package** — wrapper over the C FFI with explicit ownership.
  {% endif %}

## Installation

{% include 'partials/installation.md' %}

## Quick Start

{% include 'partials/quick_start.md' %}

## Features

- **300+ languages** — pre-compiled tree-sitter grammars covering every major programming language and many minor ones.
- **On-demand download + cache** — parsers fetched at first use; subsequent runs hit the local cache.
- **Code intelligence** — extract functions, classes, imports, exports, symbols, docstrings, and diagnostics with one API.
- **Syntax-aware chunking** — semantic chunks for RAG/LLM pipelines.
- **Polyglot bindings** — native APIs across 15 languages: Rust, Python, TypeScript/Node.js, Go, Java, C#, Ruby, PHP, Elixir, WebAssembly, Dart, Kotlin, Swift, Zig, and C/C++ via [alef](https://github.com/xberg-io/alef).

## Documentation

- **[Documentation](https://docs.tree-sitter-language-pack.xberg.io)** -- Full docs and API reference
- **[GitHub Repository](https://github.com/xberg-io/tree-sitter-language-pack)** -- Source, issues, and discussions

## Part of Kreuzberg.dev

- [Kreuzberg](https://github.com/xberg-io/kreuzberg) — document intelligence: text, tables, metadata from 90+ formats with optional OCR.
- [Kreuzberg Cloud](https://github.com/xberg-io/kreuzberg-cloud) — managed extraction API with SDKs, dashboards, and observability.
- [kreuzcrawl](https://github.com/xberg-io/kreuzcrawl) — web crawling and scraping with HTML→Markdown and headless-Chrome fallback.
- [html-to-markdown](https://github.com/xberg-io/html-to-markdown) — fast, lossless HTML→Markdown engine.
- [liter-llm](https://github.com/xberg-io/liter-llm) — universal LLM API client with native bindings for 14 languages and 143 providers.
- [alef](https://github.com/xberg-io/alef) — the polyglot binding generator that produces this README and all per-language bindings.
- [Discord](https://discord.gg/xt9WY3GnKR) — community, roadmap, announcements.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](https://github.com/xberg-io/tree-sitter-language-pack/blob/main/CONTRIBUTING.md) for guidelines.

Join our [Discord community](https://discord.gg/xt9WY3GnKR) for questions and discussion.

## License

MIT -- see [LICENSE](https://github.com/xberg-io/tree-sitter-language-pack/blob/main/LICENSE) for details.
