---
status: proposed
date: 2026-11-04
decision-makers: ["Na'aman Hirschfeld"]
---

# Tag the release after pinning the Swift bundle checksum

## Context and Problem Statement

SwiftPM consumers resolve `from:`/`exact:` versions to the core `v<version>` tag, so that tag's root `Package.swift`
must carry the artifact bundle's checksum. The publish workflow built the bundle after the tag existed, committed the
checksum and force-moved the core tag and `release/swift/<version>` while other registries were already publishing
from the original tag ([issue 215](https://github.com/xberg-io/tree-sitter-language-pack/issues/215)).

## Considered Options

* Build the bundle in the dry run, pin its checksum on main and tag that commit before publishing
* Keep committing the checksum after tagging and force-move the core tag
* Publish the checksum only on a separate Swift tag or branch

## Decision Outcome

Chosen option: "Build the bundle in the dry run, pin its checksum on main and tag that commit before publishing",
because every publisher and every SwiftPM consumer then reads one immutable tag.

The Publish Release dry run on the version-bump commit is the only run that builds the bundle; its receipt
(`build-swift-artifactbundle` `receipt-version`) binds those bytes to that commit. `release-prepare.yaml` takes that
run's ZIP and receipt, runs `alef sync-versions --regen` to pin the checksum, accepts only a one-line Package.swift
change that holds the ZIP's SHA-256 and the tag's download URL, pushes that commit to main and creates the tag in one
atomic non-forced push, and attaches the ZIP and receipt to the draft release. The real publish run does not rebuild
the bundle (the ZIP is not reproducible); `xberg-io/actions/publish-swift-release@v1` verifies the tagged manifest
against the release asset and creates `release/swift/<version>` at the tagged commit only when it is absent.

### Consequences

* Good, because the core tag object and its commit never change after creation, on first runs and on retries.
* Good, because `from:`/`exact:` consumers and the legacy Swift branch resolve the same checksummed commit.
* Bad, because cutting a release needs a successful dry run of the current main commit and one more dispatch.
* Bad, because main holds the previous release's checksum under the next version's URL between a version bump and
  its preparation; `alef verify` accepts that.

### Confirmation

* `scripts/ci/test_release_dry_mutations.py` rejects any `git tag -f`, forced push or placeholder `sed` in
  `publish.yaml` and `release-prepare.yaml`, requires the dry-run-only bundle build with its receipt and the
  `publish-swift-release` job, and runs the preparation steps against a disposable remote: the tag lands on the
  pinned child of the dry run commit, a rerun leaves the tag object and main unchanged, and an unrelated existing
  tag, a moved main, a wrong checksum, URL or extra edit, and a bundle that differs from its receipt are rejected
  before any push.
