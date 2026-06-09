dnl Configuration for Rust-based PHP extension via ext-php-rs.
dnl Allows phpize to recognize this extension during source compilation (PIE fallback).

PHP_ARG_ENABLE([tree_sitter_language_pack],
  [whether to enable the tree_sitter_language_pack extension],
  [AS_HELP_STRING([--enable-tree_sitter_language_pack],
    [Enable tree_sitter_language_pack extension support])],
  [yes])

if test "$PHP_TREE_SITTER_LANGUAGE_PACK_ENABLED" = "yes"; then
  dnl Register the extension directory so phpize creates modules/ and sets up build rules.
  PHP_NEW_EXTENSION(tree_sitter_language_pack, [], $ext_shared)

  dnl Invoke cargo build to compile the Rust FFI library and copy it to modules/.
  AC_CONFIG_COMMANDS([cargo-build], [
    if test -f "crates/tree-sitter-language-pack-php/Cargo.toml"; then
      (cd crates/tree-sitter-language-pack-php && cargo build --release) || exit 1

      dnl Detect output filename based on platform
      if test -f "crates/tree-sitter-language-pack-php/target/release/libtree-sitter-language-pack_php.dylib"; then
        cargo_lib="crates/tree-sitter-language-pack-php/target/release/libtree-sitter-language-pack_php.dylib"
      elif test -f "crates/tree-sitter-language-pack-php/target/release/libtree-sitter-language-pack_php.so"; then
        cargo_lib="crates/tree-sitter-language-pack-php/target/release/libtree-sitter-language-pack_php.so"
      else
        echo "ERROR: cargo build succeeded but .so/.dylib not found in crates/tree_sitter_language_pack-php/target/release" >&2
        exit 1
      fi

      mkdir -p modules
      cp "$cargo_lib" "modules/tree-sitter-language-pack.so" || exit 1
    else
      echo "ERROR: crates/tree_sitter_language_pack-php/Cargo.toml not found" >&2
      exit 1
    fi
  ], [])
fi
