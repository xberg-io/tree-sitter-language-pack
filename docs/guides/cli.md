---
description: "CLI reference for ts-pack — download parsers, parse source code, and run code intelligence."
---

# CLI reference

`ts-pack` downloads tree-sitter parsers and runs code intelligence from the command line.

## Installation

=== "Cargo"

    ```bash
    cargo install ts-pack-cli
    ```

=== "Homebrew (macOS / Linux)"

    ```bash
    brew trust xberg-io/tap
    brew install xberg-io/tap/ts-pack
    ```

Verify the install:

```bash
ts-pack --version
```

## Commands

| Command                               | What it does                              |
| ------------------------------------- | ----------------------------------------- |
| [`download`](#ts-pack-download)       | Download parser libraries                 |
| [`clean`](#ts-pack-clean)             | Remove cached parsers                     |
| [`list`](#ts-pack-list)               | List available languages                  |
| [`info`](#ts-pack-info)               | Show details about a language             |
| [`parse`](#ts-pack-parse)             | Parse a file and output the syntax tree   |
| [`process`](#ts-pack-process)         | Run code intelligence on a file           |
| [`cache-dir`](#ts-pack-cache-dir)     | Print the cache directory path            |
| [`init`](#ts-pack-init)               | Create a `language-pack.toml` config file |
| [`completions`](#ts-pack-completions) | Generate shell completions                |

---

### `ts-pack download`

Download parser libraries to the local cache.

```bash
ts-pack download [LANGUAGES...] [OPTIONS]
```

| Flag                | Description                                                                                     |
| ------------------- | ----------------------------------------------------------------------------------------------- |
| `--all`             | Download all 306 parsers                                                                        |
| `--groups <groups>` | Download by group (comma-separated: `web`, `systems`, `scripting`, `data`, `jvm`, `functional`) |
| `--fresh`           | Clear the cache before downloading                                                              |

```bash
# Download specific parsers
ts-pack download python javascript typescript

# Download everything
ts-pack download --all

# Re-download from scratch, ignoring the cache
ts-pack download python --fresh

# Download a group
ts-pack download --groups web,systems

# Download whatever languages-pack.toml specifies (if one is found)
ts-pack download
```

Without arguments and with no `language-pack.toml` present, the command exits with an error.

---

### `ts-pack clean`

Remove all cached parser libraries. Prompts for confirmation unless you pass `--force`.

```bash
ts-pack clean [OPTIONS]
```

| Flag      | Description                  |
| --------- | ---------------------------- |
| `--force` | Skip the confirmation prompt |

```bash
ts-pack clean          # prompts: "Continue? [y/N]"
ts-pack clean --force  # no prompt
```

---

### `ts-pack list`

List available languages.

```bash
ts-pack list [OPTIONS]
```

| Flag              | Description                                           |
| ----------------- | ----------------------------------------------------- |
| `--downloaded`    | Show only locally cached parsers                      |
| `--manifest`      | Show all languages from the remote manifest (default) |
| `--filter <text>` | Filter by substring                                   |

```bash
ts-pack list                    # all available languages
ts-pack list --downloaded       # only what's in the cache
ts-pack list --filter script    # languages whose name contains "script"
```

---

### `ts-pack info`

Show details about a specific language.

```text title="Usage"
ts-pack info <LANGUAGE>
```

```bash
ts-pack info python
```

Output when the parser has downloaded:

```text
Language:    python
Known:       true
Downloaded:  true
Cache path:  /home/user/.cache/tree-sitter-language-pack/libtree_sitter_python.so
```

Before the parser downloads, `Cache path` shows the cache directory instead.

---

### `ts-pack parse`

Parse source code and output the syntax tree.

```bash
ts-pack parse <FILE> [OPTIONS]
```

Use `-` as `FILE` to read from stdin (requires `--language`).

| Flag                      | Description                                                      |
| ------------------------- | ---------------------------------------------------------------- |
| `--language <lang>`, `-l` | Language name. Auto-detected from the file extension if omitted. |
| `--format <fmt>`, `-f`    | `sexp` (default) or `json`                                       |

```bash
ts-pack parse src/main.py
ts-pack parse src/main.py --format json
echo "def hello(): pass" | ts-pack parse - --language python
```

Sample `sexp` output:

```text
(module
  (function_definition
    name: (identifier)
    parameters: (parameters)
    body: (block
      (expression_statement
        (call ...)))))
```

The JSON format wraps the sexp string alongside the language name and an `has_errors` boolean.

---

### `ts-pack process`

Run code intelligence on a source file and output structured JSON.

```bash
ts-pack process <FILE> [OPTIONS]
```

Use `-` as `FILE` to read from stdin. When reading from stdin, you must pass `--language`.

| Flag                      | Description                                             |
| ------------------------- | ------------------------------------------------------- |
| `--language <lang>`, `-l` | Language name. Auto-detected from extension if omitted. |
| `--all`                   | Enable all analysis features                            |
| `--structure`             | Extract functions, classes, and methods                 |
| `--imports`               | Extract import statements                               |
| `--exports`               | Extract exported symbols                                |
| `--comments`              | Extract comments                                        |
| `--symbols`               | Extract all identifiers                                 |
| `--docstrings`            | Extract docstrings                                      |
| `--diagnostics`           | Report syntax errors                                    |
| `--chunk-size <n>`        | Split output into chunks of at most `n` bytes           |

Without any feature flags, the default extracts structure, imports, and exports.

```bash
# Full analysis
ts-pack process src/app.py --all

# Structure only
ts-pack process src/app.py --structure

# Chunk a large file for LLM ingestion
ts-pack process large_module.py --chunk-size 800

# From stdin
cat src/main.go | ts-pack process - --language go --imports
```

---

### `ts-pack cache-dir`

Print the effective cache directory path.

```bash
ts-pack cache-dir
# /home/user/.cache/tree-sitter-language-pack

# Use in scripts
CACHE=$(ts-pack cache-dir)
du -sh "$CACHE"
```

---

### `ts-pack init`

Create a `language-pack.toml` config file in the current directory.

```bash
ts-pack init [OPTIONS]
```

| Flag                  | Description                                        |
| --------------------- | -------------------------------------------------- |
| `--cache-dir <path>`  | Set a custom cache directory in the generated file |
| `--languages <langs>` | Comma-separated languages to pre-fill              |

```bash
ts-pack init
ts-pack init --languages python,javascript,typescript,rust
```

Generated file (empty init):

```toml
# language-pack.toml
# languages = ["python", "rust"]
```

With `--languages python,rust`:

```toml
languages = ["python", "rust"]
```

After init, run `ts-pack download` to fetch the listed parsers.

---

### `ts-pack completions`

Generate shell completion scripts.

```text title="Usage"
ts-pack completions <SHELL>
```

Supported: `bash`, `zsh`, `fish`, `powershell`, `elvish`.

```bash
ts-pack completions bash >> ~/.bash_completion
ts-pack completions zsh > ~/.zsh/completions/_ts-pack
ts-pack completions fish > ~/.config/fish/completions/ts-pack.fish
```

---

## Exit codes

| Code | Meaning                           |
| ---- | --------------------------------- |
| `0`  | Success                           |
| `1`  | Error — message printed to stderr |

---

## Use in CI

Pre-download parsers and cache them between runs:

```yaml
- name: Install ts-pack
  run: cargo install ts-pack-cli

- name: Cache parsers
  uses: actions/cache@v4
  with:
    path: ~/.cache/tree-sitter-language-pack
    key: tslp-${{ hashFiles('language-pack.toml') }}

- name: Download parsers
  run: ts-pack download

- name: Analyze
  run: ts-pack process src/main.py --all
```
