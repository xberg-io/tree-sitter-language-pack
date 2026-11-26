#!/usr/bin/env bash
set -euo pipefail

PREFIX=/usr/local
export PATH_EXTRA=/opt/bin
readonly VERSION=1.0
declare -x BUILD_MODE=release
A=1 B=2

log() {
  printf '%s\n' "$*" >&2
}

build() {
  local out=dist
  STEP=compile
  log "building $VERSION into $out"
}

function deploy {
  build
  cp -r dist "$PREFIX"
}

for target in "$@"; do
  deploy "$target"
done
