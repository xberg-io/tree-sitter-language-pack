# tree-sitter-language-pack

Parse and extract code intelligence from 372 programming languages with tree-sitter — structure, imports, symbols, and syntax-aware chunking — using the local `ts-pack` CLI in your agent.

<!-- ~keep TODO: add screenshot -->

## Install

### From the marketplace (recommended)

Pending review for official Claude marketplace.

Self-host:

```text
/plugin marketplace add xberg-io/tree-sitter-language-pack
/plugin install tree-sitter-language-pack@tree-sitter-language-pack
```

### Binary requirement

The bundled MCP launcher (`scripts/mcp-launch.sh`) resolves a `ts-pack` binary
automatically on first run: it reuses one already on `PATH`, then tries
`npx`/`uvx`, then Homebrew, then a prebuilt download from the tool's latest
GitHub release. No manual install is required to use the MCP server.

To install the `ts-pack` CLI yourself:

```bash
brew install xberg-io/tap/ts-pack
# or run it without a persistent install (the CLI proxy package self-installs the binary):
npx @xberg-io/ts-pack-cli --help
uvx --from ts-pack-cli ts-pack --help
# or download a prebuilt binary from the latest GitHub release:
#   https://github.com/xberg-io/tree-sitter-language-pack/releases/latest
# or install from crates.io (binary is installed as `ts-pack`):
cargo install ts-pack-cli
```

Parser libraries download on demand the first time a language is used. For
offline or CI runs, prefetch them:

```bash
ts-pack download --all          # every language
ts-pack download python rust    # specific languages
```

### SDKs (optional)

```bash
pip install tree-sitter-language-pack                  # Python
npm install @xberg-io/tree-sitter-language-pack       # Node.js / TypeScript
```

Rust, Ruby, Go, Java, C#, PHP, Elixir, Dart, Kotlin (Android), Swift, Zig, and WebAssembly bindings are also published.

## Skills shipped

| Skill | Trigger |
|-------|---------|
| **tree-sitter-language-pack** | Parse and extract code intelligence from 372 languages. Use when writing code that parses source, extracts structure/imports/exports/symbols/docstrings/comments, detects a language, runs diagnostics, or chunks code for LLMs — in Rust, Python, Node.js/TypeScript, or the ts-pack CLI. |
| **parsing-source** | Use when the user wants a tree-sitter syntax tree for a source file — an s-expression dump or JSON tree. |
| **extracting-code-structure** | Use when the user wants structured code metadata — functions, classes, imports, exports, symbols, docstrings, comments, or syntax diagnostics. |
| **chunking-for-llms** | Use when splitting source code into chunks for an LLM context window without breaking syntax mid-construct. |
| **detecting-languages** | Use when the user wants to know which programming language a file or snippet is — by path, extension, or content. |
| **managing-parsers** | Use when managing the parser cache — prefetch for offline/CI, list downloaded languages, inspect a language, find the cache dir, or clean it. |
| **using-the-mcp-server** | Use when parsing, processing, or detecting languages through the MCP server's `parse`/`process`/`detect_language` tools instead of the CLI. Covers the tool surface and the auto-installing launcher. |

## MCP server

The plugin auto-registers an MCP server named `tree-sitter-language-pack`,
launched via `scripts/mcp-launch.sh` (which execs `ts-pack mcp`). It exposes
eight tools for parsing, analysis, and cache management; resources for language
discovery; a code-analysis prompt; and language-name completions.

**Tools** (8):

- **parse** — syntax tree (s-expression or JSON), auto-detecting language from path or explicit.
- **process** — code intelligence: structure, imports, exports, symbols, docstrings, comments, diagnostics, chunks. Use `all` flag for all features.
- **detect_language** — identify language by path or content (the tool takes `path` and `content` only).
- **list_languages** — available, downloaded, or manifest languages; filter by name.
- **info** — show language availability and cache status.
- **download** — prefetch parsers: specific languages, groups, all, or fresh updates.
- **cache_dir** — return the cache directory path.
- **clean_cache** — remove all cached parsers.

**Resources**:

- `ts-pack://languages` — catalog of all known languages.
- `ts-pack://languages/downloaded` — currently cached languages.
- `ts-pack://language/{name}` — per-language metadata template.

**Prompt**: `analyze-code` (args: `language`, optional `focus`).

**Completions**: language-name autocomplete.

The launcher auto-installs a binary on first run (override with
`TS_PACK_LAUNCHER=auto|npx|uvx|brew|download`). The `mcp` subcommand ships in a
recent release of the tool; an older binary on `PATH` may need an upgrade to
expose it. See the **using-the-mcp-server** skill for details.

## CLI

`ts-pack` subcommands:

| Command | Purpose |
|---------|---------|
| `parse <file>` | Parse into a syntax tree (`--format sexp\|json`). |
| `process <file>` | Code-intelligence pipeline → JSON (feature flags below). |
| `list` | List languages (`--downloaded`, `--manifest`, `--filter`). |
| `info <language>` | Show whether a language is known and cached. |
| `download [langs...]` | Download parsers (`--all`, `--groups`, `--fresh`). |
| `clean` | Remove all cached parsers (`--force`). |
| `cache-dir` | Print the cache directory. |
| `init` | Write a `language-pack.toml` config. |
| `completions <shell>` | Generate shell completions. |
| `mcp` | Start the MCP server (`--transport stdio`). |

`process` flags: `--structure`, `--imports`, `--exports`, `--comments`,
`--symbols`, `--docstrings`, `--diagnostics`, `--all`, `--chunk-size <bytes>`,
`--language <name>`. With no feature flags it defaults to
`--structure --imports --exports`.

There is no `detect` or `validate` subcommand: language detection is
implicit in `parse`/`process` (from the file extension), and diagnostics
come from `process --diagnostics`. Content-based detection is available in
the SDK (`detect_language_from_content`).

## SDK

```python
from tree_sitter_language_pack import process, ProcessConfig

result = process(source_code, ProcessConfig("python", symbols=True, docstrings=True))
for item in result.structure:  # ProcessResult is an object, not a dict
    print(item.kind, item.name, item.span.start_line)
```

```typescript
import { process, detectLanguageFromPath } from "@xberg-io/tree-sitter-language-pack";

const lang = detectLanguageFromPath("src/app.ts");
const result = process(source, { language: lang ?? "typescript", structure: true, imports: true });
```

## Configuration

`ts-pack init` writes a `language-pack.toml` pinning a cache directory and a
language set:

```toml
cache_dir = ".ts-pack-cache"
languages = ["python", "rust", "typescript"]
```

Commit it to pin the parser set across a team or CI. The cache location can
also be inspected with `ts-pack cache-dir`.

## Examples

Parse a file into a syntax tree:

```text
ts-pack parse src/main.rs
```

Extract structure and imports as JSON:

```text
ts-pack process src/app.ts --structure --imports
```

Chunk a large file for an LLM:

```text
ts-pack process big_module.py --chunk-size 2000
```

## Versioning

The plugin version tracks the tree-sitter-language-pack core release (`Cargo.toml`). See [CHANGELOG.md](../CHANGELOG.md) for release notes.

## License

MIT. The upstream [tree-sitter-language-pack](https://github.com/xberg-io/tree-sitter-language-pack) library is also MIT-licensed.

## See also

- **Marketplace**: this repo — `/plugin marketplace add xberg-io/tree-sitter-language-pack`
- **Upstream**: [xberg-io/tree-sitter-language-pack](https://github.com/xberg-io/tree-sitter-language-pack)
