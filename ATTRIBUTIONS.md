# Attributions

## text-splitter

- **Author**: Ben Brandt
- **License**: MIT
- **Repository**: <https://github.com/benbrandt/text-splitter>
- **Version referenced**: 0.29.x (tree-sitter 0.26 support)

The code-splitting algorithm in `crates/ts-pack-core/src/text_splitter.rs` is
vendored and adapted from the `text-splitter` crate's `code` feature. The
original implementation uses tree-sitter's AST structure to find semantic split
points in source code, preferring boundaries at top-level declarations
(functions, classes) and falling back to statement boundaries, line boundaries,
and finally raw byte splits.

Our vendored version simplifies the algorithm by:

- Removing the generic `ChunkSizer` / `ChunkConfig` abstractions in favour of a
  plain `max_chunk_size: usize` byte-count parameter.
- Removing Unicode segmentation fallbacks (grapheme, word, sentence) since we
  operate on source code where line and byte boundaries are sufficient.
- Returning `Vec<(usize, usize)>` byte ranges instead of string slices, making
  the output easier to use with downstream systems that work with offsets.
- Targeting tree-sitter 0.26 directly without version compatibility shims.

The original `text-splitter` MIT license permits this usage. Full license text
is available at <https://github.com/benbrandt/text-splitter/blob/main/LICENSE>.
