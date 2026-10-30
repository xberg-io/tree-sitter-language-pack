---
status: accepted
date: 2026-06-19
decision-makers: ["Na'aman Hirschfeld"]
---

# Generate grammars at tree-sitter ABI 14

## Context and Problem Statement

`get_language` hands the host ecosystem's own tree-sitter `Language` type to the caller, so a grammar must load in the
host's tree-sitter runtime, not only in the pack's. 126 grammars shipped upstream `parser.c` at ABI 15, "which host
tree-sitter runtimes capped at ABI 14 (e.g. go-tree-sitter v0.24.0, max ABI 14) reject", which broke that
passthrough on those bindings (#143; 11a52b2769, 2026-06-19).

## Considered Options

* Regenerate grammars with `tree-sitter generate --abi 14`, with `abi_version` 14 as the default generation target
* Ship each grammar's upstream `parser.c` at whatever ABI upstream generated (the state before 11a52b2769)

## Decision Outcome

Chosen option: "Regenerate grammars with `tree-sitter generate --abi 14`, with `abi_version` 14 as the default
generation target", because ABI 14 loads in the host runtimes that `get_language` passthrough targets, while upstream
ABI 15 output was rejected by them (11a52b2769).

`clone_vendors.py` runs `tree-sitter generate --abi <abi_version>` for entries marked `generate: true`, and
`abi_version` defaults to 14. A grammar that cannot be regenerated keeps its committed `parser.c`, which must fall
within ABI 13 to 15. At 11a52b2769 only `perl` stayed at ABI 15, because upstream ships no `grammar.js`; the
architecture page now lists 16 such grammars, whose committed `parser.c` is too large to regenerate.

### Consequences

* Good, because the pack's grammars load in host runtimes from tree-sitter 0.21 through 0.26.
* Bad, because a regenerated grammar runs upstream `grammar.js` under Node at refresh time, which is a supply-chain
  exposure that each pin bump must review.
* Bad, because the ABI 15 exceptions still require a host runtime of 0.25 or later.

### Confirmation

* `scripts/clone_vendors.py:785` defaults `abi_version` to 14, and `scripts/clone_vendors.py:604` passes it to
  `tree-sitter generate --abi`.
* `scripts/clone_vendors.py:98` sets the minimum loadable ABI to 13, and `scripts/clone_vendors.py:617` accepts a
  committed `parser.c` only within that range.
* `docs-site/src/content/docs/concepts/architecture.md:130` documents ABI 14 and the ABI 15 exceptions.
