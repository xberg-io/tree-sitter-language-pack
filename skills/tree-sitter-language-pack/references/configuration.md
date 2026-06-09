# Configuration & Download Model Quick Reference

## Cache Directory

Where downloaded parser binaries are stored.

Default locations:

- Linux: `$XDG_CACHE_HOME/tree-sitter-language-pack` or `~/.cache/tree-sitter-language-pack`
- macOS: `~/Library/Caches/tree-sitter-language-pack`
- Windows: `%LOCALAPPDATA%\tree-sitter-language-pack`

Override via:

### Programmatic API

Python:

```python
from tree_sitter_language_pack import configure
configure(cache_dir="/custom/path")
```

TypeScript:

```typescript
import { configure } from "@kreuzberg/tree-sitter-language-pack";
configure({ cacheDir: "/custom/path" });
```

Rust:

```rust
use tree_sitter_language_pack::*;
configure(&PackConfig {
    cache_dir: Some("/custom/path".into()),
    ..Default::default()
})?;
```

### Environment Variable

```bash
export TSLP_CACHE_DIR=/custom/path
```

### CLI Flag

```bash
ts-pack --cache-dir /custom/path download python
```

## language-pack.toml Configuration File

Define languages to pre-download and cache settings.

### Format

```toml
# language-pack.toml

# Language names to pre-download
languages = ["python", "javascript", "typescript", "rust"]

# Optional: language groups (web, systems, data, jvm, functional, scripting)
# groups = ["web", "systems"]

# Optional: custom cache directory
# cache_dir = ".cache/parsers"
```

### Creation

CLI:

```bash
ts-pack init
ts-pack init --languages python,javascript,typescript,rust
ts-pack init --cache-dir ./.cache/parsers --languages python
```

Manual: Create `language-pack.toml` in project root.

### Discovery Order

1. Current directory and parent directories (up to 10 levels)
2. `$XDG_CONFIG_HOME/tree-sitter-language-pack/config.toml` or `~/.config/tree-sitter-language-pack/config.toml`
3. `TSLP_CONFIG` environment variable

Priority: CLI flags > Environment variables > Config file > Defaults

## Download Management

### Pre-downloading Languages

```python
from tree_sitter_language_pack import init, download, download_all

# Pre-download specific languages
init({"languages": ["python", "javascript", "rust"]})

# Or download after init
download(["python", "typescript"])

# Download all 306 languages
download_all()
```

CLI:

```bash
ts-pack download python rust typescript
ts-pack download --all
ts-pack download --groups web,data
```

Returns: number of newly downloaded languages.

### Checking Cache

```python
from tree_sitter_language_pack import downloaded_languages, manifest_languages

local = downloaded_languages()      # No network
remote = manifest_languages()       # Fetches manifest
missing = set(remote) - set(local)
```

CLI:

```bash
ts-pack list --downloaded
ts-pack list --manifest
ts-pack status
```

### Cleaning Cache

```python
from tree_sitter_language_pack import clean_cache
clean_cache()  # Delete all cached parsers
```

CLI:

```bash
ts-pack clean
ts-pack clean --force  # Skip confirmation
```

## Language Groups

Pre-defined collections of related languages:

| Group        | Languages                                      |
| ------------ | ---------------------------------------------- |
| `web`        | JavaScript, TypeScript, HTML, CSS, JSX, TSX    |
| `systems`    | C, C++, Rust, Go, Zig, D                       |
| `data`       | Python, R, SQL, JSON, YAML, CSV                |
| `jvm`        | Java, Kotlin, Scala, Clojure, Groovy           |
| `functional` | Haskell, Lisp, Scheme, Elixir, Erlang, Clojure |
| `scripting`  | Python, Ruby, Bash, Perl, Lua, Vim Script      |

### Usage

Python:

```python
init({"groups": ["web", "systems"]})
```

CLI:

```bash
ts-pack download --groups web,systems
```

## Download on First Use

By default, calling `parse_string()` or `get_parser()` triggers auto-download of uncached languages:

```python
# First call triggers download (network I/O)
tree = parse_string("python", "x = 1")

# Subsequent calls use cached parser (fast)
tree = parse_string("python", "y = 2")
```

This is convenient for development but adds latency in production. Pre-download languages for production deployments.

## Docker & CI Integration

### Dockerfile Pattern

Pre-bake parsers into image:

```dockerfile
FROM python:3.11-slim

RUN pip install tree-sitter-language-pack

# Pre-download parsers (bakes into image)
RUN python -c "from tree_sitter_language_pack import download; download(['python', 'javascript', 'rust'])"

COPY . /app
WORKDIR /app

# Parsing now offline, no network calls
CMD ["python", "app.py"]
```

### GitHub Actions Pattern

Cache parsers between runs:

```yaml
- name: Cache tree-sitter parsers
  uses: actions/cache@v4
  with:
    path: ~/.cache/tree-sitter-language-pack
    key: tslp-${{ hashFiles('language-pack.toml') }}
    restore-keys: tslp-

- name: Download languages
  run: ts-pack download

- name: Run tests
  run: pytest
```

## Configuration Files

### Loading from File

Python:

```python
from tree_sitter_language_pack import PackConfig, init

# Load from language-pack.toml
config = PackConfig.from_toml_file("language-pack.toml")
if config.languages:
    init({"languages": config.languages})

# Auto-discover language-pack.toml
config = PackConfig.discover()
if config:
    init({"languages": config.languages})
```

Rust:

```rust
use tree_sitter_language_pack::PackConfig;

let config = PackConfig::from_toml_file("language-pack.toml")?;
if let Some(languages) = config.languages {
    init(&PackConfig {
        languages: Some(languages),
        ..Default::default()
    })?;
}

// Discover config
if let Some(config) = PackConfig::discover() {
    println!("Found: {:?}", config.languages);
}
```

## Environment Variables

| Variable         | Type   | Default          | Description                |
| ---------------- | ------ | ---------------- | -------------------------- |
| `TSLP_CACHE_DIR` | string | Platform default | Cache directory            |
| `TSLP_CONFIG`    | string | Discovered       | Path to language-pack.toml |
| `TSLP_VERBOSE`   | flag   | off              | Verbose output             |
| `TSLP_NO_COLOR`  | flag   | off              | Disable ANSI colors        |

## Monorepo Setup

Shared cache across multiple sub-projects:

```toml
# language-pack.toml (at repo root)

languages = [
    # Backend
    "python",

    # Frontend
    "javascript",
    "typescript",
    "jsx",
    "tsx",

    # Utilities
    "rust",

    # DevOps
    "bash",
    "dockerfile",
    "yaml",
    "json",
]

# Shared cache
cache_dir = ".cache/tree-sitter"
```

## Download Model Overview

1. Call `get_parser("python")` or `parse_string("python", source)`
2. Check local cache for `python.so` / `python.dylib` / `python.dll`
3. If not cached:
   - Fetch `parsers.json` from GitHub releases (manifest)
   - Get platform-specific download URL for current OS/arch
   - Download binary to cache directory
   - Open via `dlopen` / `LoadLibrary`
4. Subsequent calls use cached binary (no network)

Manifest cached locally and refreshed on version upgrades.

## Offline Mode

1. On a machine with network access:

   ```bash
   ts-pack download --all
   tar czf ts-pack-cache.tar.gz ~/.cache/tree-sitter-language-pack
   ```

2. Transfer to offline machine:

   ```bash
   tar xzf ts-pack-cache.tar.gz -C ~
   ```

3. Parsing now works offline (cache pre-populated)

Alternatively, use Docker image with pre-baked parsers.

## Platform-Specific Notes

### Linux

Default: `$XDG_CACHE_HOME/tree-sitter-language-pack` (or `~/.cache/tree-sitter-language-pack`)

Check size:

```bash
du -sh ~/.cache/tree-sitter-language-pack
```

### macOS

Default: `~/Library/Caches/tree-sitter-language-pack`

For persistent cache on CI, use explicit `cache_dir` in config.

### Windows

Default: `%LOCALAPPDATA%\tree-sitter-language-pack`

Typical path: `C:\Users\<username>\AppData\Local\tree-sitter-language-pack`

## Troubleshooting

### Parser downloads failing

Diagnosis:

```bash
ts-pack cache-dir                        # Check location
ts-pack download python --verbose        # Verbose download
curl -I https://releases.kreuzberg.dev/  # Check network access
```

Solutions:

- Pre-download on machine with network, transfer cache
- Set `TSLP_CACHE_DIR` to custom path
- Use Docker image with pre-baked parsers
- Configure corporate proxy if behind firewall

### Stale cache

```bash
# Clear all parsers
ts-pack clean

# Or manually
rm -rf ~/.cache/tree-sitter-language-pack
```

### Disk space issues

```bash
# Check size
du -sh ~/.cache/tree-sitter-language-pack

# Move to larger drive
mkdir -p /mnt/large/ts-pack-cache
export TSLP_CACHE_DIR=/mnt/large/ts-pack-cache
ts-pack download --all
```

## Quick Start Commands

### Development

```bash
# Create config
ts-pack init --languages python,rust,typescript

# Download languages
ts-pack download

# Parse files
ts-pack parse src/main.py
```

### Production

```bash
# Pre-download before deploying
ts-pack download --all
tar czf parsers.tar.gz ~/.cache/tree-sitter-language-pack

# Deploy with cache
# Deploy parsers.tar.gz alongside application
export TSLP_CACHE_DIR=/opt/parsers
tar xzf parsers.tar.gz -C /opt

# Run application (no network calls)
python app.py
```

### Docker

```bash
# Build image with pre-baked parsers
docker build -t myapp .

# Run (cache mounted if needed)
docker run -it myapp
```
