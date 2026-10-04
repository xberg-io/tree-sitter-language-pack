# Grammar source patches

`parsers/` is `.gitignore`d (`.gitignore:64`) and holds zero tracked files. Grammar
sources are fetched fresh on every build. Editing a vendored `scanner.c` in place is
therefore not a fix: the edit is wiped by the next fetch and never reaches CI or a
release build.

This directory is the durable layer. Each patch is a committed unified diff applied
to the grammar tree before `tree-sitter generate` and compilation.

## Where patches are applied

`patch_grammar_sources` in `crates/ts-pack-core/build_support/patches.rs`, called from `build.rs`'s `main` right
after `ensure_parser_sources` returns and before `apply_msvc_compat_patches`.

That location is deliberate and load-bearing. `ensure_parser_sources` populates
`parsers/` from **four** different sources — a pre-populated workspace tree, the
`OUT_DIR/_parsers` cache, `scripts/clone_vendors.py`, and a downloaded release
tarball — and **the release-tarball path is the one published packages use**. Wiring
the patch layer into `clone_vendors.py`, the intuitive place, would cover exactly one
of the four and ship a fix that reaches no users. All four converge on the single
value `ensure_parser_sources` returns, so patching there covers every path at once.

The applier is a self-contained unified-diff implementation in `build.rs` rather than
a shell-out to `git apply`, because the tarball path exists precisely for
offline/sdist/slim-container installs where a C compiler is guaranteed but `git` is
not. It does git-compatible offset matching with **zero fuzz**, so it is never more
permissive than `git apply`; all patches here were verified to produce byte-identical
output to `git apply -p1`.

A patch that neither applies nor reverse-applies **panics the build**. It is never
skipped and never downgraded to a warning — these patches fix heap buffer overflows,
and a silently skipped one would leave the build green and the fix gone.
`TSLP_SKIP_GRAMMAR_PATCHES=1` exists as an explicit escape hatch and emits a
`cargo:warning` naming the risk.

For the tarball to carry this directory, `patches` must stay in the `paths:` list of
both `pack-source-bundle` steps in `.github/workflows/publish.yaml`.

## Layout

```text
patches/<language>/<name>.patch
```

`<language>` is the key in `sources/language_definitions.json` — the same name as the
`parsers/<language>/` directory, not the upstream repository name. This matters where
one repository supplies several languages: `markdown` and `markdown_inline` are both
cloned from `tree-sitter-grammars/tree-sitter-markdown` at the same revision but
differ by the `directory` field, so they need separate patch directories.

Patch paths are relative to the **grammar root** — the directory that contains `src/`,
i.e. `vendor/<language>/<directory>` when the definition carries a `directory` field
and `vendor/<language>` otherwise. Every patch therefore uses `a/src/scanner.c` /
`b/src/scanner.c` and is applied with `git apply -p1` from that root. Keeping the
`directory` prefix out of the diff means a grammar re-layout upstream does not
invalidate the patch.

Patches within one language directory are applied in sorted filename order.

Two patches in the same directory must not overlap: no patch may carry, as *context*, a
line another patch changes. Overlapping patches apply once and then fail forever — the
build script treats a clean reverse-apply as "already applied", and a patch whose context
was rewritten by its neighbour matches neither forward nor backward. Narrow the context of
the patch that only reads the line; a hunk may carry context on one side only.

## What is in here

Most patches are named `serialize-buffer-overflow.patch` and fix the same class of
bug. Tree-sitter hands an external scanner's `serialize()` a buffer of
exactly `TREE_SITTER_SERIALIZATION_BUFFER_SIZE` (1024) bytes. The only bound check in
the runtime is `ts_assert(length <= TREE_SITTER_SERIALIZATION_BUFFER_SIZE)` in
`ts_parser__external_scanner_serialize`, and `ts_assert` expands to `((void)(e))`
under `NDEBUG` — i.e. in every release build. A scanner that serializes an unbounded
array is a silent heap buffer overflow in production.

The fix in each case is to cap the number of serialized items so the total write
stays within the buffer, and to write the *truncated* count where the format has one
so `deserialize()` reads back a consistent state. Truncation is the correct trade:
a scanner that loses deep-nesting state produces a worse incremental re-parse, not
memory corruption. It is also what upstream generally does — most tree-sitter
scanners in this pack already clamp; these are the ones that do not.

`patches/typst/vec-u32-pop-off-by-one.patch` is the one patch outside that class. Its
`vec_u32_pop` returned `self->vec[self->len--]`, reading one element past the end of the
vector, which is outside the allocation whenever `len` has reached `cap`.

## Regenerating a patch

When a grammar's pinned `rev` moves and its patch stops applying:

1. Fetch the scanner at the new pinned revision:

   ```text
   curl -sfL -o /tmp/scanner_orig.c \
     https://raw.githubusercontent.com/<owner>/<repo>/<rev>/<directory>/src/scanner.c
   ```

2. Copy it to `/tmp/scanner.c` and re-apply the fix by hand. Check first whether
   upstream has fixed it: if the new revision already bounds the write, delete the
   patch directory instead of regenerating.

3. Emit the diff with grammar-root-relative labels:

   ```text
   diff -u --label a/src/scanner.c --label b/src/scanner.c \
     /tmp/scanner_orig.c /tmp/scanner.c > patches/<language>/<name>.patch
   ```

4. Verify it applies to a pristine tree and produces exactly the intended file:

   ```text
   mkdir -p /tmp/check/src && cp /tmp/scanner_orig.c /tmp/check/src/scanner.c
   (cd /tmp/check && git apply --check -p1 <abs-path-to-patch> && git apply -p1 <abs-path-to-patch>)
   diff /tmp/check/src/scanner.c /tmp/scanner.c
   ```

Keep patches as small as the fix allows. Large hunks carry large context and break on
every unrelated upstream edit.

## Failure handling — patches must fail loudly

A patch that fails to apply is a **hard error** that aborts the vendor sync. It is
never skipped, never warned-about-and-continued, and never downgraded to a log line.

This is deliberate. A patch layer that silently no-ops when a grammar bump moves the
context is strictly worse than no patch layer at all: the build stays green, the
release ships, and the overflow is back with nothing indicating it. This repository
already has one instance of that failure mode — `apply_msvc_compat_patches` in
`crates/ts-pack-core/build.rs` does string substitution and silently does nothing when
the target substring is absent — so the rule is stated explicitly rather than assumed.

Two further conditions are also hard errors:

- A directory under `patches/` whose name is not a known language. Otherwise a typo
  produces a patch that is never applied and never reported.
- A `.patch` file that neither applies forward nor applies in reverse. Reverse-applying
  cleanly is the "already applied" signal that makes re-running the clone idempotent;
  anything else means the tree is not in a state the patch understands.

## Verification status

`markdown` is verified end to end: an AddressSanitizer harness linking the real
scanner and the real tree-sitter runtime reproduces the heap overflow on a 256-byte
input, shows the patched scanner capping at 1021 bytes with no ASan report, and shows
the patched and unpatched scanners producing byte-identical parse trees (full,
incremental, and fresh) on ordinary markdown.

The other patches are verified statically: the local scanner is byte-identical to
upstream at the pinned revision, the bound is derived by inspection, and both the
original and patched sources compile cleanly. They do not yet have a runtime
reproducer.
