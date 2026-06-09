# CLI Reference

## Installation

```bash
# From source
cargo install --path crates/ts-pack-cli

# Or from GitHub releases
# Download pre-built binary for your platform
```

## Global Options

```text
--help, -h          Show help message
--version, -V       Show version
```

## Commands

### ts-pack download

Download parser libraries to the local cache.

```bash
ts-pack download [OPTIONS] [LANGUAGES]...
```

Arguments:

- `[LANGUAGES]...` — Language names (space-separated), or use language-pack.toml if omitted

Options:

- `--all` — Download all 306 available languages
- `--groups <GROUPS>` — Language groups (comma-separated: web, systems, scripting, data, jvm, functional)
- `--fresh` — Clean cache before downloading

Examples:

```bash
ts-pack download python rust typescript
ts-pack download --all
ts-pack download --groups web,data
ts-pack download --fresh python rust
ts-pack download  # Uses language-pack.toml if present
```

### ts-pack clean

Remove all cached parser libraries.

```bash
ts-pack clean [OPTIONS]
```

Options:

- `--force` — Skip confirmation prompt

Examples:

```bash
ts-pack clean
ts-pack clean --force
```

### ts-pack list

List available languages.

```bash
ts-pack list [OPTIONS]
```

Options:

- `--downloaded` — Show only cached languages
- `--manifest` — Show all available languages from remote manifest
- `--filter <SUBSTRING>` — Filter by substring

Examples:

```bash
ts-pack list                    # All available
ts-pack list --downloaded       # Only cached
ts-pack list --filter python    # Search for language
```

### ts-pack info

Show details about a specific language.

```bash
ts-pack info <LANGUAGE>
```

Output fields:

- Language name
- Known (compiled-in or in manifest)
- Downloaded (yes/no)
- Cache path (if downloaded) or cache directory

Example:

```bash
ts-pack info python
# Language:    python
# Known:       true
# Downloaded:  true
# Cache path:  /home/user/.cache/ts-pack/libtree_sitter_python.so
```

### ts-pack parse

Parse a file and output the syntax tree.

```bash
ts-pack parse [OPTIONS] <FILE>
```

Arguments:

- `<FILE>` — Source file to parse (use `-` for stdin)

Options:

- `--language, -l <LANG>` — Language name (auto-detected from extension if omitted)
- `--format, -f <FORMAT>` — Output format: `sexp` (default) or `json`

Examples:

```bash
ts-pack parse code.py
ts-pack parse code.py --language python
ts-pack parse code.py --format json
echo "x = 1" | ts-pack parse - --language python
```

JSON output format:

```json
{
  "language": "python",
  "sexp": "(module (expression_statement ...))",
  "has_errors": false
}
```

### ts-pack process

Run code intelligence pipeline on a file.

```bash
ts-pack process [OPTIONS] <FILE>
```

Arguments:

- `<FILE>` — Source file to process (use `-` for stdin)

Options:

- `--language, -l <LANG>` — Language name (auto-detected if omitted; required for stdin)
- `--all` — Enable all analysis features
- `--structure` — Extract functions, classes, methods
- `--imports` — Extract import statements
- `--exports` — Extract export statements
- `--comments` — Extract comments
- `--symbols` — Extract symbols/identifiers
- `--docstrings` — Extract docstrings
- `--diagnostics` — Include parse diagnostics
- `--chunk-size <BYTES>` — Maximum bytes per chunk (enables chunking)

When no feature flags given, defaults apply (structure, imports, exports enabled).

Examples:

```bash
ts-pack process code.py --all
ts-pack process code.py --structure
ts-pack process code.py --all --chunk-size 1000
cat code.py | ts-pack process - --language python --structure
```

Output is JSON to stdout:

```json
{
  "language": "python",
  "metrics": { "total_lines": 10, "code_lines": 8, ... },
  "structure": [...],
  "imports": [...],
  "chunks": [...]
}
```

### ts-pack cache-dir

Print the effective cache directory path.

```bash
ts-pack cache-dir
```

Output:

```text
/home/user/.cache/tree-sitter-language-pack/v1.0.0/libs/
```

Usage in scripts:

```bash
CACHE=$(ts-pack cache-dir)
du -sh "$CACHE"  # Show cache size
```

### ts-pack init

Create a `language-pack.toml` configuration file.

```bash
ts-pack init [OPTIONS]
```

Options:

- `--cache-dir <PATH>` — Set cache directory in config
- `--languages <LANGS>` — Languages to include (comma-separated); also downloads them

Examples:

```bash
ts-pack init
ts-pack init --languages python,rust,typescript
ts-pack init --cache-dir /opt/ts-pack --languages python
```

Generated file (`language-pack.toml`):

```toml
languages = ["python", "rust", "typescript"]
```

### ts-pack completions

Generate shell completions.

```bash
ts-pack completions <SHELL>
```

Arguments:

- `<SHELL>` — bash, zsh, fish, elvish, or powershell

Examples:

```bash
ts-pack completions bash > ~/.local/share/bash-completion/completions/ts-pack
ts-pack completions zsh > ~/.zfunc/_ts-pack
ts-pack completions fish > ~/.config/fish/completions/ts-pack.fish
```

## Exit Codes

- `0` — Success
- `1` — Error (parse failure, missing file, invalid language, network error, etc.)

Errors are printed to stderr.

## Language-pack.toml Configuration

Auto-discovered configuration file:

```toml
languages = ["python", "javascript", "typescript", "rust"]

# Optional language groups
# groups = ["web", "systems", "data"]

# Optional custom cache directory
# cache_dir = ".cache/ts-pack"
```

Discovery order:

1. Current directory and parent directories (up to 10 levels)
2. `$XDG_CONFIG_HOME/tree-sitter-language-pack/config.toml` or `~/.config/tree-sitter-language-pack/config.toml`
3. `TSLP_CONFIG` environment variable

## Common Workflows

### Setup a Project

```bash
ts-pack init --languages python,rust,typescript
ts-pack download
```

### Download and Parse

```bash
ts-pack download python
ts-pack parse code.py
```

### Batch Processing

```bash
ts-pack download python
for file in src/**/*.py; do
    ts-pack process "$file" --structure --imports
done
```

### Extract Code Structure

```bash
ts-pack process src/main.py --structure --format json | jq '.structure'
```

### Check Cache Size

```bash
du -sh "$(ts-pack cache-dir)"
```

## Environment Variables

- `TSLP_CACHE_DIR` — Override cache directory
- `TSLP_CONFIG` — Path to language-pack.toml
- `TSLP_VERBOSE` — Enable verbose output
- `TSLP_NO_COLOR` — Disable colored output
