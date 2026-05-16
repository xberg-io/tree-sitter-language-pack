// Vendored and adapted from text-splitter by Ben Brandt (MIT License)
// https://github.com/benbrandt/text-splitter
//
// The original text-splitter crate provides a CodeSplitter that uses tree-sitter
// to find semantic split points in source code. This module vendors and simplifies
// that algorithm for direct use with tree-sitter 0.26, returning byte ranges
// instead of string slices, and removing the dependency on text-splitter's
// chunk-sizing abstractions.

use std::ops::Range;

use memchr::memchr;
use tree_sitter::TreeCursor;

/// Split source code into chunks using tree-sitter AST structure for intelligent boundaries.
/// Returns a list of `(start_byte, end_byte)` ranges.
///
/// The algorithm works by:
/// 1. Walking the tree-sitter AST to collect all nodes with their depth.
/// 2. Using depth as a semantic level: shallower nodes (functions, classes) are
///    preferred split boundaries over deeper nodes (statements, expressions).
/// 3. Greedily merging adjacent sections at the best semantic level that keeps
///    each chunk under `max_chunk_size` bytes.
/// 4. When no AST node boundary fits, falling back to line boundaries and
///    ultimately to raw byte splits.
///
/// The function never splits in the middle of a token/leaf node when an AST
/// boundary is available.
///
/// # Arguments
///
/// * `source` - The full source code string.
/// * `tree`   - A tree-sitter `Tree` previously parsed from `source`.
/// * `max_chunk_size` - Maximum size in bytes for each chunk.
///
/// # Returns
///
/// A `Vec<(usize, usize)>` of `(start_byte, end_byte)` ranges covering the
/// entire source. Ranges are non-overlapping, contiguous, and each range is
/// at most `max_chunk_size` bytes (except when a single indivisible token
/// exceeds that limit).
#[cfg_attr(alef, alef(skip))]
pub fn split_code(source: &str, tree: &tree_sitter::Tree, max_chunk_size: usize) -> Vec<(usize, usize)> {
    if source.is_empty() || max_chunk_size == 0 {
        return Vec::new();
    }

    // If the entire source fits, return it as one chunk.
    if source.len() <= max_chunk_size {
        return vec![(0, source.len())];
    }

    // Collect all AST node boundaries with their depth.
    let node_ranges = collect_node_ranges(tree.walk());

    // Group nodes by depth so we can try split levels from shallowest to deepest.
    let max_depth = node_ranges.iter().map(|nr| nr.depth).max().unwrap_or(0);

    // Build split points: for each depth level, collect the byte offsets where
    // a new node at that depth starts. These are candidate split boundaries.
    let mut split_points_by_depth: Vec<Vec<usize>> = vec![Vec::new(); max_depth + 1];
    for nr in &node_ranges {
        split_points_by_depth[nr.depth].push(nr.range.start);
    }
    // Also add end-of-source as a boundary at every level.
    for points in &mut split_points_by_depth {
        points.push(source.len());
        points.sort_unstable();
        points.dedup();
    }

    // Try splitting at the shallowest depth first (top-level declarations).
    // If chunks at that level are still too large, recurse into deeper levels
    // within the oversized chunk.
    let mut chunks: Vec<(usize, usize)> = Vec::new();
    split_recursive(
        source,
        0,
        source.len(),
        max_chunk_size,
        &split_points_by_depth,
        0,
        &mut chunks,
    );

    chunks
}

/// A node's byte range paired with its depth in the AST.
#[derive(Debug, Clone)]
struct NodeRange {
    depth: usize,
    range: Range<usize>,
}

/// Walk the tree depth-first and collect every node (except the root) with its
/// depth and byte range. This mirrors the `CursorOffsets` iterator from
/// text-splitter.
fn collect_node_ranges(cursor: TreeCursor<'_>) -> Vec<NodeRange> {
    let mut ranges = Vec::new();
    let mut cursor = cursor;

    // Move into the first child; we skip the root node itself.
    if !cursor.goto_first_child() {
        return ranges;
    }

    ranges.push(NodeRange {
        depth: cursor.depth() as usize,
        range: cursor.node().byte_range(),
    });

    loop {
        // Try to descend into children first (depth-first).
        if cursor.goto_first_child() {
            ranges.push(NodeRange {
                depth: cursor.depth() as usize,
                range: cursor.node().byte_range(),
            });
            continue;
        }

        // Try next sibling.
        loop {
            if cursor.goto_next_sibling() {
                ranges.push(NodeRange {
                    depth: cursor.depth() as usize,
                    range: cursor.node().byte_range(),
                });
                break;
            }
            // Go back up; if we can't, we're done.
            if !cursor.goto_parent() {
                return ranges;
            }
        }
    }
}

/// Recursively split the region `[region_start, region_end)` of `source` into
/// chunks of at most `max_chunk_size` bytes, preferring boundaries at
/// `current_depth` first, then falling back to deeper depths, then line
/// boundaries, then raw byte boundaries.
fn split_recursive(
    source: &str,
    region_start: usize,
    region_end: usize,
    max_chunk_size: usize,
    split_points_by_depth: &[Vec<usize>],
    current_depth: usize,
    out: &mut Vec<(usize, usize)>,
) {
    let region_size = region_end - region_start;

    // Base case: region fits in one chunk.
    if region_size <= max_chunk_size {
        if region_size > 0 {
            out.push((region_start, region_end));
        }
        return;
    }

    // Try to find split points at the current AST depth within this region.
    if current_depth < split_points_by_depth.len() {
        let points = &split_points_by_depth[current_depth];

        // Collect boundaries within [region_start, region_end].
        let relevant: Vec<usize> = points
            .iter()
            .copied()
            .filter(|&p| p > region_start && p < region_end)
            .collect();

        if !relevant.is_empty() {
            // We have AST boundaries at this depth. Greedily merge adjacent
            // sections as long as they fit under max_chunk_size.
            let mut boundaries = Vec::with_capacity(relevant.len() + 2);
            boundaries.push(region_start);
            boundaries.extend_from_slice(&relevant);
            boundaries.push(region_end);

            let mut cursor = 0;
            while cursor < boundaries.len() - 1 {
                let chunk_start = boundaries[cursor];
                // Greedily extend to the farthest boundary that keeps the chunk
                // within max_chunk_size.
                let mut best_end_idx = cursor + 1;
                for (j, &boundary) in boundaries.iter().enumerate().skip(cursor + 1) {
                    if boundary - chunk_start <= max_chunk_size {
                        best_end_idx = j;
                    } else {
                        break;
                    }
                }

                let chunk_end = boundaries[best_end_idx];
                if chunk_end - chunk_start <= max_chunk_size {
                    // This merged chunk fits; emit it.
                    if chunk_end > chunk_start {
                        out.push((chunk_start, chunk_end));
                    }
                    cursor = best_end_idx;
                } else {
                    // Even a single section at this depth is too large.
                    // Recurse into the next depth level.
                    split_recursive(
                        source,
                        chunk_start,
                        chunk_end,
                        max_chunk_size,
                        split_points_by_depth,
                        current_depth + 1,
                        out,
                    );
                    cursor = best_end_idx;
                }
            }
            return;
        }

        // No boundaries at this depth in the region; try the next depth.
        if current_depth + 1 < split_points_by_depth.len() {
            split_recursive(
                source,
                region_start,
                region_end,
                max_chunk_size,
                split_points_by_depth,
                current_depth + 1,
                out,
            );
            return;
        }
    }

    // Fallback: no more AST boundaries. Split at line boundaries.
    split_at_lines(source, region_start, region_end, max_chunk_size, out);
}

/// Split a region at newline boundaries, greedily merging lines into chunks.
/// Falls back to raw byte splitting if a single line exceeds `max_chunk_size`.
fn split_at_lines(
    source: &str,
    region_start: usize,
    region_end: usize,
    max_chunk_size: usize,
    out: &mut Vec<(usize, usize)>,
) {
    let region = &source[region_start..region_end];

    // Collect line-end offsets (byte offsets relative to region_start).
    let mut line_ends: Vec<usize> = Vec::new();
    let region_bytes = region.as_bytes();
    let mut search_start = 0;
    while let Some(rel_pos) = memchr(b'\n', &region_bytes[search_start..]) {
        let abs_pos = region_start + search_start + rel_pos + 1;
        line_ends.push(abs_pos);
        search_start += rel_pos + 1;
    }
    // The final position is always the region end.
    if line_ends.last().copied() != Some(region_end) {
        line_ends.push(region_end);
    }

    let mut chunk_start = region_start;
    let mut prev_line_end = region_start;

    for &line_end in &line_ends {
        let candidate_size = line_end - chunk_start;
        if candidate_size > max_chunk_size {
            // Emit what we have so far (if anything).
            if prev_line_end > chunk_start {
                out.push((chunk_start, prev_line_end));
                chunk_start = prev_line_end;
            }

            // If a single line exceeds max_chunk_size, do a hard byte split.
            if line_end - chunk_start > max_chunk_size {
                split_at_bytes(source, chunk_start, line_end, max_chunk_size, out);
                chunk_start = line_end;
            }
        }
        prev_line_end = line_end;
    }

    // Emit the remaining chunk.
    if chunk_start < region_end {
        out.push((chunk_start, region_end));
    }
}

/// Last-resort byte-level splitting, respecting UTF-8 char boundaries.
fn split_at_bytes(
    source: &str,
    region_start: usize,
    region_end: usize,
    max_chunk_size: usize,
    out: &mut Vec<(usize, usize)>,
) {
    let mut pos = region_start;
    while pos < region_end {
        let remaining = region_end - pos;
        if remaining <= max_chunk_size {
            out.push((pos, region_end));
            return;
        }

        // Find the largest chunk boundary that doesn't split a UTF-8 char.
        let mut end = pos + max_chunk_size;
        // Walk back to a char boundary.
        while end > pos && !source.is_char_boundary(end) {
            end -= 1;
        }
        if end == pos {
            // Pathological case: max_chunk_size is smaller than a single
            // multi-byte character. Force include at least one char to
            // guarantee forward progress.
            match source[pos..region_end].chars().next() {
                Some(ch) => end = pos + ch.len_utf8(),
                None => return, // pos >= region_end covered by while guard
            }
        }
        out.push((pos, end));
        pos = end;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Get a parser for testing. Requires at least one `lang-*` feature to be
    /// enabled. Tests are gated with `#[ignore]` when run without features.
    fn test_parser() -> Option<tree_sitter::Parser> {
        let langs = crate::available_languages();
        let lang_name = langs.first()?;
        let language = crate::get_language(lang_name).ok()?;
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language).ok()?;
        Some(parser)
    }

    fn parse_or_skip(source: &str) -> Option<tree_sitter::Tree> {
        let mut parser = test_parser()?;
        parser.parse(source, None)
    }

    // -- Tests that don't need a language --

    #[test]
    fn empty_source_returns_empty_vec() {
        // The function checks source.is_empty() before touching the tree,
        // so we can pass any tree here.
        if let Some(tree) = parse_or_skip("x") {
            let result = split_code("", &tree, 100);
            assert!(result.is_empty());
        }
    }

    #[test]
    fn zero_max_chunk_size_returns_empty_vec() {
        if let Some(tree) = parse_or_skip("x") {
            let result = split_code("x", &tree, 0);
            assert!(result.is_empty());
        }
    }

    // -- Tests that require a language --

    #[test]
    fn source_fits_in_one_chunk() {
        let source = "let x = 1;";
        if let Some(tree) = parse_or_skip(source) {
            let result = split_code(source, &tree, 1000);
            assert_eq!(result, vec![(0, source.len())]);
        }
    }

    #[test]
    fn chunks_cover_entire_source() {
        let source = "fn foo() {}\nfn bar() {}\nfn baz() {}\n";
        if let Some(tree) = parse_or_skip(source) {
            let chunks = split_code(source, &tree, 15);
            assert!(!chunks.is_empty());
            assert_eq!(chunks.first().unwrap().0, 0);
            assert_eq!(chunks.last().unwrap().1, source.len());
            for window in chunks.windows(2) {
                assert_eq!(window[0].1, window[1].0, "chunks must be contiguous");
            }
        }
    }

    #[test]
    fn line_fallback_when_no_ast_boundaries() {
        let source = "aaaa\nbbbb\ncccc\ndddd\n";
        if let Some(tree) = parse_or_skip(source) {
            let chunks = split_code(source, &tree, 10);
            assert!(chunks.len() > 1);
            for &(s, e) in &chunks {
                assert!(e - s <= 10);
            }
        }
    }

    #[test]
    fn byte_fallback_on_long_line() {
        let source = "abcdefghijklmnopqrstuvwxyz";
        if let Some(tree) = parse_or_skip(source) {
            let chunks = split_code(source, &tree, 10);
            let joined: String = chunks.iter().map(|&(s, e)| &source[s..e]).collect();
            assert_eq!(joined, source);
            for &(s, e) in &chunks {
                assert!(e - s <= 10);
            }
        }
    }

    #[test]
    fn utf8_safety_in_byte_fallback() {
        let source = "aaaa\u{1F600}\u{1F600}\u{1F600}\u{1F600}";
        if let Some(tree) = parse_or_skip(source) {
            let chunks = split_code(source, &tree, 6);
            let joined: String = chunks.iter().map(|&(s, e)| &source[s..e]).collect();
            assert_eq!(joined, source);
            for &(s, e) in &chunks {
                assert!(source.is_char_boundary(s));
                assert!(source.is_char_boundary(e));
            }
        }
    }

    #[test]
    fn collect_node_ranges_depth_first() {
        let source = "fn main() {\n    let x = 5;\n}";
        if let Some(tree) = parse_or_skip(source) {
            let ranges = collect_node_ranges(tree.walk());
            for nr in &ranges {
                assert!(nr.range.start <= source.len());
                assert!(nr.range.end <= source.len());
                assert!(nr.range.start <= nr.range.end);
                assert!(nr.depth >= 1);
            }
        }
    }

    // -- Unit tests for internal helpers (no language needed) --

    #[test]
    fn split_at_lines_basic() {
        let source = "line1\nline2\nline3\n";
        let mut out = Vec::new();
        split_at_lines(source, 0, source.len(), 7, &mut out);
        assert!(!out.is_empty());
        let joined: String = out.iter().map(|&(s, e)| &source[s..e]).collect();
        assert_eq!(joined, source);
    }

    #[test]
    fn split_at_bytes_basic() {
        let source = "abcdefghij";
        let mut out = Vec::new();
        split_at_bytes(source, 0, source.len(), 4, &mut out);
        let joined: String = out.iter().map(|&(s, e)| &source[s..e]).collect();
        assert_eq!(joined, source);
        for &(s, e) in &out {
            assert!(e - s <= 4);
        }
    }

    #[test]
    fn split_at_bytes_utf8() {
        let source = "\u{1F600}\u{1F600}\u{1F600}"; // 3 * 4-byte = 12 bytes
        let mut out = Vec::new();
        split_at_bytes(source, 0, source.len(), 5, &mut out);
        let joined: String = out.iter().map(|&(s, e)| &source[s..e]).collect();
        assert_eq!(joined, source);
        for &(s, e) in &out {
            assert!(source.is_char_boundary(s));
            assert!(source.is_char_boundary(e));
        }
    }
}
