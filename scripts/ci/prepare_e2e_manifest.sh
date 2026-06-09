#!/usr/bin/env bash
# Wire a downloaded `e2e-bundles-<platform>` artifact into the runtime by
# emitting a `parsers.json` whose platform URL points at the bundle on disk
# and exporting `TREE_SITTER_LANGUAGE_PACK_MANIFEST_URL` for subsequent steps.
#
# Required env: PLATFORM_LABEL (e.g. linux-x86_64, macos-arm64),
#               BUNDLE_DIR     (directory containing parsers-<label>.tar.zst),
#               GITHUB_WORKSPACE, GITHUB_ENV.
#
# Reads the workspace version from Cargo.toml, then leaves parsers.json next
# to the bundle so the file:// URL the runtime resolves is alongside the
# extracted artifact.
set -euo pipefail

if [ -z "${PLATFORM_LABEL:-}" ]; then
  echo "ERROR: PLATFORM_LABEL is required" >&2
  exit 1
fi
if [ -z "${BUNDLE_DIR:-}" ]; then
  echo "ERROR: BUNDLE_DIR is required" >&2
  exit 1
fi
if [ -z "${GITHUB_WORKSPACE:-}" ]; then
  echo "ERROR: GITHUB_WORKSPACE is required" >&2
  exit 1
fi

BUNDLE_PATH="${BUNDLE_DIR}/parsers-${PLATFORM_LABEL}.tar.zst"
MANIFEST_PATH="${BUNDLE_DIR}/parsers.json"

if [ ! -f "$BUNDLE_PATH" ]; then
  echo "ERROR: bundle missing at $BUNDLE_PATH" >&2
  ls -la "$BUNDLE_DIR" >&2 || true
  exit 1
fi

VERSION=$(python3 -c "
import re, pathlib
for line in pathlib.Path('${GITHUB_WORKSPACE}/Cargo.toml').read_text().splitlines():
    m = re.match(r'\\s*version\\s*=\\s*\"([^\"]+)\"', line)
    if m:
        print(m.group(1)); break
")

if [ -z "$VERSION" ]; then
  echo "ERROR: could not read workspace version" >&2
  exit 1
fi

python3 "${GITHUB_WORKSPACE}/scripts/build_e2e_manifest.py" \
  --version "$VERSION" \
  --platform-label "$PLATFORM_LABEL" \
  --bundle "$BUNDLE_PATH" \
  --definitions "${GITHUB_WORKSPACE}/sources/language_definitions.json" \
  --output "$MANIFEST_PATH"

echo "TREE_SITTER_LANGUAGE_PACK_MANIFEST_URL=file://${MANIFEST_PATH}" >>"${GITHUB_ENV}"
echo "wired e2e manifest: TREE_SITTER_LANGUAGE_PACK_MANIFEST_URL=file://${MANIFEST_PATH}"
