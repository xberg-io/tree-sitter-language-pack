// ~keep Vendored and adapted from text-splitter by Ben Brandt (MIT License).
// ~keep https://github.com/benbrandt/text-splitter
// ~keep Simplified for tree-sitter 0.26 to return byte ranges without text-splitter chunk sizing.

use std::ops::Range;

use memchr::memchr;

use crate::intel::walk::{Descend, MAX_TREE_DEPTH, walk_bounded};

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
    // ~keep `max_chunk_size == 0` is unreachable through `process()`: `ProcessConfig::validate`
    // ~keep rejects it with `InvalidRange` rather than letting it silently discard the source.
    // ~keep The guard stays as a defence for direct in-crate callers.
    if source.is_empty() || max_chunk_size == 0 {
        return Vec::new();
    }

    if source.len() <= max_chunk_size {
        return vec![(0, source.len())];
    }

    let root = tree.root_node();
    let (node_ranges, truncated) = collect_node_ranges(&root);
    if truncated > 0 {
        tracing::warn!(
            target: "ts_pack::intel",
            operation = "text_splitter::split_code",
            max_depth = MAX_TREE_DEPTH,
            skipped_nodes = truncated,
            "AST deeper than the traversal depth limit; splits below the limit fall back to line boundaries"
        );
    }

    let max_depth = node_ranges.iter().map(|nr| nr.depth).max().unwrap_or(0);

    let mut split_points_by_depth: Vec<Vec<usize>> = vec![Vec::new(); max_depth + 1];
    for nr in &node_ranges {
        split_points_by_depth[nr.depth].push(nr.range.start);
    }
    for points in &mut split_points_by_depth {
        points.push(source.len());
        points.sort_unstable();
        points.dedup();
    }

    let splitter = Splitter {
        source,
        max_chunk_size,
        split_points_by_depth: &split_points_by_depth,
    };
    let mut chunks: Vec<(usize, usize)> = Vec::new();
    splitter.split_recursive(0, source.len(), 0, &mut chunks);

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
///
/// Returns the ranges plus the number of nodes dropped for sitting deeper than
/// [`MAX_TREE_DEPTH`]. That cap is what bounds `split_recursive` below: it
/// recurses once per distinct depth level, so an unbounded AST depth is an
/// unbounded native stack, and a Rust stack overflow aborts the process in a way
/// `catch_unwind` at the FFI boundary cannot contain. ~keep
fn collect_node_ranges(root: &tree_sitter::Node<'_>) -> (Vec<NodeRange>, usize) {
    let mut ranges = Vec::new();
    let truncated = walk_bounded(root, |node, depth| {
        // ~keep Depth 0 is the root; split boundaries only ever come from its descendants.
        if depth > 0 {
            ranges.push(NodeRange {
                depth,
                range: node.byte_range(),
            });
        }
        Descend::Children
    });
    (ranges, truncated)
}

/// Shared read-only state for the recursive AST-guided split.
struct Splitter<'a> {
    source: &'a str,
    max_chunk_size: usize,
    split_points_by_depth: &'a [Vec<usize>],
}

impl Splitter<'_> {
    /// Recursively split the region `[region_start, region_end)` of `source` into
    /// chunks of at most `max_chunk_size` bytes, preferring boundaries at
    /// `current_depth` first, then falling back to deeper depths, then line
    /// boundaries, then raw byte boundaries.
    fn split_recursive(
        &self,
        region_start: usize,
        region_end: usize,
        current_depth: usize,
        out: &mut Vec<(usize, usize)>,
    ) {
        let region_size = region_end - region_start;

        if region_size <= self.max_chunk_size {
            if region_size > 0 {
                out.push((region_start, region_end));
            }
            return;
        }

        if current_depth < self.split_points_by_depth.len() {
            let points = &self.split_points_by_depth[current_depth];

            let relevant: Vec<usize> = points
                .iter()
                .copied()
                .filter(|&p| p > region_start && p < region_end)
                .collect();

            if !relevant.is_empty() {
                let mut boundaries = Vec::with_capacity(relevant.len() + 2);
                boundaries.push(region_start);
                boundaries.extend_from_slice(&relevant);
                boundaries.push(region_end);

                self.merge_boundaries(&boundaries, current_depth, out);
                return;
            }

            if current_depth + 1 < self.split_points_by_depth.len() {
                self.split_recursive(region_start, region_end, current_depth + 1, out);
                return;
            }
        }

        split_at_lines(self.source, region_start, region_end, self.max_chunk_size, out);
    }

    /// Greedily merge adjacent `boundaries` into chunks of at most `max_chunk_size`
    /// bytes, recursing one depth level deeper for any span that still does not fit.
    fn merge_boundaries(&self, boundaries: &[usize], current_depth: usize, out: &mut Vec<(usize, usize)>) {
        let mut cursor = 0;
        while cursor < boundaries.len() - 1 {
            let chunk_start = boundaries[cursor];
            let mut best_end_idx = cursor + 1;
            for (j, &boundary) in boundaries.iter().enumerate().skip(cursor + 1) {
                if boundary - chunk_start <= self.max_chunk_size {
                    best_end_idx = j;
                } else {
                    break;
                }
            }

            let chunk_end = boundaries[best_end_idx];
            if chunk_end - chunk_start <= self.max_chunk_size {
                if chunk_end > chunk_start {
                    out.push((chunk_start, chunk_end));
                }
            } else {
                self.split_recursive(chunk_start, chunk_end, current_depth + 1, out);
            }
            cursor = best_end_idx;
        }
    }
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

    let mut line_ends: Vec<usize> = Vec::new();
    let region_bytes = region.as_bytes();
    let mut search_start = 0;
    while let Some(rel_pos) = memchr(b'\n', &region_bytes[search_start..]) {
        let abs_pos = region_start + search_start + rel_pos + 1;
        line_ends.push(abs_pos);
        search_start += rel_pos + 1;
    }
    if line_ends.last().copied() != Some(region_end) {
        line_ends.push(region_end);
    }

    let mut chunk_start = region_start;
    let mut prev_line_end = region_start;

    for &line_end in &line_ends {
        let candidate_size = line_end - chunk_start;
        if candidate_size > max_chunk_size {
            if prev_line_end > chunk_start {
                out.push((chunk_start, prev_line_end));
                chunk_start = prev_line_end;
            }

            if line_end - chunk_start > max_chunk_size {
                split_at_bytes(source, chunk_start, line_end, max_chunk_size, out);
                chunk_start = line_end;
            }
        }
        prev_line_end = line_end;
    }

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

        let mut end = pos + max_chunk_size;
        while end > pos && !source.is_char_boundary(end) {
            end -= 1;
        }
        if end == pos {
            match source[pos..region_end].chars().next() {
                Some(ch) => end = pos + ch.len_utf8(),
                None => return,
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

    /// Parse with a grammar that actually nests bracket expressions.
    ///
    /// `test_parser` takes whichever language sorts first, and most grammars
    /// flatten a bare `[[[...]]]` run into a shallow ERROR node — so a depth
    /// test built on it silently measures nothing. ~keep
    fn parse_deep_or_skip(source: &str) -> Option<tree_sitter::Tree> {
        let language = ["json", "javascript", "python"]
            .iter()
            .find_map(|name| crate::get_language(name).ok())?;
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language).ok()?;
        parser.parse(source, None)
    }

    #[test]
    fn empty_source_returns_empty_vec() {
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
            let (ranges, truncated) = collect_node_ranges(&tree.root_node());
            assert_eq!(truncated, 0, "a shallow tree must not truncate");
            for nr in &ranges {
                assert!(nr.range.start <= source.len());
                assert!(nr.range.end <= source.len());
                assert!(nr.range.start <= nr.range.end);
                assert!(nr.depth >= 1);
            }
        }
    }

    #[test]
    fn should_bound_collected_depth_so_splitting_deep_input_cannot_overflow_the_stack() {
        // ~keep Brackets must balance: an unclosed run parses to one flat ERROR node, not a deep tree.
        let nesting = MAX_TREE_DEPTH + 200;
        let source = format!("{}1{}", "[".repeat(nesting), "]".repeat(nesting));
        let Some(tree) = parse_deep_or_skip(&source) else {
            return;
        };

        let (ranges, truncated) = collect_node_ranges(&tree.root_node());
        assert!(truncated > 0, "input deeper than the limit must report skipped nodes");
        assert_eq!(
            ranges.iter().map(|nr| nr.depth).max(),
            Some(MAX_TREE_DEPTH),
            "collected depth must stop exactly at the limit"
        );

        // ~keep `split_recursive` recurses once per depth level; before the cap this aborted.
        let chunks = split_code(&source, &tree, 64);
        let joined: String = chunks.iter().map(|&(s, e)| &source[s..e]).collect();
        assert_eq!(joined, source, "chunks must still cover the entire source");
    }

    #[test]
    fn should_merge_adjacent_statements_up_to_exactly_max_chunk_size() {
        let source = "a = 1\nb = 2\nc = 3\n";
        let Some(language) = crate::get_language("python").ok() else {
            return;
        };
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language).unwrap();
        let tree = parser.parse(source, None).unwrap();

        assert_eq!(split_code(source, &tree, 12), vec![(0, 12), (12, 18)]);
        assert_eq!(split_code(source, &tree, 11), vec![(0, 6), (6, 12), (12, 18)]);
    }

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
        let source = "\u{1F600}\u{1F600}\u{1F600}";
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
