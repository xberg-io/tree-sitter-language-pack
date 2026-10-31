---
description: Use when the user wants to know which programming language a file or snippet is. Covers implicit detection in `ts-pack parse`/`process`, confirming support with `ts-pack list`/`info`, and the SDK detection functions for path, extension, and raw content.
name: tree-sitter-language-pack-detecting-languages
---

<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:1d8f23b5f26e371f9ced71e7f86f664ef0731d0ad7acce227eba69b83b348582
Source-Hash: blake3:09e8364c2021bf5e301e76fc67a2b84dd47e26431c68749ecdf69db12ef87caf
Schema-Version: v1
-->

# Detecting languages

tree-sitter-language-pack maps a file to one of 372 supported languages.
Detection works from a file path, a bare extension, or — via the SDK — the
file content itself.

## On the CLI: detection is implicit

There is no standalone `ts-pack detect` command. `parse` and `process`
auto-detect the language from the file extension and act on it:

```bash
ts-pack parse src/app.ts      # detects "typescript", parses
ts-pack process app.py        # detects "python", extracts
```

When detection fails (no extension, ambiguous, or stdin), pass `--language`:

```bash
cat snippet | ts-pack parse - --language go
```

To check whether a language name is supported and cached without parsing
anything, use `list` and `info`:

```bash
ts-pack list --filter type      # languages whose name contains "type"
ts-pack list --manifest         # every language in the remote manifest
ts-pack info typescript         # is it known? is it downloaded? cache path?
```

## In the SDK: explicit detection

The core exposes three detection functions, including content-based
detection that the CLI does not surface:

| Function | Detects from |
| -------- | ------------ |
| `detect_language_from_path(path)` | Full path (uses the extension). |
| `detect_language_from_extension(ext)` | A bare extension like `"rs"`. |
| `detect_language_from_content(content)` | Raw source text — no path needed. |

Each returns the language name or null/None when no grammar matches.

### Python

```python
from tree_sitter_language_pack import (
    detect_language_from_path,
    detect_language_from_extension,
    detect_language_from_content,
)

detect_language_from_path("src/app.py")      # "python"
detect_language_from_extension("rs")          # "rust"
detect_language_from_content(open("x").read())  # from the shebang line, if present
```

### Node.js / TypeScript

```typescript
import { detectLanguageFromPath, detectLanguageFromContent } from "@xberg-io/tree-sitter-language-pack";

const lang = detectLanguageFromPath("src/app.ts") ?? detectLanguageFromContent(source);
```

## Choosing CLI vs SDK

- Path or extension is enough, one-shot → just run `ts-pack parse`/`process`
  and let it auto-detect.
- Only raw content (a pasted snippet, a stream, a file with no extension) →
  use the SDK's `detect_language_from_content`, then pass the result as
  `--language` to the CLI if needed.

## When to reach for the other skills

Once the language is known, hand off to `tree-sitter-language-pack-parsing-source` (syntax tree) or
`tree-sitter-language-pack-extracting-code-structure` (metadata). To see or prefetch the parser for a
detected language, see `tree-sitter-language-pack-managing-parsers`.
