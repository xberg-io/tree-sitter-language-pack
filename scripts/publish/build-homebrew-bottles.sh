#!/usr/bin/env bash
set -euo pipefail

# Build Homebrew bottles for ts-pack + libts-pack on the current platform.
# Runs `brew install --build-bottle` (which uses the URL blocks in the
# already-published formula) followed by `brew bottle`. Renames bottle
# artifacts to single-dash convention and uploads them to the GitHub release.
# Also saves the bottle JSON manifests to OUT_DIR for the aggregation job to
# merge.
#
# Usage:
#   TAG=v1.8.0-rc.38 VERSION=1.8.0-rc.38 \
#   TAP=kreuzberg-dev/tap \
#   OUT_DIR=/tmp/bottle-json \
#   GH_TOKEN=... \
#   ./build-homebrew-bottles.sh

tag="${TAG:?TAG is required}"
version="${VERSION:?VERSION is required}"
tap="${TAP:?TAP is required (e.g. kreuzberg-dev/tap)}"
out_dir="${OUT_DIR:?OUT_DIR is required}"

mkdir -p "$out_dir"
work_dir="$(mktemp -d)"
trap 'rm -rf "$work_dir"' EXIT
cd "$work_dir"

echo "::group::brew env"
brew --version
brew config | head -20 || true
echo "::endgroup::"

echo "::group::Tap ${tap}"
brew tap "$tap"
brew update --quiet || true
echo "::endgroup::"

build_one_bottle() {
  local formula="$1"
  echo "::group::Building bottle for ${formula}"

  brew uninstall --force "${tap}/${formula}" 2>/dev/null || true

  brew install --build-bottle --verbose "${tap}/${formula}"

  brew bottle --json --no-rebuild "${tap}/${formula}"

  local original_tarball
  shopt -s nullglob
  local tarballs=("${formula}--${version}".*.bottle.tar.gz)
  shopt -u nullglob
  if [[ ${#tarballs[@]} -eq 0 ]]; then
    echo "ERROR: no bottle tarball produced for ${formula}" >&2
    ls -la
    return 1
  fi
  original_tarball="${tarballs[0]}"

  local renamed_tarball="${original_tarball/--/-}"
  if [[ "$renamed_tarball" != "$original_tarball" ]]; then
    cp "$original_tarball" "$renamed_tarball"
  fi

  shopt -s nullglob
  local json_files=("${formula}--${version}".*.bottle.json)
  shopt -u nullglob
  for jf in "${json_files[@]}"; do
    cp "$jf" "$out_dir/"
  done

  echo "Uploading ${renamed_tarball} to release ${tag}"
  gh release upload "$tag" "$renamed_tarball" --clobber --repo kreuzberg-dev/tree-sitter-language-pack

  echo "::endgroup::"
}

build_one_bottle "ts-pack"
build_one_bottle "libts-pack"

echo "Bottles built; JSON manifests saved to ${out_dir}:"
ls -la "$out_dir"
