---
name: using-the-mcp-server
description: >-
  Use when parsing source, extracting code structure, or detecting a language
  through the tree-sitter-language-pack MCP server's tools, rather than shelling
  out to the ts-pack CLI. Covers the tool surface, the auto-installing launcher,
  and when MCP beats the CLI or SDK.
---

<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:4c8c2a57d4c5fa03c6ff57dee5175b3229dcc0af0338d8e31901473b2d46a542
Source-Hash: blake3:9e073d427a178d533cfbe4d3d43f29d564c0bda35ef83d901c3958931fb60dc0
Schema-Version: v1
-->

# Using the MCP Server

The `tree-sitter-language-pack` MCP server exposes `ts-pack`'s parsing and
code-intelligence capabilities as MCP tools, so an MCP-compatible client (Claude
Code, Claude Desktop) can parse source, extract structure, and detect languages
directly, with no CLI invocation or glue code.

## How it runs in this plugin

The plugin auto-registers the server. Its command is the bundled launcher
`scripts/mcp-launch.sh`, which execs `ts-pack mcp`. On first run the launcher
resolves a `ts-pack` binary: it reuses one already on `PATH` or cached in the
plugin's `bin/`, then tries `npx`/`uvx`, then Homebrew, then downloads a prebuilt
from the tool's latest GitHub release. Override the channel with
`TS_PACK_LAUNCHER=auto|npx|uvx|brew|download`.

The `mcp` subcommand ships in a recent release of the tool. If an older
`ts-pack` is already on `PATH`, it may not expose `mcp` yet — upgrade the binary
(`brew upgrade`, re-download, or rebuild) to pick it up.

To run it manually:

```bash
ts-pack mcp        # for Claude Code / Claude Desktop (stdio)
```

## The tools

**Parsing and code intelligence:**

- **parse** — syntax tree from `source`; requires an explicit `language`, with
  optional `format` (`sexp` or `json`). It does not detect language from a path —
  use `detect_language` first if you only have a path.
- **process** — code intelligence: structure, imports, exports, symbols, docstrings, comments, diagnostics, chunks. Use `all` flag to enable all features.
- **detect_language** — identify language by `path` or `content` (those are
  the only two parameters; extension detection is implicit in `path`).

**Language and cache management:**

- **list_languages** — available, downloaded, or manifest languages; optional filter by name.
- **info** — show language availability and cache status.
- **download** — prefetch parsers: specific languages, language groups, all, or fresh updates.
- **cache_dir** — return the cache directory path.
- **clean_cache** — remove all cached parsers.

## Resources

- `ts-pack://languages` — catalog of all known languages.
- `ts-pack://languages/downloaded` — currently cached languages.
- `ts-pack://language/{name}` — per-language metadata template.

## Prompt and completions

- **analyze-code** — code-analysis prompt (args: `language`, optional `focus`).
- **Language-name autocomplete** — available language names in completions.

## When to prefer MCP over the CLI or SDK

- **Prefer MCP** inside an agent session: the agent calls parsing directly as a
  tool, with no shell-out and no process management.
- **Prefer the CLI** for one-shot parsing/extraction and shell/CI pipelines.
- **Prefer the SDK** when embedding parsing in application code or running custom
  tree-sitter queries.
