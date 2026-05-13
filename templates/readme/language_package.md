# {{ name | replace("#", "\\#") }}

{% include 'partials/badges.html' %}

{{ description }}

## Installation

{% include 'partials/installation.md' %}

## Quick Start

{% include 'partials/quick_start.md' %}

## Features

- **306 languages** — pre-compiled tree-sitter grammars covering every major programming language and many minor ones.
- **On-demand download + cache** — parsers fetched at first use; subsequent runs hit the local cache.
- **Code intelligence** — extract functions, classes, imports, exports, symbols, docstrings, and diagnostics with one API.
- **Syntax-aware chunking** — semantic chunks for RAG/LLM pipelines.
- **Polyglot bindings** — Rust core with native bindings for Python, TypeScript, Go, Java, C#, Ruby, PHP, Elixir, and WebAssembly via [alef](https://github.com/kreuzberg-dev/alef).

## Documentation

- **[Documentation](https://docs.tree-sitter-language-pack.kreuzberg.dev)** -- Full docs and API reference
- **[GitHub Repository](https://github.com/kreuzberg-dev/tree-sitter-language-pack)** -- Source, issues, and discussions

## Part of Kreuzberg.dev

- [Kreuzberg](https://github.com/kreuzberg-dev/kreuzberg) — document intelligence: text, tables, metadata from 91+ formats with optional OCR.
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
