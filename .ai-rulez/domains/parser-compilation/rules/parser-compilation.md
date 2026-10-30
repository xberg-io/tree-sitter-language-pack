---
type: Decision
title: Parser Compilation
paths:
  - "crates/ts-pack-core/build.rs"
  - "crates/ts-pack-core/build_support/**"
  - "grammars/**"
  - "patches/**"
  - "sources/**"
x-ai-rulez:
  kind: rule
  id: parser-compilation
  domain: parser-compilation
  metadata:
    priority: high
---

- Grammars are not in git: `mise run clone` / `mise run clone:if-missing` (`scripts/clone_vendors.py`) fetch every pinned `rev` into gitignored `parsers/`, and `crates/ts-pack-core/build.rs` compiles from there with `cc`. `grammars/` holds only the 8 `local` grammars.
- Pin `rev` to a full 40-hex commit SHA; `scripts/clone_vendors.py` rejects tags and branches. `abi_version` (generation target) defaults to 14; loaded grammars must fall in ABI 13-15.
- `generate: true` runs upstream `grammar.js` under Node with your privileges: review its diff before bumping that pin (`clone_vendors.py` header).
- `TSLP_LANGUAGES` picks the grammars compiled in (`build_support/definitions.rs` `selected_languages`): a comma list, or `all`/`*` for all 372. Unset compiles none on native targets, which load grammars by runtime download, and a curated subset on wasm32. Iterate with `TSLP_LANGUAGES=a,b mise run build`; `mise run test` compiles only its fixed test set, overriding the caller's `TSLP_LANGUAGES`. `TSLP_LINK_MODE` (`build.rs` default `dynamic`) selects linking, but the `rust:build` and `rust:test` tasks pin it to `dynamic`: pass another mode to a direct `cargo` call. `TSLP_ALLOW_FAILED_GRAMMARS=1` is for local debugging only.
- `scanner.c` is compiled when the grammar's `src/` has one. Each parser exports uniquely prefixed C symbols (`c_symbol` overrides the name).
