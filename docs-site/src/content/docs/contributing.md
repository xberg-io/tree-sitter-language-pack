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
- Python 3.11+ and [uv](https://docs.astral.sh/uv/)
- Node.js 22+ and [pnpm](https://pnpm.io/)

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
| `generate`   | No       | Regenerate before compiling, except oversized committed parsers |
| `force_generate` | No | With `generate`, require regeneration even for oversized parsers; a timeout fails instead of using committed output |
| `abi_version` | No | Generation ABI target (default `14`); the runtime supports ABI 13–15 |

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

### 5. Add a tags query

Every language must bundle a working `tags.scm` unless `sources/tags_coverage.json` classifies
it. A new grammar with neither fails `every_language_requiring_tags_has_a_fixture`. The
[tags coverage](/tags-coverage/) page lists every language's status; regenerate it with
`mise run docs:generate:tags-coverage`.

- **`not_applicable`**: tags do not apply, because the language declares nothing worth indexing
  (data formats, doc-comment markup, injection-only template layers). The reason names what the
  language has and why none of it is a definition.
- **`grammar_blocked`**: tags apply, but the vendored grammar cannot yield correct names (for
  example, a lexer that folds the whitespace after a keyword into the identifier, or statements
  parsed as one opaque token). The reason names the defect. List it here instead of shipping
  wrong names.

A classified language has no overlay and no fixture, and the test fails once it bundles a tags
query, so the entry is revisited when the grammar changes.

**Query.** Use the grammar's `queries/tags.scm`, or write `query-overlays/<lang>/tags.scm`, which
replaces it at build time, when upstream ships none or a broken one. Write it from the grammar
(`node-types.json`, `grammar.js`, corpus tests), not from another editor's queries. Each pattern
that emits a tag has a role capture and a `@name` capture. Tags are resolved as
tree-sitter-tags resolves them: one tag per `@name` node, from the earliest pattern that matches
it; `@ignore` claims a node without a tag; a match without a role emits nothing; tags come out in
source position order.

**Roles.** Only these roles are allowed:

- `definition.`: `class`, `interface`, `function`, `method`, `constructor`, `module`, `type`,
  `enum`, `constant`, `variable`, `field`, `macro`, `section`, `label`
- `reference.`: `call`, `class`, `type`, `interface`, `implementation`, `module`, `macro`,
  `label`, `variable`, `constant`

Map other kinds onto them: struct, record, object or table to `class`; trait, protocol or
signature to `interface`; typedef or type alias to `type`; enum variant to `constant`; property
or attribute to `field`; namespace, package or schema to `module`; grammar rule, recipe, task or
subroutine to `function`; heading to `section`; anchor or link target to `label`; any send or
invocation to `reference.call`.

**Names.** `@name` is the identifier only: no leading or trailing whitespace, no newline or tab,
not wrapped in quotes or brackets. Capture the string content or identifier child, not the quoted
literal. Keep a sigil only when it is part of the name at every use site (perl `$x`, llvm `@f`).

**Definitions.** Tag each declared entity once: overloads and multi-clause functions tag the first
clause (claim the rest with `@ignore`), and a prototype or forward declaration plus its definition
tag once. Locals, parameters and loop variables are not definitions. Tag references only where
they are precise: calls, type uses, inheritance and imports.

**Fixture.** Add `crates/ts-pack-core/tests/fixtures/tags/<lang>/` with exactly one source file
and `expected.tags`: one `<role>\t<name>` line per tag, in source position order. The source is a
realistic file covering the language's declaration forms, plus things that must not be tagged,
and parses without errors. `TSLP_TAGS_BLESS=1` writes `expected.tags` from the query output;
review every line.

Besides matching `expected.tags`, the test checks each language and reports every violation:

| Check | Fails when |
|---|---|
| `roles` | a tag or a query capture uses a role outside the list above |
| `names` | a name has surrounding whitespace, a newline or tab, or surrounding quotes or brackets |
| `definitions` | the fixture yields no `definition.*` tag |
| `dead_patterns` | a pattern with a role capture yields no tag on the fixture |
| `duplicates` | two definitions with the same role and name start at the same position, have definition nodes under the same parent, or have overlapping definition nodes |

A dead pattern means the fixture misses a form or the pattern can never match: extend the
fixture, or delete the pattern with a comment saying why. When a pattern is right but the fixture
cannot exercise it (the grammar accepts only one declaration per document, for example), list it
under `unexercised_patterns` in `sources/tags_coverage.json` with its normalized text (comments
dropped, whitespace collapsed; the failure message prints it) and a reason:

```json
"unexercised_patterns": {
  "mylang": [{ "pattern": "(alias_annotation (identifier) @name) @definition.type", "reason": "..." }]
}
```

An entry whose pattern fires or no longer exists fails too. The duplicate check allows the same
name under different parents, such as a method `area` in two classes.

```bash
TSLP_LANGUAGES=mylang cargo test -p tree-sitter-language-pack --test tags_coverage
```

`TSLP_TAGS_REPORT=<path>` writes the violations of every built language as JSON, grouped by
check.

### 6. Open a pull request

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
