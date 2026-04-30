---
title: Chunking for LLMs
description: "Split code at natural syntax boundaries for LLM ingestion — never mid-function, never mid-class."
---

Naive line-count or character-count splitting breaks code apart at random. A function split across two chunks loses its signature. A class split mid-method gives the model half a definition.

Syntax-aware chunking walks the concrete syntax tree and splits at natural boundaries. Here's the difference:

```python
def process_order(order_id: str, quantity: int) -> dict:
    """Process an order and return the result."""
    # validate input
    if quantity <= 0:
        raise ValueError("quantity must be positive")
    item = fetch_item(order_id)
    price = item["price"] * quantity
    return {"order_id": order_id, "total": price, "status": "pending"}
```

Naive chunking at 100 bytes might split after `raise ValueError(...)`, leaving the return statement in the next chunk. Syntax-aware chunking keeps `process_order` together as one unit. The chunker splits inside a function when that function alone exceeds the byte budget.

## Basic usage

Set `chunk_max_size` in `ProcessConfig` to enable chunking:

=== "Python"

    ```python
    from tree_sitter_language_pack import process, ProcessConfig

    with open("src/service.py") as f:
        source = f.read()

    result = process(source, ProcessConfig(
        language="python",
        chunk_max_size=1000,   # max bytes per chunk
        structure=True,        # include structure metadata
    ))

    for i, chunk in enumerate(result["chunks"]):
        print(f"Chunk {i + 1}: lines {chunk['start_line']}-{chunk['end_line']} "
              f"({chunk['end_byte'] - chunk['start_byte']} bytes)")
    ```

=== "Node.js"

    ```typescript
    import { process } from "@kreuzberg/tree-sitter-language-pack";
    import { readFileSync } from "fs";

    const source = readFileSync("src/service.ts", "utf8");

    const result = await process(source, {
      language: "typescript",
      chunkMaxSize: 1000,
      structure: true,
    });

    result.chunks.forEach((chunk, i) => {
      console.log(`Chunk ${i + 1}: lines ${chunk.startLine}-${chunk.endLine} (${chunk.endByte - chunk.startByte} bytes)`);
    });
    ```

=== "Rust"

    ```rust
    use tree_sitter_language_pack::{process, ProcessConfig};

    let mut config = ProcessConfig::new("rust").with_chunking(1000);
    config.structure = true;

    let result = process(&source, &config)?;

    for (i, chunk) in result.chunks.iter().enumerate() {
        println!("Chunk {}: lines {}-{} ({} bytes)",
            i + 1, chunk.start_line, chunk.end_line, chunk.end_byte - chunk.start_byte);
    }
    ```

=== "CLI"

    ```bash
    ts-pack process src/service.py --chunk-size 1000 \
      | jq '.chunks[] | {lines: "\(.start_line)-\(.end_line)", bytes: (.end_byte - .start_byte)}'
    ```

## Chunk fields

| Field | Type | Description |
|-------|------|-------------|
| `content` | str | Source code text for this chunk |
| `start_byte` | int | Start byte offset in source |
| `end_byte` | int | End byte offset in source |
| `start_line` | int | First line (1-indexed) |
| `end_line` | int | Last line (1-indexed) |
| `node_types` | list[str] | Top-level tree-sitter node types in this chunk |

## How it works

The chunker runs three passes:

1. Collect top-level declarations (functions, classes, methods) as atomic units. Comments and docstrings above a declaration attach to it.
2. Pack units into chunks without exceeding `chunk_max_size`. When the current chunk would overflow, close it and start a new one.
3. For any single unit that exceeds `chunk_max_size` on its own, split at the next logical sub-boundary — between methods in a class, or between statement blocks in a function.

The result: functions are never split unless they're individually too large, decorators stay with their function, and imports group into a single chunk at the top.

## Byte budget

`chunk_max_size` is an upper bound in bytes, not a fixed size. The chunker may produce smaller chunks when a natural boundary falls before the limit.

## Structure metadata with chunks

When `structure=True` is also set, each chunk's `node_types` field shows what kind of code it contains. This is useful for metadata-enriched vector store ingestion:

```python
config = ProcessConfig(
    language="python",
    chunk_max_size=1000,
    structure=True,
    docstrings=True,
)
result = process(source, config)

documents = []
for chunk in result["chunks"]:
    documents.append({
        "content": chunk["content"],
        "metadata": {
            "language": "python",
            "start_line": chunk["start_line"],
            "end_line": chunk["end_line"],
            "node_types": chunk["node_types"],
            "size_bytes": chunk["end_byte"] - chunk["start_byte"],
        }
    })
```

## Indexing a repository

A complete example that walks a codebase and produces LLM-ready chunks:

```python
import os
from pathlib import Path
from tree_sitter_language_pack import process, ProcessConfig, has_language

LANGUAGE_MAP = {
    ".py": "python", ".js": "javascript", ".ts": "typescript",
    ".rs": "rust",   ".go": "go",         ".java": "java",
    ".rb": "ruby",   ".ex": "elixir",     ".php": "php",
    ".cs": "csharp", ".cpp": "cpp",       ".c": "c",
}

def chunk_repository(repo_path: str, chunk_size: int = 800) -> list[dict]:
    chunks = []
    for root, _, files in os.walk(repo_path):
        for filename in files:
            ext = Path(filename).suffix
            language = LANGUAGE_MAP.get(ext)
            if not language or not has_language(language):
                continue

            filepath = os.path.join(root, filename)
            try:
                source = Path(filepath).read_text(encoding="utf-8", errors="ignore")
            except OSError:
                continue

            result = process(source, ProcessConfig(
                language=language,
                chunk_max_size=chunk_size,
                structure=True,
                imports=True,
                docstrings=True,
            ))

            for chunk in result["chunks"]:
                chunks.append({
                    "content": chunk["content"],
                    "file": filepath,
                    "start_line": chunk["start_line"],
                    "end_line": chunk["end_line"],
                    "language": language,
                    "node_types": chunk["node_types"],
                    "size_bytes": chunk["end_byte"] - chunk["start_byte"],
                })
    return chunks

docs = chunk_repository("./my-project")
print(f"{len(docs)} chunks from {len(set(d['file'] for d in docs))} files")
```

## Next steps

- [Code intelligence](intelligence.md) — the other `ProcessConfig` fields that work alongside chunking
- [Concepts: Code intelligence](../concepts/code-intelligence.md) — the extraction engine design
