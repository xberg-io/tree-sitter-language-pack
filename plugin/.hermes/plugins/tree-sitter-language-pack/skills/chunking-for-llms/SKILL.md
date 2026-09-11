---
name: chunking-for-llms
description: >-
  Use when the user wants to split source code into chunks for an LLM
  context window without breaking syntax mid-construct. Covers
  `ts-pack process --chunk-size`, why syntax-aware splits beat fixed-byte
  splits, picking a size, and the chunk JSON shape.
---

<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:194808ab41cbfc122dee398cccb0ecad135949fb11fd3201110a2ea6f8ec3369
Source-Hash: blake3:9e073d427a178d533cfbe4d3d43f29d564c0bda35ef83d901c3958931fb60dc0
Schema-Version: v1
-->

# Syntax-aware chunking for LLMs

Splitting code on a fixed byte or line count cuts functions in half and
strips context. `ts-pack process <file> --chunk-size <bytes>` splits on
syntactic boundaries (whole functions, classes, blocks) so each chunk is a
coherent unit, and emits them in the JSON `chunks` array.

## Quick recipe

```bash
# ~2 KB chunks aligned to syntax boundaries
ts-pack process src/app.ts --chunk-size 2000
```

`--chunk-size` is a maximum size in bytes. The splitter packs whole
syntactic units up to that bound; an oversized single construct becomes its
own chunk rather than being cut. Chunks are added to the normal `process`
JSON output under `chunks`.

## Picking a size

- Match the downstream model's token budget. A rough rule: bytes ÷ 4 ≈
  tokens for code, so `--chunk-size 4000` is on the order of ~1k tokens.
- Larger chunks preserve more local context but fit fewer per request.
- Leave headroom for the prompt, the surrounding messages, and the
  response — do not size chunks to the full context window.

## Combining with extraction

Chunking composes with the other `process` features, so you can attach
structure metadata to each request:

```bash
ts-pack process src/service.py --structure --chunk-size 3000 \
  | jq '{chunks: (.chunks | length), functions: (.structure | length)}'
```

## Chunk output

`chunks` is a list of code-chunk objects in the `process` JSON. Each chunk
carries its source text plus span information (line/byte offsets), so you
can cite or re-locate a chunk back in the original file. Iterate the array
to feed an LLM one coherent unit at a time:

```bash
ts-pack process big_module.py --chunk-size 2500 \
  | jq -c '.chunks[]'
```

## SDK equivalent

The SDK exposes chunking through the process config: set the
`chunk_max_size` field (in bytes) on `ProcessConfig` — the same value the
CLI's `--chunk-size` flag sets. `ProcessConfig` is a frozen dataclass, so
pass it to the constructor:

```python
from tree_sitter_language_pack import process, ProcessConfig

config = ProcessConfig("python", chunk_max_size=2500)
result = process(source_code, config)
for chunk in result.chunks:          # ProcessResult is an object, not a dict
    send_to_llm(chunk.content)
```

## When not to chunk

For a single small file that already fits the context window, skip
chunking and pass the file whole. Reach for chunking when a file is large,
when you are batching many files into a RAG index, or when you need stable,
syntactically coherent units to cite.
