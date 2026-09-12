---
name: tree-sitter-language-pack
description: >-
  Parse and extract code intelligence from 371 programming languages using
  tree-sitter grammars. Use when writing code that parses source, extracts
  structure/imports/exports/symbols/docstrings/comments, detects a language,
  runs syntax diagnostics, or produces syntax-aware chunks for LLMs — in
  Rust, Python, Node.js/TypeScript, or the ts-pack CLI. Covers installation,
  the CLI surface, the SDK surface, and parser-cache management.
license: MIT
metadata:
  author: xberg-io
  version: "0.1.0"
  repository: https://github.com/xberg-io/tree-sitter-language-pack
---

<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:acf2620bd0cc3aa6cd470abd32dc6479160b83099ae3de5fc3f658e8e823e1aa
Source-Hash: blake3:08f124151839f7172a3a1d5dc141d6dba3e827f8067d64dce95eca8b1df89d50
Schema-Version: v1
-->

# Tree-Sitter Language Pack

tree-sitter-language-pack is a polyglot code parsing and analysis library
with a high-performance Rust core and polyglot bindings for 15 languages —
Python, Node.js/TypeScript, Ruby, Go, Java, C#, PHP, Elixir, Dart,
Kotlin (Android), Swift, Zig, and WebAssembly, plus a C ABI and JNI. It
compiles 371 tree-sitter grammars into efficient parsers and exposes code
intelligence on top of them.

## Capabilities

- **Parse 371 languages** into concrete syntax trees (s-expression or JSON).
- **Extract structure** — functions, classes, methods, modules with line
  and byte spans, child nesting, and visibility.
- **Extract imports and exports** — statements, sources, exported kinds.
- **Extract symbols** — all identifiers, for search and indexing.
- **Extract docstrings and comments** — attached to definitions or standalone.
- **Language detection** — from a file path, an extension, or content.
- **Syntax diagnostics** — error nodes and syntax errors with positions.
- **Syntax-aware chunking** — split source on syntactic boundaries for LLM
  context windows, not on arbitrary byte offsets.
- **Custom tree-sitter queries** — run your own query patterns over a tree
  (SDK only).
- **Parser cache management** — download, list, inspect, and clean the
  on-demand parser cache for offline/CI use.
- **Distribution** — 15 language bindings (including WebAssembly, a C ABI,
  and JNI) plus the `ts-pack` CLI.

Use this skill when writing code that parses source in any supported
language, extracts code metadata, chunks code for an LLM, detects a
language, or validates syntax.

## When to use

Reach for tree-sitter-language-pack when you need a real syntax tree or
structured code metadata — building a code index, feeding code to an LLM
in semantically coherent chunks, linting for syntax errors across many
languages, or detecting a file's language. For plain text extraction from
documents (PDF, Office, HTML), use Xberg instead.

## CLI vs SDK

- **CLI (`ts-pack`)** — quick one-shot parsing and extraction over files,
  parser-cache management, and shell/CI pipelines. `parse` and `process`
  print to stdout (`process` always emits JSON). Auto-detects the language
  from the file extension.
- **SDK** — embed parsing in an application, run custom tree-sitter
  queries, detect a language from raw content (not just a path), or hold
  parsers across many calls. Use the Rust core or the Python/Node bindings.
- **MCP server (`ts-pack mcp`)** — exposes `parse`/`process`/`detect_language`
  (plus language and cache tools) as agent tools, so an MCP client can parse
  and analyze code directly with no shell-out.
  This plugin auto-registers it; see the **using-the-mcp-server** skill.

Prefer the CLI for ad-hoc work in an agent session; prefer the SDK when the
result feeds back into a larger program; prefer the MCP server when the agent
should call parsing directly as a tool.

## Installation

### CLI

```bash
brew install xberg-io/tap/ts-pack
# or run without a persistent install (the CLI proxy package self-installs the binary):
npx @xberg-io/ts-pack-cli --help
uvx --from ts-pack-cli ts-pack --help
# or download a prebuilt binary from the latest GitHub release:
#   https://github.com/xberg-io/tree-sitter-language-pack/releases/latest
# or install from crates.io:
cargo install ts-pack-cli
# binary is installed as `ts-pack`
```

### Python

```bash
pip install tree-sitter-language-pack
# or: uv add tree-sitter-language-pack
```

### Node.js / TypeScript

```bash
npm install @xberg-io/tree-sitter-language-pack
# or: pnpm add @xberg-io/tree-sitter-language-pack
```

### Rust

```toml
# Cargo.toml
[dependencies]
tree-sitter-language-pack = { version = "1", features = ["download"] }
```

Other bindings: Ruby (`gem install tree_sitter_language_pack`), Go, Java,
C#, PHP, Elixir, Dart, Kotlin (Android), Swift, Zig, and WebAssembly
(`npm install @xberg-io/tree-sitter-language-pack-wasm`). See
<https://github.com/xberg-io/tree-sitter-language-pack>.

## CLI surface

`ts-pack` has these subcommands:

| Command | Purpose |
| ------- | ------- |
| `parse <file>` | Parse a file into a syntax tree (`--format sexp|json`). |
| `process <file>` | Run the code-intelligence pipeline, emit JSON. |
| `detect`* | No standalone command — `parse`/`process` auto-detect from the extension. Use the SDK for content detection. |
| `list` | List available languages (`--downloaded`, `--manifest`, `--filter`). |
| `info <language>` | Show whether a language is known and cached. |
| `download [langs...]` | Download parser libraries (`--all`, `--groups`, `--fresh`). |
| `clean` | Remove all cached parser libraries (`--force`). |
| `cache-dir` | Print the effective cache directory. |
| `init` | Write a `language-pack.toml` config (`--languages`, `--cache-dir`). |
| `completions <shell>` | Generate shell completions. |
| `mcp` | Start the MCP server (`--transport stdio`). |

\* Language detection is implicit in `parse`/`process`. There is no
`detect` or `validate` subcommand; diagnostics come from
`process --diagnostics`.

### Parse

```bash
ts-pack parse src/main.rs                 # s-expression tree (auto-detect)
ts-pack parse src/main.rs --format json   # { language, sexp, has_errors }
ts-pack parse - --language python         # read from stdin, explicit language
```

### Process (code intelligence)

```bash
# Defaults to structure + imports + exports when no feature flags are given:
ts-pack process src/app.ts

# Pick features explicitly:
ts-pack process src/app.ts --structure --imports --symbols --docstrings

# Everything:
ts-pack process src/app.ts --all

# Syntax-aware chunks (bytes) for an LLM:
ts-pack process src/app.ts --chunk-size 2000

# Syntax diagnostics:
ts-pack process src/app.ts --diagnostics
```

`process` always prints JSON. Top-level keys: `language`, `metrics`,
`structure`, `imports`, `exports`, `comments`, `docstrings`, `symbols`,
`diagnostics`, and `chunks` (when `--chunk-size` is set).

### SDK quick start

```python
from tree_sitter_language_pack import process, ProcessConfig, detect_language_from_path

config = ProcessConfig("python", symbols=True, docstrings=True)  # structure/imports/exports default to True
result = process(source_code, config)
for item in result.structure:                 # ProcessResult is an object, not a dict
    print(item.kind, item.name, item.span.start_line)

# Path-based detection (use detect_language_from_content for raw text;
# see the detecting-languages skill):
lang = detect_language_from_path("src/app.py")
```

```typescript
import { process, detectLanguageFromPath } from "@xberg-io/tree-sitter-language-pack";

const lang = detectLanguageFromPath("src/app.ts");
const result = process(source, { language: lang ?? "typescript", structure: true, imports: true });
```

## Parser cache

Parsers download on demand the first time a language is used and are cached
locally. For offline or CI runs, prefetch:

```bash
ts-pack download python rust typescript   # specific languages
ts-pack download --all                    # everything
ts-pack download --groups all             # by group (see manifest_groups())
ts-pack list --downloaded                 # what is cached
ts-pack cache-dir                         # where the cache lives
ts-pack clean --force                     # wipe the cache
```

See the `managing-parsers` skill for the full cache workflow.

## Related skills

- `parsing-source` — parse a file into a syntax tree / s-expression.
- `extracting-code-structure` — `process` with structure/imports/exports/
  symbols/docstrings.
- `chunking-for-llms` — `--chunk-size` syntax-aware splitting.
- `detecting-languages` — detect by path, extension, or content.
- `managing-parsers` — download/clean/list/info parser-cache management.
- `using-the-mcp-server` — call `parse`/`process`/`detect_language` over MCP
  instead of the CLI.

Full documentation: <https://github.com/xberg-io/tree-sitter-language-pack>
