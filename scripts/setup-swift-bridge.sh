#!/bin/bash
# Setup Swift bridge files after cargo build
# Usage: setup-swift-bridge.sh [debug|release]  (default: release)

set -e

PROFILE="${1:-release}"
BUILD_DIR="target/${PROFILE}/build"

# Find the most recently built output directory
# Use platform-appropriate stat flags: BSD/macOS uses `-f`, GNU/Linux uses `-c`.
# Use a bash array so the format string survives `-exec` word-splitting; a plain
# variable would split `%m %N` into two args and stat would treat `%N` as a
# filename, printing only mtime and erroring on a missing file.
if stat -f '%m %N' "$BUILD_DIR" >/dev/null 2>&1; then
  STAT_FMT=(-f '%m %N')
else
  STAT_FMT=(-c '%Y %n')
fi
OUT=$(find "$BUILD_DIR" -maxdepth 2 -type d -name out -path '*ts-pack-core-swift-*' \
  -exec stat "${STAT_FMT[@]}" {} + 2>/dev/null | sort -rn | head -1 | cut -d' ' -f2-)
if [ -z "$OUT" ]; then
  echo "ERROR: Could not find swift-bridge build output in ${BUILD_DIR}/"
  exit 1
fi

echo "Using swift-bridge output from: $OUT"

# Ensure target directories exist
mkdir -p packages/swift/Sources/RustBridgeC
mkdir -p packages/swift/Sources/RustBridge

# Copy C headers
cat "$OUT/SwiftBridgeCore.h" "$OUT/ts-pack-core-swift/ts-pack-core-swift.h" \
  >packages/swift/Sources/RustBridgeC/RustBridgeC.h

# Copy Swift bridge files with import statement prepended
{
  printf 'import RustBridgeC\n'
  cat "$OUT/SwiftBridgeCore.swift"
} >packages/swift/Sources/RustBridge/SwiftBridgeCore.swift
{
  printf 'import RustBridgeC\n'
  cat "$OUT/ts-pack-core-swift/ts-pack-core-swift.swift"
} >packages/swift/Sources/RustBridge/ts-pack-core-swift.swift

echo "Swift-bridge files setup complete"
