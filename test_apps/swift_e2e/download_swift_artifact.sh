#!/bin/bash
set -euo pipefail

# Download the Swift artifact bundle and compute its checksum.
# SwiftPM requires a stable SHA256 checksum for binary targets.
# Cache is validated against the expected checksum in Package.swift to detect
# version mismatches (e.g., when upgrading from rc.49 to rc.50, the filename
# stays the same but the URL changes and the cached zip becomes stale).

ARTIFACT_URL="https://github.com/xberg-io/tree-sitter-language-pack/releases/download/v1.9.0-rc.17/TreeSitterLanguagePack-rs.artifactbundle.zip"
ARTIFACT_FILE="TreeSitterLanguagePack-rs.artifactbundle.zip"
PACKAGE_SWIFT="Package.swift"

# Extract the expected checksum from Package.swift.
# Look for the pattern: checksum: "0123456789abcdef..."
EXPECTED_CHECKSUM=$(grep -oE 'checksum:\s+"[a-f0-9]{64}"' "$PACKAGE_SWIFT" | head -1 | grep -oE '[a-f0-9]{64}' || true)

# Determine whether to use or invalidate the cache.
SHOULD_DOWNLOAD=true
if [ -f "$ARTIFACT_FILE" ]; then
  if [ -n "$EXPECTED_CHECKSUM" ]; then
    # Cache exists and we know the expected checksum: validate before reusing.
    ACTUAL_CHECKSUM=$(swift package compute-checksum "$ARTIFACT_FILE")
    if [ "$EXPECTED_CHECKSUM" = "$ACTUAL_CHECKSUM" ]; then
      echo "Using cached artifact (checksum validated): $ARTIFACT_FILE"
      SHOULD_DOWNLOAD=false
    else
      echo "Cached artifact checksum mismatch (expected: $EXPECTED_CHECKSUM, got: $ACTUAL_CHECKSUM)"
      echo "Removing stale cache and re-downloading"
      rm -f "$ARTIFACT_FILE"
    fi
  else
    # Expected checksum not yet resolved (placeholder not substituted): assume cache is stale
    echo "Unable to extract expected checksum from $PACKAGE_SWIFT; invalidating cache"
    rm -f "$ARTIFACT_FILE"
  fi
fi

# Download if needed
if [ "$SHOULD_DOWNLOAD" = true ]; then
  echo "Downloading Swift artifact from $ARTIFACT_URL"
  curl -fsSL -o "$ARTIFACT_FILE" "$ARTIFACT_URL"
fi

# Compute SHA256 checksum
CHECKSUM=$(swift package compute-checksum "$ARTIFACT_FILE")
echo "Computed checksum: $CHECKSUM"

# Substitute the placeholder checksum in Package.swift
sed -i.bak "s/__ALEF_SWIFT_CHECKSUM__/$CHECKSUM/g" "$PACKAGE_SWIFT"
rm -f "${PACKAGE_SWIFT}.bak"

echo "Updated $PACKAGE_SWIFT with checksum"
