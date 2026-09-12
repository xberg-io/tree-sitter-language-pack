#include "RustBridgeC.h"

// ~keep anchor TU so XCBuild emits RustBridgeC.o (issue #449)
void tree_sitter_language_pack_swift_rust_bridge_c_anchor(void) {}

// ~keep A Swift binary ends up with two tree-sitter C runtimes: SwiftPM compiles tree-sitter
// 0.25.10 as loose objects for SwiftTreeSitter, while our Rust staticlib carries the 0.27
// runtime inside a single archive member. Those loose objects already define every ts_* symbol
// our Rust code needs except `ts_language_is_parseable`, which 0.27 introduced. Resolving that
// one symbol forces the archive member in, and all 253 other runtime symbols then collide at the
// test-executable link (issue #189). Defining it from a loose object keeps the member unreferenced,
// so the binary holds one runtime exactly as it did before 0.27.
//
// The definition is weak: if a future SwiftPM tree-sitter exports its own strong
// `ts_language_is_parseable`, that one wins instead of colliding with this fallback.
//
// Upstream is `return self && self->lex_fn`, and the only code that produces a language with a
// NULL lex_fn sits behind `#ifdef __wasm__` in ts_language_copy_without_callbacks. On the native
// platforms this package targets, the null check alone is therefore exact.
typedef struct TSLanguage TSLanguage;

__attribute__((weak)) bool ts_language_is_parseable(const TSLanguage *self) { return self != NULL; }
