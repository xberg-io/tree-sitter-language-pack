#!/usr/bin/env bash
set -euo pipefail

# Update Homebrew formula files with new version + SHA256s for the GitHub release.
# - ts-pack: source-build formula (downloads the GH source archive, builds via cargo)
# - libts-pack: pre-built FFI library tarballs per platform (4 SHAs)
#
# Usage:
#   TAG=v1.8.0-rc.38 VERSION=1.8.0-rc.38 \
#   TAP_DIR=/path/to/homebrew-tap \
#   ./update-homebrew-formula.sh

tag="${TAG:?TAG is required (e.g. v1.8.0-rc.38)}"
version="${VERSION:?VERSION is required (e.g. 1.8.0-rc.38)}"
tap_dir="${TAP_DIR:?TAP_DIR is required (path to homebrew-tap checkout)}"

cli_formula="${tap_dir}/Formula/ts-pack.rb"
ffi_formula="${tap_dir}/Formula/libts-pack.rb"

[[ -f "$cli_formula" ]] || {
  echo "Missing $cli_formula" >&2
  exit 1
}
[[ -f "$ffi_formula" ]] || {
  echo "Missing $ffi_formula" >&2
  exit 1
}

work_dir="$(mktemp -d)"
trap 'rm -rf "$work_dir"' EXIT

compute_sha() {
  local asset="$1"
  echo "Downloading $asset..." >&2
  gh release download "$tag" -p "$asset" -D "$work_dir" --clobber
  shasum -a 256 "$work_dir/$asset" | awk '{print $1}'
}

# Source archive SHA: GitHub auto-generates the source tarball at
# /archive/${tag}.tar.gz. Download via curl since `gh release download` only
# fetches uploaded assets.
download_source_sha() {
  local url="https://github.com/kreuzberg-dev/tree-sitter-language-pack/archive/${tag}.tar.gz"
  echo "Downloading source archive from $url..." >&2
  curl -fsSL "$url" -o "$work_dir/source.tar.gz"
  shasum -a 256 "$work_dir/source.tar.gz" | awk '{print $1}'
}

# CLI: only the source archive (cargo install builds from it)
cli_source_sha=$(download_source_sha)

# FFI: 4 prebuilt tarballs
ffi_macos_arm_sha=$(compute_sha "tree-sitter-language-pack-ffi-${tag}-aarch64-apple-darwin.tar.gz")
ffi_macos_intel_sha=$(compute_sha "tree-sitter-language-pack-ffi-${tag}-x86_64-apple-darwin.tar.gz")
ffi_linux_arm_sha=$(compute_sha "tree-sitter-language-pack-ffi-${tag}-aarch64-unknown-linux-gnu.tar.gz")
ffi_linux_intel_sha=$(compute_sha "tree-sitter-language-pack-ffi-${tag}-x86_64-unknown-linux-gnu.tar.gz")

write_cli_formula() {
  cat >"$cli_formula" <<EOF
# typed: false
# frozen_string_literal: true

class TsPack < Formula
  desc "Tree-sitter language pack CLI - download and manage 305 parser grammars"
  homepage "https://github.com/kreuzberg-dev/tree-sitter-language-pack"
  version "${version}"
  url "https://github.com/kreuzberg-dev/tree-sitter-language-pack/archive/v#{version}.tar.gz"
  sha256 "${cli_source_sha}"
  license any_of: ["MIT", "Apache-2.0"]

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: "crates/ts-pack-cli")
  end

  test do
    assert_match "ts-pack", shell_output("#{bin}/ts-pack --help")
  end
end
EOF
}

write_ffi_formula() {
  cat >"$ffi_formula" <<EOF
# typed: false
# frozen_string_literal: true

class LibtsPack < Formula
  desc "C library for tree-sitter language pack (FFI bindings)"
  homepage "https://github.com/kreuzberg-dev/tree-sitter-language-pack"
  version "${version}"
  license any_of: ["MIT", "Apache-2.0"]

  on_macos do
    on_arm do
      url "https://github.com/kreuzberg-dev/tree-sitter-language-pack/releases/download/v#{version}/tree-sitter-language-pack-ffi-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "${ffi_macos_arm_sha}"
    end

    on_intel do
      url "https://github.com/kreuzberg-dev/tree-sitter-language-pack/releases/download/v#{version}/tree-sitter-language-pack-ffi-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "${ffi_macos_intel_sha}"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/kreuzberg-dev/tree-sitter-language-pack/releases/download/v#{version}/tree-sitter-language-pack-ffi-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "${ffi_linux_arm_sha}"
    end

    on_intel do
      url "https://github.com/kreuzberg-dev/tree-sitter-language-pack/releases/download/v#{version}/tree-sitter-language-pack-ffi-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "${ffi_linux_intel_sha}"
    end
  end

  def install
    include.install Dir["include/*.h"]

    if OS.mac?
      lib.install Dir["lib/*.dylib"]
    elsif OS.linux?
      lib.install Dir["lib/*.so"]
    end
    lib.install Dir["lib/*.a"]

    (lib / "pkgconfig").install "share/pkgconfig/tree-sitter-language-pack.pc"

    inreplace lib / "pkgconfig/tree-sitter-language-pack.pc", /prefix=.*/, "prefix=#{prefix}"

    (lib / "cmake/tree-sitter-language-pack").install Dir["lib/cmake/tree-sitter-language-pack/*"]
  end

  test do
    (testpath / "test.c").write <<~C
      #include <ts_pack.h>
      #include <stdio.h>
      int main(void) {
          /* Smoke: verify a known symbol resolves at link time. */
          (void) ts_pack_available_languages;
          printf("ts_pack OK\\n");
          return 0;
      }
    C

    system ENV.cc, "test.c", "-o", "test",
           "-I#{include}", "-L#{lib}", "-lts_pack_core_ffi"
    assert_match "ts_pack OK", shell_output("./test")
  end
end
EOF
}

write_cli_formula
write_ffi_formula

echo "Updated formulas:"
echo "  $cli_formula"
echo "  $ffi_formula"
