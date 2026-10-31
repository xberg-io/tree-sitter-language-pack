#!/usr/bin/env bash
# Usage: ./generate_checksums.sh <version>

set -euo pipefail

VERSION="${1:?Usage: $0 <version>}"
REPO="xberg-io/tree-sitter-language-pack"
CHECKSUM_FILE="packages/elixir/checksum-Elixir.TreeSitterLanguagePack.Native.exs"

# Must match :targets in packages/elixir/mix.exs and lib/.../native.ex, and the
# build-elixir-nifs matrix in .github/workflows/publish.yaml. A target missing here
# gets no checksum entry, and RustlerPrecompiled then refuses to install it. ~keep
TARGETS=(
  "aarch64-apple-darwin"
  "x86_64-apple-darwin"
  "aarch64-unknown-linux-gnu"
  "x86_64-unknown-linux-gnu"
)

NIF_VERSIONS=("2.16" "2.17")

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

echo "Generating checksums for v${VERSION}..."
echo "Download directory: $TMPDIR"

CURL_OPTS=(-fsSL)
if [[ -n "${GH_TOKEN:-${GITHUB_TOKEN:-}}" ]]; then
  CURL_OPTS+=(-H "Authorization: token ${GH_TOKEN:-${GITHUB_TOKEN}}")
  echo "Using authenticated GitHub API access"
fi

CHECKSUMS=()

for TARGET in "${TARGETS[@]}"; do
  for NIF_VERSION in "${NIF_VERSIONS[@]}"; do
    for PREFIX in "libtree_sitter_language_pack_nif" "libtree-sitter-language-pack-nif"; do
      FILENAME="${PREFIX}-v${VERSION}-nif-${NIF_VERSION}-${TARGET}.so.tar.gz"
      URL="https://github.com/${REPO}/releases/download/v${VERSION}/${FILENAME}"

      echo "Downloading: $FILENAME"
      echo "  URL: $URL"

      DOWNLOADED=false
      for ATTEMPT in 1 2 3 4 5 6 7 8 9 10; do
        if curl "${CURL_OPTS[@]}" -o "${TMPDIR}/${FILENAME}" "$URL"; then
          DOWNLOADED=true
          break
        fi
        echo "  Attempt $ATTEMPT failed, waiting 15s..."
        sleep 15
      done

      if $DOWNLOADED; then
        if command -v sha256sum &>/dev/null; then
          CHECKSUM=$(sha256sum "${TMPDIR}/${FILENAME}" | cut -d' ' -f1)
        elif command -v shasum &>/dev/null; then
          CHECKSUM=$(shasum -a 256 "${TMPDIR}/${FILENAME}" | cut -d' ' -f1)
        else
          echo "ERROR: No sha256sum or shasum command found"
          exit 1
        fi

        echo "  Checksum: sha256:${CHECKSUM}"
        CHECKSUMS+=("  \"${FILENAME}\" => \"sha256:${CHECKSUM}\",")
      else
        echo "  ERROR: Failed to download $FILENAME"
        exit 1
      fi
    done
  done
done

SORTED_CHECKSUMS=()
while IFS= read -r CHECKSUM; do
  SORTED_CHECKSUMS+=("$CHECKSUM")
done < <(printf '%s\n' "${CHECKSUMS[@]}" | sort)

echo "Writing checksum file: $CHECKSUM_FILE"
{
  echo "%{"
  for CHECKSUM in "${SORTED_CHECKSUMS[@]}"; do
    echo "$CHECKSUM"
  done
  echo "}"
} >"$CHECKSUM_FILE"

echo ""
echo "Done! Generated checksums for ${#SORTED_CHECKSUMS[@]} files."
echo ""
echo "Contents of $CHECKSUM_FILE:"
cat "$CHECKSUM_FILE"
