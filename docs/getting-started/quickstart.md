---
title: Quick Start
description: "Parse your first file with tree-sitter-language-pack in under 5 minutes."
---

This guide walks you from install to parsing, code intelligence, and LLM chunking.

---

## 1. Install

=== "Python"

    ```bash
    pip install tree-sitter-language-pack
    ```

=== "Node.js"

    ```bash
    npm install @kreuzberg/tree-sitter-language-pack
    ```

=== "Rust"

    ```bash
    cargo add tree-sitter-language-pack
    ```

=== "CLI"

    ```bash
    brew install kreuzberg-dev/tap/ts-pack
    ```

!!! Tip "Other ecosystems" Go, Java, Ruby, Elixir, PHP, and WebAssembly are also supported. See [Installation](installation.md) for the full list.

---

## 2. Download Parsers

Parsers download automatically on first use. For **production, CI, Docker, or offline** environments, pre-download them.

### Specific languages

=== "CLI"

    --8<-- "snippets/cli/download.md"

=== "Python"

    --8<-- "snippets/python/download.md"

=== "Node.js"

    --8<-- "snippets/typescript/download.md"

=== "Ruby"

    --8<-- "snippets/ruby/download.md"

=== "PHP"

    --8<-- "snippets/php/download.md"

=== "Go"

    --8<-- "snippets/go/download.md"

=== "Java"

    --8<-- "snippets/java/download.md"

=== "C#"

    --8<-- "snippets/csharp/download.md"

=== "Elixir"

    --8<-- "snippets/elixir/download.md"

=== "Dart"

    --8<-- "snippets/dart/download.md"

=== "Swift"

    --8<-- "snippets/swift/download.md"

=== "Zig"

    --8<-- "snippets/zig/download.md"

=== "Kotlin (Android)"

    --8<-- "snippets/kotlin-android/download.md"

=== "WebAssembly"

    --8<-- "snippets/wasm/download.md"

=== "Rust"

    --8<-- "snippets/rust/download.md"

### All 306 languages

=== "CLI"

    ```bash
    ts-pack download --all
    ```

=== "Python"

    ```python
    from tree_sitter_language_pack import download_all

    download_all()
    ```

=== "Node.js"

    ```typescript
    import { downloadAll } from "@kreuzberg/tree-sitter-language-pack";

    await downloadAll();
    ```

=== "Rust"

    ```rust
    use tree_sitter_language_pack::download_all;

    download_all()?;
    ```

### By language group

Groups bundle related languages: `web`, `systems`, `scripting`, `data`, `jvm`, `functional`.

=== "CLI"

    ```bash
    # Download all web languages (HTML, CSS, JS, TS, Vue, Svelte, …)
    ts-pack download --groups web,data

    # See what's cached
    ts-pack list --downloaded
    ```

=== "Python"

    ```python
    from tree_sitter_language_pack import init

    init({"groups": ["web", "data"]})
    ```

=== "Node.js"

    ```typescript
    import { init } from "@kreuzberg/tree-sitter-language-pack";

    await init({ groups: ["web", "data"] });
    ```

=== "Rust"

    ```rust
    use tree_sitter_language_pack::{PackConfig, init};

    let config = PackConfig {
        groups: Some(vec!["web".into(), "data".into()]),
        ..Default::default()
    };
    init(&config)?;
    ```

### Docker and CI

Pre-download parsers during your build to avoid runtime network calls:

```dockerfile title="Dockerfile"
FROM python:3.12-slim
RUN pip install tree-sitter-language-pack
# Pre-download at build time — no network needed at runtime
RUN python -c "from tree_sitter_language_pack import download_all; download_all()"
```

```yaml title="GitHub Actions"
- name: Install and pre-download parsers
  run: |
    pip install tree-sitter-language-pack
    python -c "from tree_sitter_language_pack import download; download(['python', 'javascript', 'rust'])"
```

### Configuration file

Declare which languages your project needs in a `language-pack.toml`:

```toml title="language-pack.toml"
languages = ["python", "javascript", "rust", "go"]
# groups = ["web", "systems"]
# cache_dir = "/tmp/parsers"
```

Then download everything declared in the config:

=== "CLI"

    ```bash
    # Reads language-pack.toml automatically
    ts-pack download
    ```

=== "Python"

    ```python
    from tree_sitter_language_pack import init

    # Reads language-pack.toml from current directory
    init()
    ```

!!! Info "Cache location" Parsers cache to `~/.cache/tree-sitter-language-pack/` on Linux/macOS and `%LOCALAPPDATA%\tree-sitter-language-pack\` on Windows. Override with `cache_dir` in `language-pack.toml` or the programmatic API. See [Download Model](../concepts/download-model.md) for full details.

---

## 3. Parse Code

Build a concrete syntax tree from source code.

=== "CLI"

    --8<-- "snippets/cli/quickstart.md"

=== "Python"

    --8<-- "snippets/python/quickstart.md"

=== "Node.js"

    --8<-- "snippets/typescript/quickstart.md"

=== "Ruby"

    --8<-- "snippets/ruby/quickstart.md"

=== "PHP"

    --8<-- "snippets/php/quickstart.md"

=== "Go"

    --8<-- "snippets/go/quickstart.md"

=== "Java"

    --8<-- "snippets/java/quickstart.md"

=== "C#"

    --8<-- "snippets/csharp/quickstart.md"

=== "Elixir"

    --8<-- "snippets/elixir/quickstart.md"

=== "Dart"

    --8<-- "snippets/dart/quickstart.md"

=== "Swift"

    --8<-- "snippets/swift/quickstart.md"

=== "Zig"

    --8<-- "snippets/zig/quickstart.md"

=== "Kotlin (Android)"

    --8<-- "snippets/kotlin-android/quickstart.md"

=== "WebAssembly"

    --8<-- "snippets/wasm/quickstart.md"

=== "Rust"

    --8<-- "snippets/rust/quickstart.md"

---

## 4. Extract Code Intelligence

Go beyond the raw syntax tree. Extract functions, classes, imports, docstrings, and more with `process`.

=== "CLI"

    --8<-- "snippets/cli/process.md"

=== "Python"

    --8<-- "snippets/python/process.md"

=== "Node.js"

    --8<-- "snippets/typescript/process.md"

=== "Ruby"

    --8<-- "snippets/ruby/process.md"

=== "PHP"

    --8<-- "snippets/php/process.md"

=== "Go"

    --8<-- "snippets/go/process.md"

=== "Java"

    --8<-- "snippets/java/process.md"

=== "C#"

    --8<-- "snippets/csharp/process.md"

=== "Elixir"

    --8<-- "snippets/elixir/process.md"

=== "Dart"

    --8<-- "snippets/dart/process.md"

=== "Swift"

    --8<-- "snippets/swift/process.md"

=== "Zig"

    --8<-- "snippets/zig/process.md"

=== "Kotlin (Android)"

    --8<-- "snippets/kotlin-android/process.md"

=== "WebAssembly"

    --8<-- "snippets/wasm/process.md"

=== "Rust"

    --8<-- "snippets/rust/process.md"

---

## 5. Run Extraction Queries

Use `extract` to run custom tree-sitter queries and get structured results with captured text and metadata.

=== "Python"

    ```python
    import tree_sitter_language_pack as tslp

    source = """
    def greet(name: str) -> str:
        return f"Hello, {name}!"

    def farewell(name: str) -> str:
        return f"Goodbye, {name}!"
    """

    result = tslp.extract(source, {
        "language": "python",
        "patterns": {
            "functions": {
                "query": "(function_definition name: (identifier) @name)",
                "capture_output": "Text",
            }
        }
    })

    for match in result["results"]["functions"]["matches"]:
        print(match["captures"][0]["text"])
    # greet
    # farewell
    ```

---

## 7. Chunk for LLMs

Split code at natural boundaries so language models receive coherent, complete units which is ideal for embedding pipelines and context windows.

=== "Python"

    ```python
    from tree_sitter_language_pack import process, ProcessConfig

    with open("large_module.py") as f:
        source = f.read()

    config = ProcessConfig(
        language="python",
        chunk_max_size=1500,  # max bytes per chunk
        structure=True,
    )
    result = process(source, config)

    for i, chunk in enumerate(result["chunks"]):
        print(f"Chunk {i}: lines {chunk['start_line']}-{chunk['end_line']} "
              f"({chunk['end_byte'] - chunk['start_byte']} bytes)")
    ```

=== "Node.js"

    ```typescript
    import { process } from "@kreuzberg/tree-sitter-language-pack";
    import { readFileSync } from "fs";

    const source = readFileSync("large_module.ts", "utf8");

    const result = await process(source, {
      language: "typescript",
      chunkMaxSize: 1500,
      structure: true,
    });

    result.chunks.forEach((chunk, i) => {
      console.log(`Chunk ${i}: lines ${chunk.startLine}-${chunk.endLine} (${chunk.endByte - chunk.startByte} bytes)`);
    });
    ```

=== "CLI"

    ```bash
    # Chunk a file for LLM ingestion
    ts-pack process large_module.py --chunk-size 1500 \
      | jq '.chunks[] | {start: .start_line, end: .end_line, bytes: (.end_byte - .start_byte)}'
    ```

---

You now have the full workflow. You can now install, download, parse, extract intelligence, run queries, and chunk for LLMs.
Go further with the following guides:

- [:material-arrow-right: Parsing guide](../guides/parsing.md) — syntax trees, error handling, and incremental parsing
- [:material-arrow-right: Configuration](../guides/configuration.md) — `language-pack.toml` and advanced options
- [:material-arrow-right: API Reference](../reference/api-python.md) — full API docs for every binding
