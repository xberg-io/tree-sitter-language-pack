---
status: accepted
date: 2026-10-30
decision-makers: ["Na'aman Hirschfeld"]
---

# Record architecture decisions

## Context and Problem Statement

tree-sitter-language-pack made its lasting choices (how parsers reach users, how grammars are selected, pinned,
licensed and generated, what the WebAssembly package ships) in commits, rules, the changelog and code comments.
Nothing records why a choice was made, what it replaced, or when it changed, so a later change can drift from it
without anyone noticing. The organization keeps its own decisions in the agent-conventions log, which repository logs
cite instead of restating.

## Considered Options

* A repository log of MADR records in `adrs/`, checked by adrlint

No alternatives were recorded.

## Decision Outcome

Chosen option: "A repository log of MADR records in `adrs/`, checked by adrlint", because it follows
[org ADR-0000 (Record architecture decisions)](https://github.com/xberg-io/agent-conventions/blob/main/adrs/0000-record-architecture-decisions.md),
which gives each repository a log for its own decisions and its deviations from an org record.

Records follow MADR 4 as adrlint generates it (`adrlint new`), numbered from 0000. Records 0001 to 0006 are seeds:
decisions already in force, dated to the commit that made them. An accepted record is not edited: a change is a new
record that supersedes or amends it. Org-wide decisions, such as the alef-generated bindings
([org ADR-0002](https://github.com/xberg-io/agent-conventions/blob/main/adrs/0002-rust-cores-with-alef-generated-bindings.md))
and the MIT license
([org ADR-0003](https://github.com/xberg-io/agent-conventions/blob/main/adrs/0003-licensing-line.md)), are cited as
`org ADR-NNNN` and not restated. The index is the docs-site page
`docs-site/src/content/docs/reference/decision-records.md`, outside the log.

### Consequences

* Good, because a later change to one of these decisions has a record to supersede or amend.
* Good, because adrlint checks the shape, the numbering and, once the log is locked, the immutability of every
  record.
* Bad, because a decision now has two homes, the code that applies it and the record that explains it, and they can
  drift.

### Confirmation

`.adrlint.toml` sets `dir = "adrs"` and `start = 0`. `adrlint lint` and `adrlint format --check` exit 0 on `adrs/`,
and the poly hooks `adrlint-lint`, `adrlint-format-check` and `adrlint-lock-check` run them on every commit.

## More Information

* [MADR](https://adr.github.io/madr/)
* [adrlint](https://github.com/Goldziher/adrlint)
