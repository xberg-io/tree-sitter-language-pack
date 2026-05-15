---
title: tree-sitter-language-pack
description: "tree-sitter-language-pack — 306 tree-sitter parsers with code intelligence, syntax-aware chunking, and polyglot bindings for Python, TypeScript, Rust, Go, Java, Kotlin (Android), C#, Ruby, PHP, Elixir, Dart, Swift, Zig, and WebAssembly."
---

## Tree-sitter-language-pack

A Rust core that bundles 306 [tree-sitter](https://tree-sitter.github.io/tree-sitter/) grammars behind one parsing and code-intelligence API. Parsers download on demand and cache locally, so the install footprint stays small. Native bindings ship for Python, TypeScript / Node.js, Rust, Go, Java, Kotlin (Android), C#, Ruby, PHP, Elixir, Dart, Swift, Zig, and WebAssembly, plus the standalone `ts-pack` CLI.

<div class="hero-badges" markdown>

[:material-lightning-bolt: Quick Start](getting-started/quickstart.md){ .md-button .md-button--primary }
[:material-package-variant: Installation](getting-started/installation.md){ .md-button }
[:fontawesome-brands-github: GitHub](https://github.com/kreuzberg-dev/tree-sitter-language-pack){ .md-button }
[:fontawesome-brands-discord: Join our Community](https://discord.gg/xt9WY3GnKR){ .md-button }

</div>

---

### Why tree-sitter-language-pack

<div class="grid cards" markdown>

- :material-code-tags:{ .lg .middle } **306 Languages**

  One pack covers every mainstream language and most niche ones — Python, Rust, Go, Java, TypeScript, C++, Kotlin, Swift, Zig, Elixir, Haskell, Julia, R, and 290+ more.

- :material-flash:{ .lg .middle } **Native-speed Parsing**

  Tree-sitter parsers are C code, called directly from a Rust core. No interpreter overhead, no per-file process spawn.

- :material-cloud-download-outline:{ .lg .middle } **On-demand Download**

  Parsers are fetched and cached on first use. The base install stays small; you only pay for the languages you actually parse.

- :material-lightbulb-on-outline:{ .lg .middle } **Code Intelligence**

  Beyond raw syntax trees: functions, classes, imports, exports, symbols, comments, and docstrings — extracted with one call.

- :material-content-cut:{ .lg .middle } **LLM-aware Chunking**

  Split source at natural boundaries (functions, classes, blocks) so chunks stay semantically intact for embeddings and prompt windows.

- :material-translate:{ .lg .middle } **14 Language Bindings + CLI**

  The same Rust core ships as a PyPI wheel, an npm module, a crate, a Go module, a Maven JAR, an Android AAR (Maven), a NuGet package, a gem, a Composer package, a Hex package, a pub.dev package, a SwiftPM package, a Zig tarball, a WASM module, and a static-binary CLI.

</div>

---

### Language Support

| Language                 | Install                                                                                                        | API Reference                                |
| :----------------------- | :------------------------------------------------------------------------------------------------------------- | :------------------------------------------- |
| **Python**               | `pip install tree-sitter-language-pack`                                                                        | [API Reference](reference/api-python.md)     |
| **TypeScript / Node.js** | `npm install @kreuzberg/tree-sitter-language-pack`                                                             | [API Reference](reference/api-typescript.md) |
| **Rust**                 | `cargo add tree-sitter-language-pack`                                                                          | [API Reference](reference/api-rust.md)       |
| **Go**                   | `go get github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go`                                        | [API Reference](reference/api-go.md)         |
| **Java**                 | Maven Central `dev.kreuzberg.treesitterlanguagepack:tree-sitter-language-pack`                                 | [API Reference](reference/api-java.md)       |
| **C#**                   | `dotnet add package TreeSitterLanguagePack`                                                                    | [API Reference](reference/api-csharp.md)     |
| **Ruby**                 | `gem install tree_sitter_language_pack`                                                                        | [API Reference](reference/api-ruby.md)       |
| **PHP**                  | `composer require kreuzberg-dev/tree-sitter-language-pack`                                                     | [API Reference](reference/api-php.md)        |
| **Elixir**               | `{:tree_sitter_language_pack, "~> 1.9"}`                                                                       | [API Reference](reference/api-elixir.md)     |
| **Dart / Flutter**       | `dart pub add tree_sitter_language_pack`                                                                       | [API Reference](reference/api-dart.md)       |
| **Kotlin (Android)**     | `implementation("dev.kreuzberg.tslp:tslp-android:1.9.0")`                                                      | [API Reference](reference/api-kotlin-android.md) |
| **Swift**                | `.package(url: "https://github.com/kreuzberg-dev/tree-sitter-language-pack-swift", from: "1.9.0")`             | [API Reference](reference/api-swift.md)      |
| **Zig**                  | `zig fetch --save <release tarball url>`                                                                       | [API Reference](reference/api-zig.md)        |
| **WebAssembly**          | `npm install @kreuzberg/tree-sitter-language-pack-wasm`                                                        | [API Reference](reference/api-wasm.md)       |
| **C (FFI)**              | Shared library + header                                                                                        | [API Reference](reference/api-c.md)          |
| **CLI**                  | `curl -fsSL https://raw.githubusercontent.com/kreuzberg-dev/tree-sitter-language-pack/main/install.sh \| bash` | [CLI Guide](guides/cli.md)                   |

→ **[See all 306 supported languages](languages.md)**

---

### Explore the Docs

<div class="grid cards" markdown>

- :material-rocket-launch:{ .lg .middle } **Getting Started**

  Install for your language, download parsers, and parse your first file in minutes.

  [:octicons-arrow-right-24: Installation](getting-started/installation.md) · [:octicons-arrow-right-24: Quick Start](getting-started/quickstart.md)

- :material-code-braces:{ .lg .middle } **Parsing**

  Build syntax trees, choose a language, walk nodes, handle parse errors.

  [:octicons-arrow-right-24: Parsing guide](guides/parsing.md)

- :material-lightbulb-on-outline:{ .lg .middle } **Code Intelligence**

  Structure, imports, exports, symbols, comments, and docstrings — not just raw nodes.

  [:octicons-arrow-right-24: Code intelligence guide](guides/intelligence.md)

- :material-content-cut:{ .lg .middle } **Chunking for LLMs**

  Split source at natural boundaries so chunks stay semantically intact.

  [:octicons-arrow-right-24: Chunking guide](guides/chunking.md)

- :material-puzzle-outline:{ .lg .middle } **Concepts**

  Architecture, download model, and the code-intelligence pipeline.

  [:octicons-arrow-right-24: Architecture](concepts/architecture.md)

- :material-api:{ .lg .middle } **API Reference**

  Complete reference for every binding: Python, TypeScript, Rust, Go, Java, Kotlin (Android), C#, Ruby, PHP, Elixir, Dart, Swift, Zig, WASM, and C FFI.

  [:octicons-arrow-right-24: Python API](reference/api-python.md)

</div>

---

### Part of kreuzberg.dev

Tree-sitter-language-pack is built by the [kreuzberg.dev](https://kreuzberg.dev) team, the same people behind a family of Rust-core, polyglot-bindings libraries.

<div class="grid cards kreuzberg-ecosystem-grid" markdown>

- :material-file-document-multiple-outline:{ .lg .middle } **[Kreuzberg](https://docs.kreuzberg.dev/)**

  Document intelligence for 90+ formats — PDF, Office, images, HTML, email — with optional OCR.

- :material-cloud-outline:{ .lg .middle } **[Kreuzberg Cloud](https://docs.kreuzberg.cloud/)**

  Managed document extraction API. Same engine as the open-source library, hosted.

- :material-language-html5:{ .lg .middle } **[html-to-markdown](https://docs.html-to-markdown.kreuzberg.dev/)**

  Fast HTML to Markdown conversion with the same Rust-core, polyglot-bindings shape.

- :material-spider-web:{ .lg .middle } **[kreuzcrawl](https://docs.kreuzcrawl.kreuzberg.dev/)**

  Polite, resumable web crawler that hands pages to html-to-markdown or Kreuzberg for extraction.

- :material-robot-outline:{ .lg .middle } **[liter-llm](https://docs.liter-llm.kreuzberg.dev/)**

  Universal LLM API client: one surface across many providers, proxy and MCP servers included.

- :fontawesome-brands-discord:{ .lg .middle } **[Discord](https://discord.gg/xt9WY3GnKR)**

  Join the community for questions, design discussions, and announcements across all kreuzberg.dev projects.

</div>

---

### Getting Help

- **Bugs and feature requests** — [Open an issue on GitHub](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues)
- **Community chat** — [Join the Discord](https://discord.gg/xt9WY3GnKR)
- **Contributing** — [Read the contributor guide](contributing.md)
