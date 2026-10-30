---
status: accepted
date: 2026-08-12
decision-makers: ["Na'aman Hirschfeld"]
---

# Grammar pins validated as commit SHAs on allowlisted forges

## Context and Problem Statement

Grammars are not committed: the vendoring scripts clone each upstream repository named in
`sources/language_definitions.json` at its pinned `rev` and compile from there. The `repo`, `branch` and `rev` values
are passed to git verbatim, and git parses a leading `-` as an option. Until 2026-08-12 these values were not
validated, and `check_grammar_updates.py` ran "a third unvalidated git ls-remote" (0960cd483a, 2026-08-12). All 363
remote entries already pinned a 40-hex SHA by convention.

## Considered Options

* Validate every pin before any git call: `rev` must be a full 40-hex lowercase commit SHA and `repo` an `https` URL
  on `github.com` or `gitlab.com`, from one shared module
* Pass the definition values to git unvalidated (the state before 0960cd483a)

## Decision Outcome

Chosen option: "Validate every pin before any git call: `rev` must be a full 40-hex lowercase commit SHA and `repo`
an `https` URL on `github.com` or `gitlab.com`, from one shared module", because "a poisoned definition aborts before
any clone runs" (0960cd483a), and a full SHA makes a refresh reproducible: "the same pins fetch the same bytes"
(`scripts/clone_vendors.py:8`).

`scripts/_vendor_sources.py` holds the allowlist and the validators, used by `clone_vendors.py`, `pin_vendors.py` and
`check_grammar_updates.py`. `clone_vendors.py` validates at load time and again in `clone_repository`. Tags and
branch names are rejected as a `rev`.

### Consequences

* Good, because a malformed or hostile definition cannot inject git options or fetch from an unexpected host.
* Good, because the same pins always fetch the same grammar sources.
* Bad, because a grammar hosted on any other forge cannot be added without widening the allowlist.
* Bad, because a pin is reproducible, not reviewed: a pinned commit can still contain anything, so each pin bump
  needs review (`scripts/clone_vendors.py:9`).

### Confirmation

* `scripts/_vendor_sources.py:17` defines `ALLOWED_REPO_HOSTS`, `scripts/_vendor_sources.py:23` the SHA pattern, and
  `scripts/_vendor_sources.py:56` `validate_rev`.
* `scripts/clone_vendors.py:377` and `scripts/clone_vendors.py:381` revalidate the URL and the rev before cloning.
* The `language-management` rule in `.ai-rulez/domains/language-management/rules/language-management.md` requires
  `rev` to be a full 40-hex commit SHA.
