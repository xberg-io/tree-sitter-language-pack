---
title: tree-sitter-language-pack
description: "tree-sitter-language-pack — 306 tree-sitter parsers with bindings for Python, TypeScript, Rust, Go, Java, Ruby, Elixir, PHP, and WebAssembly."
---

Tree-sitter-language-pack bundles 306 [tree-sitter](https://tree-sitter.github.io/tree-sitter/) grammars behind a single Rust core with native bindings for Python, TypeScript, Rust, Go, Java, Ruby, Elixir, PHP, WebAssembly, and more. Parsers download on demand and cache locally. One API surface for parsing, code intelligence, extraction queries, and syntax-aware chunking for LLM workflows. It ships as libraries per ecosystem plus the `ts-pack` CLI.

<div class="hero-badges" markdown>
[:material-lightning-bolt: Quick Start](getting-started/quickstart.md){ .md-button .md-button--primary }
[:material-package-variant: Installation](getting-started/installation.md){ .md-button }
[:fontawesome-brands-github: GitHub](https://github.com/kreuzberg-dev/tree-sitter-language-pack){ .md-button }
[:fontawesome-brands-discord: Community](https://discord.gg/xt9WY3GnKR){ .md-button }
</div>

---

## Explore the Docs

<div class="grid cards doc-explore-grid" markdown>

- :material-rocket-launch-outline:{ .lg .middle } **Getting started**

    ---

    Install for your language, download parsers, and parse your first file.

    [:material-arrow-right: Installation](getting-started/installation.md) · [:material-arrow-right: Quick start](getting-started/quickstart.md)

- :material-code-braces:{ .lg .middle } **Parsing**

    ---

    Build syntax trees, choose a language, and handle parse errors.

    [:material-arrow-right: Parsing guide](guides/parsing.md)

- :material-lightbulb-on-outline:{ .lg .middle } **Code intelligence**

    ---

    Structure, imports, exports, symbols, comments, and docstrings — not just raw nodes.

    [:material-arrow-right: Code intelligence guide](guides/intelligence.md)

- :material-content-cut:{ .lg .middle } **Chunking for LLMs**

    ---

    Split source at natural boundaries so chunks stay semantically intact.

    [:material-arrow-right: Chunking guide](guides/chunking.md)

- :material-cloud-download-outline:{ .lg .middle } **Download model**

    ---

    On-demand grammars, cache layout, manifest, and offline or CI workflows.

    [:material-arrow-right: Download model](concepts/download-model.md)

- :material-file-document-outline:{ .lg .middle } **API reference**

    ---

    Full reference for Python, TypeScript, Rust, Go, Java, Ruby, Elixir, PHP, WASM, C FFI, and the CLI.

</div>

---

### Part of kreuzberg.dev

Tree-sitter-language-pack is built by the [kreuzberg.dev](https://kreuzberg.dev) team, the same people behind a family of Rust-core, polyglot-bindings libraries.

<div class="grid cards kreuzberg-ecosystem-grid" markdown>

- :material-file-document-multiple-outline:{ .lg .middle } **[Kreuzberg](https://docs.kreuzberg.dev/)**

    ---

    Document extraction for 90+ formats: PDF, Office, images, HTML, and more.

- :material-robot-outline:{ .lg .middle } **[liter-llm](https://docs.liter-llm.kreuzberg.dev/)**

    ---

    Universal LLM API client: one API surface across many providers, proxy and MCP servers included.

- :material-language-html5:{ .lg .middle } **[html-to-markdown](https://docs.html-to-markdown.kreuzberg.dev/)**

    ---

    Fast HTML to Markdown conversion with a Rust core and matching bindings.

</div>

---

### Getting help

- **Bugs and feature requests** — [Open an issue on GitHub](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues)
- **Community chat** — [Join the Discord](https://discord.gg/xt9WY3GnKR)
- **Contributing** — [Read the contributor guide](contributing.md)
