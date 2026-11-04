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

* Prepare the release with the shared xberg-io/actions Swift workflow, then publish the tag it creates
* Keep committing the checksum after tagging and force-move the core tag
* Publish the checksum only on a separate Swift tag or branch

## Decision Outcome

Chosen option: "Prepare the release with the shared xberg-io/actions Swift workflow, then publish the tag it
creates", because every publisher and every SwiftPM consumer then reads one immutable tag, by the same procedure as
the organization's other Swift packages.

A release takes two dispatches, following the
[prepare-swift-release README](https://github.com/xberg-io/actions/blob/v1/prepare-swift-release/README.md):

1. **Prepare Swift release** (`.github/workflows/prepare-swift-release.yaml`) on the version-bump commit of main.
   It generates the vendored grammar sources like Publish Release does, then calls
   `reusable-swift-release-prepare.yml@v1`, which builds the bundle once with a receipt, pins its checksum with
   alef's version sync, pushes that one commit to main together with the tag in a non-forced atomic push, and
   attaches the ZIP and receipt to a draft release.
2. **Publish Release** on the tag. Its `publish-swift-release` job verifies the tagged manifest against the attached
   ZIP and receipt (`bundle-zip: ""`), builds no bundle, and creates `release/swift/<version>` at the tagged
   commit only when it is absent.

### Consequences

* Good, because the core tag object and its commit never change after creation, on first runs and on retries.
* Good, because `from:`/`exact:` consumers and the legacy Swift branch resolve the same checksummed commit.
* Good, because the pin, verification and recovery logic lives and is tested in xberg-io/actions.
* Bad, because cutting a release needs one more dispatch; Publish Release builds no Swift bundle, so the prepare
  workflow's dry run is the only Swift bundle rehearsal.
* Bad, because main holds the previous release's checksum under the next version's URL between a version bump and
  its preparation; `alef verify` accepts that.

### Confirmation

* `scripts/ci/test_release_dry_mutations.py` rejects any `git tag -f`, forced push or placeholder `sed` in
  `publish.yaml` and `prepare-swift-release.yaml`, pins the caller to `reusable-swift-release-prepare.yml@v1` with
  this repository's inputs, keeps its vendor jobs identical to Publish Release's, and requires the
  `publish-swift-release` job to reuse the release asset.
