#!/usr/bin/env bash
# Build ts-pack-core-ffi and stage it where packages/go/binding.go's cgo LDFLAGS look for it.
#
# binding.go links `-L${SRCDIR}/.lib/<platform> -lts_pack_core_ffi`, and `.lib/` is gitignored,
# so any `go build` in packages/go fails with `cannot find -lts_pack_core_ffi` until something
# puts the library there. `alef snippets check` validates Go snippets at compile level, i.e.
# `go build ./...`, so it needs the same artifact — this script is the go snippet session's
# `before` hook in alef.toml, and mirrors ci-e2e.yaml's "Restore FFI to expected paths" step.
#
# TSLP_LANGUAGES is deliberately left unset: an empty grammar selection is what lets this run
# with no `parsers/` tree (build.rs downloads or clones grammar sources only for languages it
# was asked to compile), and TSLP_OFFLINE keeps that path off the network entirely. Snippets
# are only compiled, never executed, so a grammar-free library links exactly the same.
set -euo pipefail

ROOT="$(CDPATH='' cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
CRATE=ts-pack-core-ffi
LIB_STEM=libts_pack_core_ffi

case "$(uname -s)" in
Linux)
  lib_ext=so
  os_label=linux
  ;;
Darwin)
  lib_ext=dylib
  os_label=macos
  ;;
*)
  echo "ERROR: unsupported OS $(uname -s); stage the library manually" >&2
  exit 1
  ;;
esac

case "$(uname -m)" in
x86_64 | amd64) arch_label=x86_64 ;;
arm64 | aarch64) arch_label=$([ "$os_label" = macos ] && echo arm64 || echo aarch64) ;;
*)
  echo "ERROR: unsupported architecture $(uname -m)" >&2
  exit 1
  ;;
esac

PLATFORM="${os_label}-${arch_label}"
DEST="${ROOT}/packages/go/.lib/${PLATFORM}"
BUILT="${ROOT}/target/release/${LIB_STEM}.${lib_ext}"

(
  cd "$ROOT"
  PROJECT_ROOT="$ROOT" TSLP_OFFLINE=1 TSLP_LINK_MODE=dynamic \
    cargo build --locked --release -p "$CRATE"
)

if [ ! -f "$BUILT" ]; then
  echo "ERROR: cargo reported success but $BUILT does not exist" >&2
  exit 1
fi

mkdir -p "$DEST"
cp "$BUILT" "$DEST/"
echo "staged ${LIB_STEM}.${lib_ext} into packages/go/.lib/${PLATFORM}"
