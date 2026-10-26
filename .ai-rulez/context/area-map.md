---
type: Concept
title: Area Map
x-ai-rulez:
  kind: context
  id: area-map
  metadata:
    priority: high
---

# Area Map

tree-sitter-language-pack is a Rust grammar pack and code-intelligence library with polyglot bindings.

- `crates/ts-pack-core/` — core library and parser build (`core-layout`, `parser-compilation` rules)
- `crates/ts-pack-cli/` — command-line binary
- `crates/ts-pack-core-{py,node,php,wasm,ffi}/`, `crates/tree-sitter-language-pack-jni/` — binding crates
- `sources/` — language definitions and grammar notices (`language-management` rule)
- `grammars/`, `patches/`, `query-overlays/` — vendored grammars, patches, query overrides
- `packages/<lang>/` — generated language packages
- `e2e/<lang>/`, `fixtures/` — generated e2e suites and fixtures
- `test_apps/<lang>/`, `contract-tests/` — published-package suites, binding contract tests
- `docs-site/`, `templates/` — docs site and templates
- `skills/`, `plugin/` — published skills, agent plugin
- `docker/`, `scripts/` — image, vendoring and CI scripts
