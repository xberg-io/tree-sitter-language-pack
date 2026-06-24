---
title: Contributing
description: "How to contribute to tree-sitter-language-pack — adding languages, fixing bugs, improving bindings, and writing docs."
---

Contributions are welcome: adding a grammar, fixing a bug, improving a binding, or writing documentation.

For CI/CD workflow details, see the [CI/CD reference](contributing/ci.md).

## Prerequisites

You'll need the following tools installed:

- [Task](https://taskfile.dev/) — the project task runner
- Rust stable toolchain via [rustup](https://rustup.rs/)
- Python 3.10+ and [uv](https://docs.astral.sh/uv/)
- Node.js 18+ and [pnpm](https://pnpm.io/)

## Getting started

```bash
# Install Task (macOS)
brew install go-task

# Clone the repository
git clone https://github.com/xberg-io/tree-sitter-language-pack.git
cd tree-sitter-language-pack

# Install all language dependencies
task setup

# Build the Rust core
task build

# Run all tests
task test
```

!!! Tip "Linux"
On Debian/Ubuntu, install Task with `apt install go-task` or download from [taskfile.dev](https://taskfile.dev/installation/).

## Common tasks

```bash
task --list          # show all available tasks
task build           # build Rust core + bindings
task test            # run all test suites
task lint            # run all linters (clippy, ruff, oxlint, rubocop, …)
task format          # auto-format all code
task generate:e2e    # regenerate e2e test suites from fixtures
task test:e2e        # run e2e tests
```

Run `task --list` to see all available tasks.

## Adding a language

The most common contribution is adding a new tree-sitter grammar.

### 1. Find or create a grammar

The grammar must:

- **Be permissively licensed** — MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause,
  ISC, or Unlicense only. We do **not** accept GPL, AGPL, LGPL, MPL, or any
  copyleft license. This ensures tree-sitter-language-pack can be used freely
  in any project without imposing license obligations on downstream users.
- Have a **public Git repository**.
- Produce valid `parser.c` output from `tree-sitter generate`.
- Compile cleanly on **Linux, macOS, and Windows**.

### 2. Add the grammar definition

Edit `sources/language_definitions.json` and add an entry:

```json
{
  "mylang": {
    "repo": "https://github.com/example/tree-sitter-mylang",
    "rev": "abc123def456",
    "branch": "main"
  }
}
```

Always pin to an **exact commit** (`rev`), not a branch tip. This ensures reproducible builds.

Available fields:

| Field        | Required | Description                                                            |
| ------------ | -------- | ---------------------------------------------------------------------- |
| `repo`       | Yes      | Grammar repository URL                                                 |
| `rev`        | Yes      | Exact commit SHA to pin                                                |
| `branch`     | No       | Branch name (used by `scripts/pin_vendors.py` to find latest)          |
| `directory`  | No       | Subdirectory within the repo containing the grammar                    |
| `extensions` | No       | File extensions that map to this language (e.g. `["rs"]`)              |
| `ambiguous`  | No       | Extensions shared with other languages (e.g. `{"h": ["cpp", "objc"]}`) |
| `c_symbol`   | No       | Override for the C symbol name when it differs from the language name  |
| `generate`   | No       | Set to `true` to force running `tree-sitter generate` before compiling |

### 3. Build and test

```bash
# Compile the new parser
task build

# Run the test suite
task test

# Verify the parser works end-to-end
ts-pack download mylang
ts-pack parse example.mylang --language mylang
```

### 4. Add test fixtures

Add at least one fixture under `fixtures/` (the e2e suite consumes them) and a runnable snippet under `docs/snippets/<lang>/`:

```json
[
  {
    "id": "mylang_basic_parse",
    "category": "basic",
    "description": "Parse a simple mylang file",
    "language": "mylang",
    "source_code": "// example mylang source",
    "assertions": {
      "tree_not_null": true,
      "has_error_nodes": false
    },
    "tags": ["smoke"]
  }
]
```

Then regenerate and run e2e tests:

```bash
task generate:e2e
task test:e2e
```

### 5. Open a pull request

- **Title:** `feat: add <language> parser`
- **Body:** link to the upstream grammar repository, note any quirks or limitations

## Fixing a bug

1. Check the [issue tracker](https://github.com/xberg-io/tree-sitter-language-pack/issues) — the bug may already be reported.
2. Write a **failing test** that reproduces the issue.
3. Fix the bug in the appropriate crate.
4. Confirm all tests pass with `task test`.
5. Open a PR with a clear description of the root cause and fix.

## Improving bindings

Binding improvements (better error messages, idiomatic APIs, new methods) are
welcome. Each binding lives in `crates/ts-pack-<language>/`. See the
[Architecture](concepts/architecture.md) page for the full crate layout.

Binding changes must:

- **Not add logic that belongs in the Rust core.** Bindings are pure translation layers.
- **Have test coverage** in the binding's native test suite.
- **Follow the existing API surface** documented in `docs/api-mapping.yaml`.

## Documentation

Doc fixes and new guides follow the same workflow as code changes:

1. Fork and create a branch.
2. Edit files under `docs/`.
3. Preview locally with `zensical serve`.
4. Run `task lint` if you touch any scripted checks.
5. Open a pull request.

!!! Tip "Quick edits"
Use the **Edit** button in the page header to jump directly from any docs page to the matching file on GitHub.

## Code quality

The project uses pre-commit hooks managed by [prek](https://github.com/xberg-io/prek):

```bash
prek install
prek install --hook-type commit-msg
```

Before committing, verify these three commands pass:

```bash
task lint     # zero warnings required
task test     # all tests must pass
task format   # code must be formatted
```

## Commit style

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```text
feat: add kotlin parser
fix: correct memory layout in Java FFI array freeing
chore: update tree-sitter to 0.25
docs: add chunking guide
test: add e2e fixtures for ruby
```

Keep commits **small and focused**. Each commit should represent one logical change.

## Pull request checklist

- [ ] `task test` passes
- [ ] `task lint` passes (zero warnings)
- [ ] New language has runnable snippets under `docs/snippets/<lang>/` (validated via `task docs:snippets:check`)
- [ ] `task generate:e2e && task test:e2e` passes
- [ ] `task sync-versions` run if any manifest was bumped
- [ ] PR description explains the change and links related issues

## Getting help

- [GitHub Discussions](https://github.com/xberg-io/tree-sitter-language-pack/discussions) — questions and design conversations
- [Discord](https://discord.gg/xt9WY3GnKR) — real-time chat with maintainers
- [Issue tracker](https://github.com/xberg-io/tree-sitter-language-pack/issues) — bug reports and feature requests
