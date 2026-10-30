---
status: accepted
date: 2026-03-24
decision-makers: ["Na'aman Hirschfeld"]
---

# Permissively licensed grammars only

## Context and Problem Statement

The pack redistributes hundreds of third-party grammars, compiled into its packages and parser bundles, under its
own MIT license
([org ADR-0003 (Licensing line)](https://github.com/xberg-io/agent-conventions/blob/main/adrs/0003-licensing-line.md)).
A copyleft grammar would impose license obligations on every downstream user. In March 2026 the pack held two
copyleft grammars, `nim` (MPL-2.0) and `prolog` (AGPL-3.0) (1032c8bd56, 2026-03-24).

## Considered Options

* Accept only permissively licensed grammars, checked in CI by a license linter

No alternatives were recorded.

## Decision Outcome

Chosen option: "Accept only permissively licensed grammars, checked in CI by a license linter", because the policy
"ensures that tree-sitter-language-pack can be freely used in any project — commercial or otherwise — without
imposing license obligations on downstream users" (ca4adf7f05, 2026-03-24).

The same day the policy was documented (ca4adf7f05), `scripts/lint_grammar_licenses.py` was added to check every
grammar against GitHub's license API (34a0981302), and the two copyleft grammars were replaced by MIT and ISC
alternatives (1032c8bd56). Since 0960cd483a (2026-08-12) the linter fails closed: an unknown or unresolvable license
fails the check instead of passing.

### Consequences

* Good, because the pack and its bundles can be used in any project without copyleft obligations.
* Bad, because a language whose only grammar is copyleft cannot be added.
* Bad, because the check calls the GitHub API and needs an authenticated `gh` in CI.

### Confirmation

* `scripts/lint_grammar_licenses.py:38` and `scripts/lint_grammar_licenses.py:54` define the permissive and copyleft
  license sets.
* `mise.toml:101` defines `lint:licenses:check`, which `.github/workflows/ci.yaml:443` runs after refusing to run
  without the definitions, the license cache or an authenticated `gh`.
* `docs-site/src/content/docs/contributing.md:68` states the policy for contributors.
