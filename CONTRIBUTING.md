# Contributing to Tree-Sitter Language Pack

Thank you for your interest in contributing to tree-sitter-language-pack! This guide will help you get started with development.

## Table of Contents

- [Development Setup](#development-setup)
  - [Task Installation](#task-installation)
  - [Quick Start](#quick-start)
- [Development Workflow](#development-workflow)
  - [Common Commands](#common-commands)
  - [Language-Specific Tasks](#language-specific-tasks)
- [Parser Caching](#parser-caching)
- [Adding Languages](#adding-languages)
- [E2E Tests](#e2e-tests)
- [Exploring Tasks](#exploring-tasks)
- [Code Quality](#code-quality)
- [Submitting Changes](#submitting-changes)

## Development Setup

### Task Installation

This project uses [Task](https://taskfile.dev/) for task automation and orchestration. Task is a task runner that simplifies development workflows across multiple languages and platforms.

#### Install Task

Choose the installation method for your platform:

**macOS (Homebrew):**

```bash
brew install go-task
```

**Linux:**

```bash
# Using the installer script
sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b ~/.local/bin
# Or via package managers:
apt install go-task  # Debian/Ubuntu
pacman -S go-task    # Arch
```

**Windows:**

```powershell
# Using Scoop
scoop install task

# Or using Chocolatey
choco install go-task
```

For complete installation instructions, visit the [official Task documentation](https://taskfile.dev/installation/).

### Quick Start

After installing Task, set up your development environment:

```bash
# One-time setup - installs all dependencies
task setup

# Clone grammar sources
task clone

# Build in dev mode (a few languages, fast iteration)
task build:dev
```

The setup command will install Rust, Python, Node.js, Go, Java, and Elixir tooling as needed.

## Development Workflow

### Common Commands

```bash
# Build all crates (all languages, dynamic mode)
task build

# Build in dev mode (few languages, fast iteration)
task build:dev

# Build in release mode (optimized)
task build:release
```

```bash
# Run all tests
task test

# Run all checks (lint + test)
task check
```

```bash
# Format all code
task format

# Run all linters via prek
task lint

# Generate READMEs from templates
task generate-readme
```

```bash
# Update all dependencies
task update

# Clean all build artifacts
task clean
```

### Language-Specific Tasks

Each language binding has its own namespace:

**Rust:**

```bash
task rust:build
task rust:test
task rust:format
task rust:lint
```

**Python:**

```bash
task python:install
task python:test
task python:format
task python:lint
```

**Node.js:**

```bash
task node:build        # Build NAPI-RS native module (release)
task node:build:dev    # Build in debug mode
task node:test
```

**Go:**

```bash
task go:build          # Build Go bindings (requires FFI)
task go:build:ffi      # Build FFI static library for Go
task go:test
task go:format
task go:lint
```

**Java:**

```bash
task java:build:ffi    # Build FFI shared library for Java
task java:test
```

**Elixir:**

```bash
task elixir:build      # Compile (includes Rustler NIF)
task elixir:test
task elixir:deps
```

**Ruby:**

```bash
task ruby:build        # Build Ruby native extension
task ruby:test         # Run Ruby tests
task ruby:format       # Format Ruby code
task ruby:lint         # Lint Ruby code
```

**WebAssembly:**

```bash
task wasm:build         # Build WASM package (web target)
task wasm:build:bundler # Build WASM package (bundler target)
task wasm:build:node    # Build WASM package (Node.js target)
task wasm:test          # Run WASM tests
```

**C:**

```bash
task c:build:ffi       # Build FFI library for C tests
task c:e2e:build       # Build C E2E tests
task c:e2e:test        # Run C E2E tests
```

## Parser Caching

Cloning 306 tree-sitter grammar repositories is slow. The build system includes a multi-layer caching strategy to avoid redundant work.

### How It Works

1. **Cache manifest** (`parsers/.cache_manifest.json`): Tracks a SHA256-based key for each language derived from its full configuration (repo URL, revision, branch, directory, generate flag, ABI version). On subsequent runs, only languages whose configuration has changed — or whose parser files are missing from disk — are re-cloned.

2. **CI cache** (`actions/cache@v4`): All CI workflows cache the `parsers/` directory keyed on `sources/language_definitions.json`. When definitions haven't changed between runs, the clone step completes instantly.

3. **Stale entry cleanup**: If a language is removed from `language_definitions.json`, the next run deletes its parser directory and manifest entry automatically.

### Environment Variables

| Variable          | Default                  | Description                                                                        |
| ----------------- | ------------------------ | ---------------------------------------------------------------------------------- |
| `TSLP_CACHE_DIR`  | `<project_root>/parsers` | Override the directory where compiled parser sources are stored                    |
| `TSLP_VENDOR_DIR` | `<project_root>/vendor`  | Override the directory where grammar repos are cloned                              |
| `TSLP_NO_CACHE`   | (unset)                  | Set to `1`, `true`, or `yes` to force a full re-clone, ignoring the cache manifest |

### Common Scenarios

```bash
# Normal clone (uses cache, only re-clones changed languages)
task clone

# Force full re-clone (deletes parsers/ and vendor/, re-clones everything)
TSLP_NO_CACHE=1 task clone

# Use a custom cache directory (useful for shared CI caches)
TSLP_CACHE_DIR=/tmp/tslp-parsers task clone
```

## Adding Languages

### License Requirements

All included grammars **must** be released under a permissive open-source license such as MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, or Unlicense. We do **not** accept grammars licensed under GPL, AGPL, LGPL, MPL, or any other copyleft license. This policy ensures that tree-sitter-language-pack can be freely used in any project — commercial or otherwise — without imposing license obligations on downstream users.

Before proposing a new grammar, verify its license by checking the `LICENSE` file in the grammar repository.

### Steps

1. **Add a language definition** to `sources/language_definitions.json`:

   ```json
   {
     "language_name": {
       "repo": "https://github.com/tree-sitter/tree-sitter-language",
       "rev": "commit-hash",
       "branch": "main",
       "directory": "path/to/src",
       "generate": false
     }
   }
   ```

   Fields:
   - `repo` (required): Repository URL
   - `rev` (required): Specific commit hash for reproducible builds
   - `branch` (optional): Branch name if not "main"
   - `directory` (optional): Path to src folder if not in root
   - `generate` (optional): Run tree-sitter generate command

1. **Add a Cargo feature** for the language in `crates/ts-pack-core/Cargo.toml`

1. **Clone and build**

   ```bash
   task clone
   task build:dev
   ```

1. **Regenerate E2E smoke fixtures and test**

   ```bash
   task e2e:generate:smoke-fixtures
   task e2e:generate:all
   task test
   ```

## E2E Tests

E2E tests are generated from JSON fixtures in `tools/e2e-generator/fixtures/` and produce runnable test suites for each language binding.

```bash
# Generate E2E tests for all languages
task e2e:generate:all

# Generate for a specific language
task e2e:generate:rust
task e2e:generate:python
task e2e:generate:go
task e2e:generate:java
task e2e:generate:elixir
task e2e:generate:ruby
task e2e:generate:c

# Run Rust E2E tests
task e2e:test:rust

# Auto-generate smoke fixtures from language_definitions.json
task e2e:generate:smoke-fixtures
```

Generated test files in `e2e/` should not be edited directly — modify fixtures or the generator source instead.

## Exploring Tasks

```bash
# Show all available tasks
task --list

# Show all tasks including internal ones
task --list-all
```

## Code Quality

### Pre-commit Hooks

The project uses [prek](https://github.com/Goldziher/gitfluff) for pre-commit hooks:

```bash
# Install hooks
prek install
prek install --hook-type commit-msg

# Run all hooks manually
prek run --all-files
```

### Commit Messages

We use conventional commits:

- `feat: add support for tree-sitter-language`
- `fix: correct parser initialization for language`
- `docs: update installation instructions`
- `chore: update dependencies`
- `test: add tests for new language`

## Submitting Changes

1. **Create a feature branch**

   ```bash
   git checkout -b feat/add-language-support
   ```

1. **Make your changes** and run checks locally:

   ```bash
   task check
   ```

1. **Commit and push**

   ```bash
   git commit -m "feat: add support for new language"
   git push origin feat/add-language-support
   ```

1. **Create a Pull Request** — link any related issues and ensure CI passes.

## Maintenance Tasks

### Updating Language Versions

```bash
# Update all languages to latest revisions
uv run --no-sync scripts/pin_vendors.py

# Update only missing revisions
uv run --no-sync scripts/pin_vendors.py --only-missing

# Update specific languages
uv run --no-sync scripts/pin_vendors.py --languages=python,rust,go
```

### Version Synchronization

Version is managed in `Cargo.toml` workspace and synced across all manifests:

```bash
task version:sync
```

## Questions?

- Check existing [issues](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues)
- Join our [Discord community](https://discord.gg/xt9WY3GnKR)

Thank you for contributing to tree-sitter-language-pack!
