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

    ```bash
    ts-pack download python javascript rust go
    ```

=== "Python"

    ```python
    from tree_sitter_language_pack import download

    download(["python", "javascript", "rust", "go"])
    ```

=== "Node.js"

    ```typescript
    import { download } from "@kreuzberg/tree-sitter-language-pack";

    await download(["python", "javascript", "rust", "go"]);
    ```

=== "Rust"

    ```rust
    use tree_sitter_language_pack::download;

    download(&["python", "javascript", "rust", "go"])?;
    ```

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

=== "Python"

    ```python
    from tree_sitter_language_pack import get_parser

    parser = get_parser("python")

    source = b"""
    def greet(name: str) -> str:
        return f"Hello, {name}!"

    result = greet("world")
    """

    tree = parser.parse(source)
    root = tree.root_node

    print(root.type)           # module
    print(root.child_count)    # 2
    print(root.sexp()[:120])   # S-expression preview
    ```

=== "Node.js"

    ```typescript
    import { parseString, treeRootNodeType, treeRootChildCount } from "@kreuzberg/tree-sitter-language-pack";

    const source = `
    function greet(name) {
      return \`Hello, \${name}!\`;
    }
    greet("world");
    `;

    const tree = parseString("javascript", source);

    console.log(treeRootNodeType(tree));       // program
    console.log(treeRootChildCount(tree));     // 2
    ```

=== "Rust"

    ```rust
    use tree_sitter_language_pack::get_parser;

    fn main() -> anyhow::Result<()> {
        let mut parser = get_parser("rust")?;

        let source = r#"
    fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }
    "#;

        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();

        println!("{}", root.kind());        // source_file
        println!("{}", root.child_count()); // 1
        println!("{}", root.to_sexp());
        Ok(())
    }
    ```

=== "CLI"

    ```bash
    # Parse a file and display the syntax tree
    ts-pack parse src/main.py

    # Output as JSON
    ts-pack parse src/main.py --format json

    # Parse inline code via stdin
    echo "def hello(): pass" | ts-pack parse --language python
    ```

---

## 4. Extract Code Intelligence

Go beyond the raw syntax tree. Extract functions, classes, imports, docstrings, and more with `process`.

=== "Python"

    ```python
    from tree_sitter_language_pack import process, ProcessConfig

    source = """
    import os
    from pathlib import Path

    def read_file(path: str) -> str:
        \"\"\"Read and return the contents of a file.\"\"\"
        return Path(path).read_text()

    class FileManager:
        def __init__(self, base_dir: str):
            self.base_dir = base_dir

        def get(self, name: str) -> str:
            return read_file(os.path.join(self.base_dir, name))
    """

    config = ProcessConfig(
        language="python",
        structure=True,   # functions and classes
        imports=True,     # import statements
        comments=True,    # inline comments
        docstrings=True,  # docstring extraction
    )
    result = process(source, config)

    print(f"Imports:   {[i['name'] for i in result['imports']]}")
    print(f"Symbols:   {[s['name'] for s in result['structure']]}")
    print(f"Docstring: {result['structure'][0]['docstring']}")
    ```

=== "Node.js"

    ```typescript
    import { process } from "@kreuzberg/tree-sitter-language-pack";

    const source = `
    import fs from "fs";
    import { join } from "path";

    /**
     * Read and return the contents of a file.
     */
    function readFile(path: string): string {
      return fs.readFileSync(path, "utf8");
    }

    class FileManager {
      constructor(private baseDir: string) {}

      get(name: string): string {
        return readFile(join(this.baseDir, name));
      }
    }
    `;

    const result = await process(source, {
      language: "typescript",
      structure: true,
      imports: true,
      docstrings: true,
    });

    console.log("Imports:", result.imports.map(i => i.name));
    console.log("Symbols:", result.structure.map(s => s.name));
    ```

=== "Rust"

    ```rust
    use tree_sitter_language_pack::{process, ProcessConfig};

    fn main() -> anyhow::Result<()> {
        let source = r#"
    use std::fs;
    use std::path::Path;

    /// Read and return the contents of a file.
    fn read_file(path: &str) -> String {
        fs::read_to_string(path).unwrap()
    }

    struct FileManager {
        base_dir: String,
    }
    "#;

        let mut config = ProcessConfig::new("rust");
        config.structure = true;
        config.imports = true;
        config.docstrings = true;

        let result = process(source, &config)?;

        println!("Imports: {:?}", result.imports.iter().map(|i| &i.name).collect::<Vec<_>>());
        println!("Symbols: {:?}", result.structure.iter().map(|s| &s.name).collect::<Vec<_>>());
        Ok(())
    }
    ```

=== "CLI"

    ```bash
    # Full code intelligence on a file
    ts-pack process src/main.py --structure --imports --docstrings

    # JSON output for piping
    ts-pack process src/main.py --all | jq '.structure[].name'
    ```

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
