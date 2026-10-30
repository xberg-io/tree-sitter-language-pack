---
title: Contributing
description: "How to contribute to tree-sitter-language-pack — adding languages, fixing bugs, improving bindings, and writing docs."
---

Contributions are welcome: adding a grammar, fixing a bug, improving a binding, or writing documentation.

For CI/CD workflow details, see the [CI/CD reference](/contributing/ci/).

## Prerequisites

You'll need the following tools installed:

- [mise](https://mise.jdx.dev/) — the project toolchain and task runner
- Rust stable toolchain via [rustup](https://rustup.rs/)
- Python 3.10+ and [uv](https://docs.astral.sh/uv/)
- Node.js 18+ and [pnpm](https://pnpm.io/)

## Getting started

```bash
# Install mise (macOS)
brew install mise

# Clone the repository
git clone https://github.com/xberg-io/tree-sitter-language-pack.git
cd tree-sitter-language-pack

# Trust the checkout's mise.toml, then install all language dependencies
mise trust
mise run setup

# Build the Rust core
mise run build

# Run all tests
mise run test
```

:::tip[Linux]
:::

On Debian/Ubuntu, install mise with the [official installer](https://mise.jdx.dev/getting-started.html).

## Common tasks

```bash
mise tasks               # show all available tasks
mise run build           # build Rust core + bindings
mise run test            # run all test suites
mise run lint            # run all linters (clippy, ruff, oxlint, rubocop, …)
mise run format          # auto-format all code
mise run e2e:generate    # regenerate e2e test suites from fixtures
mise run go:e2e:test     # run one binding's e2e tests (<lang>:e2e:test)
alef all --clean --skip-snippet-validation --skip-compile  # regenerate the alef-managed bindings and docs
```

Run `mise tasks` to see all available tasks.

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
mise run build

# Run the test suite
mise run test

# Verify the parser works end-to-end
ts-pack download mylang
ts-pack parse example.mylang --language mylang
```

### 4. Add test fixtures

Add at least one fixture under `fixtures/`. Fixtures are the single source for both the e2e
suites and the documentation snippet corpus — `mise run e2e:generate` renders each fixture into
`docs-site/src/snippets/generated/<lang>/<category>/<id>.md` for all 14 bindings. Do not
hand-write snippets; add a fixture instead.

A fixture file holds either a **single JSON object** or an **array** of them, grouped into a
per-category directory — for example `fixtures/process/python_intel.json`. Only `id` and
`description` are required, `additionalProperties` is `false`, the payload goes under `input`,
and `assertions` is a list of typed assertion objects, not a map of booleans. See
`fixtures/schema.json` for the full assertion-type enum.

```json
{
  "id": "mylang_function_process",
  "description": "Intel: extract structure from a mylang function definition",
  "category": "process",
  "tags": ["intel"],
  "input": {
    "source_code": "// example mylang source",
    "config": {
      "language": "mylang"
    }
  },
  "assertions": [
    {
      "type": "equals",
      "field": "language",
      "value": "mylang"
    },
    {
      "type": "count_min",
      "field": "structure",
      "value": 1
    },
    {
      "type": "equals",
      "field": "metrics.error_count",
      "value": 0
    }
  ]
}
```

Then regenerate and run e2e tests:

```bash
mise run e2e:generate
alef test --e2e --jobs 1
```

### 5. Open a pull request

- **Title:** `feat: add <language> parser`
- **Body:** link to the upstream grammar repository, note any quirks or limitations

## Fixing a bug

1. Check the [issue tracker](https://github.com/xberg-io/tree-sitter-language-pack/issues) — the bug may already be reported.
2. Write a **failing test** that reproduces the issue.
3. Fix the bug in the appropriate crate.
4. Confirm all tests pass with `mise run test`.
5. Open a PR with a clear description of the root cause and fix.

## Improving bindings

Binding improvements (better error messages, idiomatic APIs, new methods) are
welcome. Compiled binding crates live under `crates/` (`ts-pack-core-py`, `ts-pack-core-node`,
`ts-pack-core-php`, `ts-pack-core-wasm`, `ts-pack-core-ffi`, `tree-sitter-language-pack-jni`);
the host-language packages they feed live under `packages/`. See the
[Architecture](/concepts/architecture/) page for the full crate layout.

Binding changes must:

- **Not add logic that belongs in the Rust core.** Bindings are pure translation layers.
- **Have test coverage** in the binding's native test suite.
- **Follow the existing API surface** — most binding surfaces are alef-generated; regenerate
  them with `alef all --clean --skip-snippet-validation --skip-compile` rather than hand-editing
  generated files.

## Documentation

Doc fixes and new guides follow the same workflow as code changes:

1. Fork and create a branch.
2. Edit files under `docs-site/src/content/docs/`. Runnable snippets are generated from
   `fixtures/` into `docs-site/src/snippets/generated/<lang>/` — change the fixture and
   rerun `mise run e2e:generate`, never edit a generated snippet.
3. Preview locally with `pnpm --dir docs-site dev` (the site is Astro / Starlight).
4. Run `mise run lint` if you touch any scripted checks.
5. Open a pull request.

:::caution[Generated reference pages]
Everything under `docs-site/src/content/docs/reference/` is alef-generated from the Rust
source. Fix the doc comments upstream and run `alef docs` — do not edit those pages.
:::

:::tip[Quick edits]
:::

Use the **Edit** button in the page header to jump directly from any docs page to the matching file on GitHub.

## Code quality

The project uses pre-commit hooks managed by [prek](https://github.com/xberg-io/prek):

```bash
prek install
prek install --hook-type commit-msg
```

Before committing, verify these three commands pass:

```bash
mise run lint      # zero warnings required
mise run test      # all tests must pass
mise run format    # code must be formatted
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

- [ ] `mise run test` passes
- [ ] `mise run lint` passes (zero warnings)
- [ ] New language has at least one fixture under `fixtures/`, with the regenerated snippets
      under `docs-site/src/snippets/generated/` committed alongside it
- [ ] `mise run e2e:generate && alef test --e2e --jobs 1` passes
- [ ] `mise run version:sync` run if any manifest was bumped
- [ ] PR description explains the change and links related issues

## Getting help

- [GitHub Discussions](https://github.com/xberg-io/tree-sitter-language-pack/discussions) — questions and design conversations
- [Discord](https://discord.gg/xt9WY3GnKR) — real-time chat with maintainers
- [Issue tracker](https://github.com/xberg-io/tree-sitter-language-pack/issues) — bug reports and feature requests
