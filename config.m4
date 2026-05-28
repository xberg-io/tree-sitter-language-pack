dnl Configuration for Rust-based PHP extension via ext-php-rs.
dnl Allows phpize to recognize this extension during source compilation (PIE fallback).

PHP_ARG_ENABLE([tree_sitter_language_pack],
  [whether to enable the tree_sitter_language_pack extension],
  [AS_HELP_STRING([--enable-tree_sitter_language_pack],
    [Enable tree_sitter_language_pack extension support])],
  [yes])

if test "$PHP_TREE_SITTER_LANGUAGE_PACK_ENABLED" = "yes"; then
  dnl Recognize the extension directory for phpize/make
  PHP_NEW_EXTENSION(tree_sitter_language_pack, [], $ext_shared)
fi
