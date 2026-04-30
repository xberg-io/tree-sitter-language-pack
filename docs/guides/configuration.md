---
description: "Configuring tree-sitter-language-pack — cache directories, pre-downloads, and configuration discovery."
---

# Configuration

Downloaded parser binaries go in the cache directory. The default:

```text
~/.cache/tree-sitter-language-pack/<version>/libs/
```

You can customize the cache location, pre-download languages on startup, and wire up automatic discovery through a TOML file, the programmatic API, or CLI commands.

## Language-pack.toml

Create `language-pack.toml` in your project root:

```toml
# language-pack.toml
languages = ["python", "javascript", "typescript", "rust"]

# Optional: language groups (web, systems, data, jvm, functional, scripting)
# groups = ["web", "systems"]

# Optional: custom cache directory
# cache_dir = ".cache/ts-pack"
```

Run `ts-pack init` to generate this file interactively, or create it by hand.

### Configuration discovery

The library searches for `language-pack.toml` in this order:

1. Current directory and parent directories (up to 10 levels)
2. `$XDG_CONFIG_HOME/tree-sitter-language-pack/config.toml` (`~/.config/tree-sitter-language-pack/config.toml` on Linux/macOS)

CLI flags override config file settings.

### Monorepo example

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
    "bash",
    "dockerfile",
    "yaml",
    "json",
]

# Shared cache across all sub-projects
cache_dir = ".cache/tree-sitter"
```

## Programmatic API

=== "Python"

    ```python
    from tree_sitter_language_pack import init, configure, PackConfig
    from pathlib import Path

    # Pre-download specific languages
    init(languages=["python", "javascript", "rust"])

    # Download by language group
    init(groups=["web"])      # JavaScript, TypeScript, HTML, CSS
    init(groups=["systems"])  # C, C++, Rust, Go
    init(groups=["data"])     # Python, R, SQL, JSON

    # Combine languages and groups
    init(languages=["python"], groups=["web", "systems"])

    # Set custom cache directory (call before first parse)
    configure(cache_dir="/opt/ts-pack-cache")

    # Load from language-pack.toml
    config = PackConfig.from_toml_file(Path("language-pack.toml"))
    if config.languages:
        init(languages=config.languages)

    # Discover in current dir and parents
    config = PackConfig.discover()
    if config and config.languages:
        init(languages=config.languages)
    ```

=== "Node.js"

    ```typescript
    import { init, configure, PackConfig } from "@kreuzberg/tree-sitter-language-pack";

    await init({ languages: ["python", "javascript", "rust"] });
    await init({ groups: ["web"] });
    await configure({ cacheDir: "/opt/ts-pack-cache" });
    ```

=== "Rust"

    ```rust
    use tree_sitter_language_pack::PackConfig;
    use std::path::Path;

    // Programmatic configuration
    let config = PackConfig {
        cache_dir: Some(Path::new("/opt/cache").to_path_buf()),
        languages: Some(vec!["python".to_string(), "rust".to_string()]),
        groups: None,
    };

    // Load from file
    let config = PackConfig::from_toml_file(Path::new("language-pack.toml"))?;

    // Discover in parent directories
    if let Some(config) = PackConfig::discover() {
        println!("Found languages: {:?}", config.languages);
    }
    ```

## Build-time environment variables

`build.rs` reads these at compile time, not at runtime. See [Building from source](building.md) for full details.

| Variable | Description |
|----------|-------------|
| `TSLP_LANGUAGES` | Comma-separated languages to compile statically into the binary |
| `TSLP_LINK_MODE` | `dynamic` (default), `static`, or `both` |
| `PROJECT_ROOT` | Override directory search for `sources/language_definitions.json` |
| `WASI_SYSROOT` | WASI sysroot path for `wasm32-wasi` cross-compilation |
| `TSLP_BUILD` | Set to `1` to force rebuild of the Elixir NIF native extension |

## CLI commands

### `ts-pack init`

Create a `language-pack.toml`:

```bash
# Interactive
ts-pack init

# With specific languages
ts-pack init --languages python,javascript,typescript,rust

# With custom cache directory
ts-pack init --cache-dir ./local-cache --languages python
```

### `ts-pack cache-dir`

Print the effective cache directory:

```bash
ts-pack cache-dir
# /home/user/.cache/tree-sitter-language-pack/<version>/libs/

# Use in scripts
CACHE=$(ts-pack cache-dir)
du -sh "$CACHE"
```

### `ts-pack download`

```bash
# Languages from config
ts-pack download

# Specific languages
ts-pack download python rust javascript

# All available languages
ts-pack download --all

# Clear cache and re-download
ts-pack download --fresh

# By group
ts-pack download --groups web,systems
```

### `ts-pack list`

```bash
# All available languages
ts-pack list

# Cached only
ts-pack list --downloaded

# Filter by name
ts-pack list --filter python
```

## Troubleshooting

**Downloads failing**

```bash
ts-pack cache-dir           # verify the cache path
ts-pack download python     # retry the download
ts-pack clean               # clear a corrupted cache
```

For offline environments: pre-download on a machine with network access, then copy the cache directory to the target machine. See [Docker](docker.md) for baking parsers into a container image.

**Disk space**

```bash
du -sh ~/.cache/tree-sitter-language-pack

# Move the cache to a larger drive
mv ~/.cache/tree-sitter-language-pack /mnt/large-drive/ts-pack-cache
ln -s /mnt/large-drive/ts-pack-cache ~/.cache/tree-sitter-language-pack
```

## Next steps

- [Building from source](building.md) — compile-time flags and environment variables
- [Docker](docker.md) — bake parsers into container images
- [Parsing code](parsing.md) — syntax trees after languages finish downloading
