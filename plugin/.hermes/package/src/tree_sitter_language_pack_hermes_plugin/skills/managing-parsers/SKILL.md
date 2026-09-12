---
name: managing-parsers
description: >-
  Use when the user needs to manage the tree-sitter parser cache —
  prefetch parsers for offline or CI runs, list what is downloaded,
  inspect a language, find the cache directory, or clean it. Covers
  `ts-pack download`, `list`, `info`, `cache-dir`, `clean`, and `init`.
---

<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:53d41dddeaa1a575e46096a98399ceeac7825a5ee9fef412f92b032680924b3c
Source-Hash: blake3:08f124151839f7172a3a1d5dc141d6dba3e827f8067d64dce95eca8b1df89d50
Schema-Version: v1
-->

# Managing parsers

Parser libraries download on demand the first time a language is used and
are cached on disk. Manage that cache explicitly for offline work, CI
prefetch, reproducible builds, or to reclaim space.

> Note: parser cache management (`download`, `clean`, `init`) is done via the
> `ts-pack` CLI directly. The OpenCode tools expose only parse/process/info.

## Prefetch (offline / CI)

```bash
ts-pack download python rust typescript   # specific languages
ts-pack download --all                    # every available language
ts-pack download --groups all             # by group
ts-pack download python --fresh           # clean cache first, then download
```

The manifest currently defines exactly one group, `all`. Group names are
manifest data, not a fixed list — enumerate them with `manifest_groups()`
rather than hardcoding. Run `download` in your CI setup step so later
`parse`/`process` calls hit the cache instead of the network.

## Inspect

```bash
ts-pack list                 # all available languages
ts-pack list --downloaded    # only languages currently cached
ts-pack list --manifest      # everything in the remote manifest
ts-pack list --filter script # filter by name substring

ts-pack info python          # known? downloaded? cache path?
ts-pack cache-dir            # print the effective cache directory
```

## Clean

```bash
ts-pack clean           # prompts for confirmation
ts-pack clean --force   # skip the prompt (use in scripts/CI)
```

`clean` removes all cached parser libraries; they re-download on next use.

## Config file (init)

`ts-pack init` writes a `language-pack.toml` to the current directory
pinning a cache location and/or a language set. This **writes a file** to
the working directory.

```bash
ts-pack init --languages python,rust --cache-dir .ts-pack-cache
```

Resulting `language-pack.toml`:

```toml
cache_dir = ".ts-pack-cache"
languages = ["python", "rust"]
```

With no flags, `init` writes a commented template. When `--languages` is given,
`init` also triggers the download for those languages. (`init` takes only
`--languages` and `--cache-dir` — for group-based prefetch use
`ts-pack download --groups <names>`.) Commit `language-pack.toml` to pin the
parser set across a team or CI.

## Typical CI flow

```bash
ts-pack init --languages python,rust,typescript --cache-dir .ts-pack-cache
# ... later steps run parse/process offline against the warm cache ...
ts-pack list --downloaded   # assert the expected parsers are present
```

## When to reach for the other skills

Once parsers are cached, use `parsing-source` to get syntax trees and
`extracting-code-structure` to pull metadata. `info`/`list` here also serve
the language-support checks described in `detecting-languages`.
