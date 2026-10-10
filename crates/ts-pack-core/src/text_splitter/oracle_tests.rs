//! Equivalence, correctness and optimality tests for [`split_code`] over the on-disk corpus
//! and the seeded generators in `tests/support/shapes.rs`.
//!
//! `oracle_split` is a verbatim copy of the algorithm before the per-region lookup was
//! indexed (every split point of a level filtered for every region). The optimised
//! splitter must produce byte-identical ranges. Grammars that are not linked are reported
//! on stderr as `SKIPPED` and the case is skipped.

#![allow(clippy::print_stderr, clippy::unwrap_used, clippy::expect_used)]

use std::collections::BTreeSet;

use super::*;
use crate::intel::walk::MAX_TREE_DEPTH;

#[path = "../../tests/support/shapes.rs"]
mod shapes;

#[path = "../../tests/support/corpus.rs"]
mod corpus;

use shapes::{Rng, SHAPES};

fn shape_named(name: &str) -> &'static shapes::Shape {
    SHAPES.iter().find(|s| s.name == name).expect("known shape")
}

const MAX_SIZES: &[usize] = &[1, 4, 7, 20, 64, 100, 333, 1000, 4096];

fn parse(lang: &str, src: &str) -> Option<tree_sitter::Tree> {
    let language = match crate::get_language(lang) {
        Ok(language) => language,
        Err(error) => {
            eprintln!("SKIPPED grammar '{lang}' is not available: {error}");
            return None;
        }
    };
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language).ok()?;
    parser.parse(src, None)
}

fn oracle_split(source: &str, tree: &tree_sitter::Tree, max_chunk_size: usize) -> Vec<(usize, usize)> {
    if source.is_empty() || max_chunk_size == 0 {
        return Vec::new();
    }
    if source.len() <= max_chunk_size {
        return vec![(0, source.len())];
    }
    let (node_ranges, _) = collect_node_ranges(&tree.root_node());
    let max_depth = node_ranges.iter().map(|nr| nr.depth).max().unwrap_or(0);
    let mut by_depth: Vec<Vec<usize>> = vec![Vec::new(); max_depth + 1];
    for nr in &node_ranges {
        by_depth[nr.depth].push(nr.range.start);
    }
    for points in &mut by_depth {
        points.push(source.len());
        points.sort_unstable();
        points.dedup();
    }
    let mut out = Vec::new();
    oracle_recursive(source, &by_depth, max_chunk_size, 0, source.len(), 0, &mut out);
    out
}

fn oracle_recursive(
    source: &str,
    by_depth: &[Vec<usize>],
    max: usize,
    region_start: usize,
    region_end: usize,
    depth: usize,
    out: &mut Vec<(usize, usize)>,
) {
    let region_size = region_end - region_start;
    if region_size <= max {
        if region_size > 0 {
            out.push((region_start, region_end));
        }
        return;
    }
    if depth < by_depth.len() {
        let relevant: Vec<usize> = by_depth[depth]
            .iter()
            .copied()
            .filter(|&p| p > region_start && p < region_end)
            .collect();
        if !relevant.is_empty() {
            let mut boundaries = Vec::with_capacity(relevant.len() + 2);
            boundaries.push(region_start);
            boundaries.extend_from_slice(&relevant);
            boundaries.push(region_end);
            let mut cursor = 0;
            while cursor < boundaries.len() - 1 {
                let chunk_start = boundaries[cursor];
                let mut best = cursor + 1;
                for (j, &b) in boundaries.iter().enumerate().skip(cursor + 1) {
                    if b - chunk_start <= max {
                        best = j;
                    } else {
                        break;
                    }
                }
                let chunk_end = boundaries[best];
                if chunk_end - chunk_start <= max {
                    if chunk_end > chunk_start {
                        out.push((chunk_start, chunk_end));
                    }
                } else {
                    oracle_recursive(source, by_depth, max, chunk_start, chunk_end, depth + 1, out);
                }
                cursor = best;
            }
            return;
        }
        if depth + 1 < by_depth.len() {
            oracle_recursive(source, by_depth, max, region_start, region_end, depth + 1, out);
            return;
        }
    }
    split_at_lines(source, region_start, region_end, max, out);
}

/// Every named-or-anonymous non-root node as `(start, end, depth)`, plus whether the walk
/// hit the depth cap (in which case boundary-availability assertions do not apply).
fn nodes(tree: &tree_sitter::Tree) -> (Vec<(usize, usize, usize)>, bool) {
    let mut out = Vec::new();
    let mut capped = false;
    let mut cursor = tree.walk();
    let mut depth = 0usize;
    loop {
        let node = cursor.node();
        if depth > 0 {
            out.push((node.start_byte(), node.end_byte(), depth));
        }
        if depth >= MAX_TREE_DEPTH {
            capped = true;
        } else if cursor.goto_first_child() {
            depth += 1;
            continue;
        }
        loop {
            if depth == 0 {
                return (out, capped);
            }
            if cursor.goto_next_sibling() {
                break;
            }
            cursor.goto_parent();
            depth -= 1;
        }
    }
}

/// Inputs the whole suite runs over: real corpus files plus generated shapes.
fn inputs(generated_bytes: usize) -> Vec<(String, &'static str, String)> {
    let mut all: Vec<(String, &'static str, String)> = corpus::corpus()
        .into_iter()
        .map(|f| (f.path.display().to_string(), f.lang, f.source))
        .collect();
    for shape in SHAPES {
        all.push((
            shape.name.to_string(),
            shape.lang,
            shapes::generate(shape, generated_bytes),
        ));
    }
    all
}

#[test]
fn optimized_split_is_byte_identical_to_the_original_on_corpus_and_generated_inputs() {
    let mut compared = 0usize;
    for (name, lang, source) in inputs(20 * 1024) {
        let Some(tree) = parse(lang, &source) else { continue };
        for &max in MAX_SIZES {
            assert_eq!(
                split_regions(&source, &tree, max),
                oracle_split(&source, &tree, max),
                "{name} ({lang}) diverged from the original at max={max}"
            );
            compared += 1;
        }
    }
    eprintln!("oracle equivalence: {compared} (input, max) pairs identical");
}

#[test]
fn optimized_split_matches_the_original_on_seeded_random_inputs() {
    let mut rng = Rng::new(0x5eed);
    for round in 0..120 {
        let shape = &SHAPES[rng.below(SHAPES.len())];
        let target = rng.range(200, 20_000);
        let source = (shape.generate)(&mut Rng::new(rng.next_u64()), target);
        let Some(tree) = parse(shape.lang, &source) else {
            continue;
        };
        let max = rng.range(1, 3000);
        assert_eq!(
            split_regions(&source, &tree, max),
            oracle_split(&source, &tree, max),
            "round {round}: {} target={target} max={max}",
            shape.name
        );
    }
}

fn check_structure(name: &str, source: &str, chunks: &[(usize, usize)], max: usize) {
    assert!(!chunks.is_empty(), "{name}: no chunks for non-empty source");
    assert_eq!(chunks[0].0, 0, "{name}: must start at 0");
    assert_eq!(
        chunks[chunks.len() - 1].1,
        source.len(),
        "{name}: must cover the source"
    );
    for &(s, e) in chunks {
        assert!(s < e, "{name}: empty chunk ({s},{e})");
        assert!(
            source.is_char_boundary(s) && source.is_char_boundary(e),
            "{name}: split inside a char"
        );
        if max >= 4 {
            assert!(e - s <= max, "{name}: chunk {s}..{e} exceeds max={max}");
        } else {
            assert!(
                e - s <= max || source[s..e].chars().count() == 1,
                "{name}: multi-char chunk over max={max}"
            );
        }
    }
    for pair in chunks.windows(2) {
        assert_eq!(pair[0].1, pair[1].0, "{name}: chunks must be contiguous");
    }
}

#[test]
fn chunks_are_contiguous_cover_the_source_and_respect_size_and_utf8() {
    for (name, lang, source) in inputs(20 * 1024) {
        let Some(tree) = parse(lang, &source) else { continue };
        for &max in MAX_SIZES {
            if max >= source.len() {
                continue;
            }
            check_structure(&name, &source, &split_code(&source, &tree, max), max);
        }
    }
}

fn line_of(newlines: &[usize], byte: usize) -> usize {
    newlines.partition_point(|&p| p < byte)
}

#[test]
fn chunk_line_numbers_agree_with_byte_offsets() {
    use crate::intel::chunking::chunk_source;
    for (name, lang, source) in inputs(12 * 1024) {
        let Ok(language) = crate::get_language(lang) else {
            continue;
        };
        let Some(tree) = parse(lang, &source) else { continue };
        let newlines: Vec<usize> = source.match_indices('\n').map(|(i, _)| i).collect();
        for &max in &[20usize, 333, 4096] {
            for chunk in chunk_source(&source, lang, max, &language, &tree) {
                assert_eq!(chunk.content, source[chunk.start_byte..chunk.end_byte], "{name}");
                assert_eq!(
                    chunk.start_line,
                    line_of(&newlines, chunk.start_byte),
                    "{name} start_line"
                );
                let last = chunk.end_byte.saturating_sub(1).max(chunk.start_byte);
                assert_eq!(chunk.end_line, line_of(&newlines, last), "{name} end_line");
            }
        }
    }
}

/// Greedy maximality at the merge level: splitting a flat run of equal-width items, no two
/// adjacent chunks fit within the limit together, and the chunk count is the ideal
/// `ceil(items / items_per_chunk)`.
#[test]
fn uniform_siblings_are_packed_maximally_with_the_ideal_chunk_count() {
    let item = |i: usize| format!("fn f{i:07}(a: i32) -> i32 {{ g(a) }}\n");
    let unit = item(0).len();
    let items = 600;
    let source: String = (0..items).map(item).collect();
    let Some(tree) = parse("rust", &source) else { return };
    for per_chunk in [1usize, 2, 3, 7, 40, 599] {
        for slack in [0, unit - 1] {
            let max = per_chunk * unit + slack;
            let chunks = split_code(&source, &tree, max);
            assert_eq!(
                chunks.len(),
                items.div_ceil(per_chunk),
                "per_chunk={per_chunk} slack={slack}"
            );
            for (idx, &(s, e)) in chunks.iter().enumerate() {
                if idx + 1 < chunks.len() {
                    assert_eq!(e - s, per_chunk * unit, "only the last chunk may be short");
                }
                assert_eq!(s % unit, 0, "boundary must be an item start");
            }
            let mergeable = chunks.windows(2).filter(|p| p[1].1 - p[0].0 <= max).count();
            assert_eq!(mergeable, 0, "adjacent chunks that could have been merged");
        }
    }
}

/// Boundary quality: boundaries only fall mid-token when the token is longer than the limit,
/// and every such byte-level chunk is filled to the limit (no tiny pieces).
#[test]
fn boundaries_are_syntax_boundaries_unless_forced_to_split_a_long_line() {
    let mut mid_token = 0usize;
    for (name, lang, source) in inputs(16 * 1024) {
        let Some(tree) = parse(lang, &source) else { continue };
        let (all_nodes, capped) = nodes(&tree);
        if capped {
            continue;
        }
        let starts: BTreeSet<usize> = all_nodes.iter().map(|n| n.0).collect();
        let ends: BTreeSet<usize> = all_nodes.iter().map(|n| n.1).collect();
        for &max in &[7usize, 20, 100, 1000] {
            let chunks = split_code(&source, &tree, max);
            for &(s, e) in &chunks {
                if e == source.len() {
                    continue;
                }
                let on_node = starts.contains(&e) || ends.contains(&e);
                let on_line = source.as_bytes()[e - 1] == b'\n';
                if on_node || on_line {
                    continue;
                }
                mid_token += 1;
                assert!(
                    e - s + 3 >= max,
                    "{name} max={max}: undersized byte-level chunk ({s},{e})"
                );
            }
        }
    }
    eprintln!("mid-token boundaries observed (all forced): {mid_token}");
}

/// Top-level items are preferred over nested statements: when every module fits the limit,
/// every internal boundary is the start of a top-level item.
#[test]
fn shallowest_boundaries_are_chosen_when_the_top_level_items_fit() {
    let shape = shape_named("rust_nested_modules");
    let source = shapes::generate(shape, 64 * 1024);
    let Some(tree) = parse("rust", &source) else { return };
    let (all_nodes, _) = nodes(&tree);
    let depth1: BTreeSet<usize> = all_nodes.iter().filter(|n| n.2 == 1).map(|n| n.0).collect();
    let biggest = all_nodes.iter().filter(|n| n.2 == 1).map(|n| n.1 - n.0).max().unwrap();
    for max in [biggest + 1, biggest * 2, biggest * 5] {
        let before = split_regions(&source, &tree, max);
        assert!(
            before.iter().all(|&(_, e)| e == source.len() || depth1.contains(&e)),
            "max={max}: regional split left the top level"
        );
        // Only the re-cut of a tiny chunk (here the file's short last module) may leave it,
        // and each re-cut removes at least one tiny chunk.
        let tiny_before = before.iter().filter(|&&(s, e)| (e - s) * 4 < max).count();
        let off_level = split_code(&source, &tree, max)
            .iter()
            .filter(|&&(_, e)| e != source.len() && !depth1.contains(&e))
            .count();
        assert!(
            off_level <= tiny_before,
            "max={max}: {off_level} off-level cuts, {tiny_before} tiny chunks"
        );
    }
}
/// Chunk counts never exceed the original's, and never undercut the information-theoretic
/// floor `ceil(len / max)`.
#[test]
fn chunk_count_is_never_worse_than_the_original_and_respects_the_floor() {
    for (name, lang, source) in inputs(16 * 1024) {
        let Some(tree) = parse(lang, &source) else { continue };
        for &max in MAX_SIZES {
            if max >= source.len() {
                continue;
            }
            let got = split_code(&source, &tree, max);
            let want = oracle_split(&source, &tree, max);
            assert!(
                got.len() <= want.len(),
                "{name} max={max}: {} > {}",
                got.len(),
                want.len()
            );
            if max >= 4 {
                assert!(
                    got.len() >= source.len().div_ceil(max),
                    "{name} max={max}: below the floor"
                );
            }
        }
    }
}

/// Orphans (tiny non-final chunks) never exceed what the original produced.
#[test]
fn orphan_chunks_are_never_more_common_than_in_the_original() {
    let tiny = |chunks: &[(usize, usize)], max: usize| {
        chunks.iter().rev().skip(1).filter(|&&(s, e)| (e - s) * 4 < max).count()
    };
    let mut total = 0usize;
    for (name, lang, source) in inputs(16 * 1024) {
        let Some(tree) = parse(lang, &source) else { continue };
        for &max in &[100usize, 333, 1000, 4096] {
            let got = tiny(&split_code(&source, &tree, max), max);
            assert!(got <= tiny(&oracle_split(&source, &tree, max), max), "{name} max={max}");
            total += got;
        }
    }
    eprintln!("tiny non-final chunks across corpus (same as original): {total}");
}

/// Regression for the region-edge defect: the oversized `fn a` used to leave a 7-byte head
/// (`fn a() `) and `fn b` alone (3 chunks) although a line boundary at byte 24 gives a valid
/// two-chunk split.
#[test]
fn an_oversized_unit_does_not_leave_an_orphan_head_when_the_neighbour_fits() {
    let source = "fn a() {\n    let x = 1;\n    let y = 2;\n}\nfn b() {}\n";
    let Some(tree) = parse("rust", source) else { return };
    let max = 36;
    assert_eq!(split_regions(source, &tree, max), vec![(0, 7), (7, 41), (41, 51)]);
    assert_eq!(split_code(source, &tree, max), vec![(0, 24), (24, 51)]);
}

/// After rebalancing, a short tail is merged into the unit that follows it.
#[test]
fn a_short_tail_of_a_split_unit_is_merged_with_the_next_unit() {
    let source = "fn a() {\n    let x = 1;\n    let y = 2;\n    let z = 3;\n}\nfn b() {}\n";
    let Some(tree) = parse("rust", source) else { return };
    let max = 40;
    let chunks = split_code(source, &tree, max);
    check_structure("tail", source, &chunks, max);
    assert!(chunks.windows(2).all(|p| p[1].1 - p[0].0 > max), "{chunks:?}");
    assert!(chunks.len() <= split_regions(source, &tree, max).len());
}

/// No two adjacent chunks fit together within the limit, and the count is no worse than
/// before rebalancing, over the whole corpus and every generator.
#[test]
fn rebalanced_chunks_leave_no_adjacent_pair_that_could_merge() {
    let (mut fewer, mut total_before, mut total_after) = (0usize, 0usize, 0usize);
    for (name, lang, source) in inputs(16 * 1024) {
        let Some(tree) = parse(lang, &source) else { continue };
        for &max in MAX_SIZES {
            if max >= source.len() {
                continue;
            }
            let got = split_code(&source, &tree, max);
            check_structure(&name, &source, &got, max);
            assert!(
                got.windows(2).all(|p| p[1].1 - p[0].0 > max),
                "{name} max={max}: adjacent chunks could merge"
            );
            let before = split_regions(&source, &tree, max).len();
            assert!(got.len() <= before, "{name} max={max}");
            fewer += before - got.len();
            total_before += before;
            total_after += got.len();
        }
    }
    eprintln!("edge rebalancing: {total_before} -> {total_after} chunks ({fewer} removed)");
}

/// Byte spans of the lines of `source` that start with `prefix`, excluding the newline.
fn line_spans(source: &str, prefix: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut offset = 0;
    for line in source.split_inclusive('\n') {
        let text = line.trim_end_matches(['\n', '\r']);
        if text.trim_start().starts_with(prefix) {
            let lead = text.len() - text.trim_start().len();
            spans.push((offset + lead, offset + text.len()));
        }
        offset += line.len();
    }
    spans
}

fn contained(span: (usize, usize), chunk: (usize, usize)) -> bool {
    span.0 >= chunk.0 && span.1 <= chunk.1
}

/// Metadata lands on the chunk that actually contains the node: a definition is a symbol of
/// exactly the chunk holding all of it, never of a chunk that only overlaps it.
#[test]
fn symbols_are_attributed_only_to_the_chunk_that_contains_the_definition() {
    use crate::intel::chunking::chunk_source;
    let cases: [(&str, &str, &str); 3] = [
        ("rust_many_fns", "rust", "fn f"),
        ("python_many_defs", "python", "def f"),
        ("js_many_fns", "javascript", "function f"),
    ];
    for (shape_name, lang, prefix) in cases {
        let source = shapes::generate(shape_named(shape_name), 24 * 1024);
        let Ok(language) = crate::get_language(lang) else {
            continue;
        };
        let Some(tree) = parse(lang, &source) else { continue };
        let starts: Vec<usize> = line_spans(&source, prefix).iter().map(|d| d.0).collect();
        let defs: Vec<(usize, usize)> = starts
            .iter()
            .enumerate()
            .map(|(i, &s)| {
                let next = starts.get(i + 1).copied().unwrap_or(source.len());
                (s, s + source[s..next].trim_end().len())
            })
            .collect();
        for max in [60usize, 200, 1000] {
            let chunks = chunk_source(&source, lang, max, &language, &tree);
            let mut attributed = 0;
            for chunk in &chunks {
                let range = (chunk.start_byte, chunk.end_byte);
                let expected: Vec<String> = defs
                    .iter()
                    .filter(|&&d| contained(d, range))
                    .map(|&(s, _)| {
                        let rest = &source[s + prefix.len() - 1..];
                        rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect()
                    })
                    .collect();
                attributed += expected.len();
                assert_eq!(
                    chunk.metadata.symbols_defined, expected,
                    "{shape_name} max={max} chunk {range:?}"
                );
            }
            let whole = defs
                .iter()
                .filter(|&&d| chunks.iter().any(|c| contained(d, (c.start_byte, c.end_byte))))
                .count();
            assert_eq!(
                attributed, whole,
                "{shape_name} max={max}: each contained definition exactly once"
            );
        }
    }
}

/// Comments are attributed to the chunk containing them, once.
#[test]
fn comments_are_attributed_only_to_the_chunk_that_contains_them() {
    use crate::intel::chunking::chunk_source;
    let source = shapes::generate(shape_named("python_comments_docstrings"), 24 * 1024);
    let Ok(language) = crate::get_language("python") else {
        return;
    };
    let Some(tree) = parse("python", &source) else { return };
    let comment_spans = line_spans(&source, "#");
    for max in [80usize, 300, 2000] {
        let chunks = chunk_source(&source, "python", max, &language, &tree);
        let mut total = 0;
        for chunk in &chunks {
            let range = (chunk.start_byte, chunk.end_byte);
            let expected = comment_spans.iter().filter(|&&c| contained(c, range)).count();
            assert_eq!(chunk.metadata.comments.len(), expected, "max={max} chunk {range:?}");
            for comment in &chunk.metadata.comments {
                assert!(
                    source[range.0..range.1].contains(comment.text.trim_end()),
                    "comment text not in its chunk"
                );
            }
            total += expected;
        }
        let contained_somewhere = comment_spans
            .iter()
            .filter(|&&c| chunks.iter().any(|ch| contained(c, (ch.start_byte, ch.end_byte))))
            .count();
        assert_eq!(total, contained_somewhere, "max={max}");
    }
}

/// `node_types` lists a kind only when the chunk holds a whole node of it at its top level.
#[test]
fn node_types_name_a_definition_only_for_chunks_that_hold_a_whole_one() {
    use crate::intel::chunking::chunk_source;
    let source = shapes::generate(shape_named("rust_many_fns"), 24 * 1024);
    let Ok(language) = crate::get_language("rust") else {
        return;
    };
    let Some(tree) = parse("rust", &source) else { return };
    let defs = line_spans(&source, "fn f");
    for max in [30usize, 200, 1000] {
        for chunk in chunk_source(&source, "rust", max, &language, &tree) {
            let range = (chunk.start_byte, chunk.end_byte);
            let holds_whole = defs.iter().any(|&d| contained(d, range));
            let lists = chunk.metadata.node_types.iter().any(|t| t == "function_item");
            assert_eq!(
                lists, holds_whole,
                "max={max} chunk {range:?}: {:?}",
                chunk.metadata.node_types
            );
        }
    }
}

/// A comment that straddles a chunk boundary is reported by the chunk holding its start,
/// whole and exactly once, instead of by none.
#[test]
fn a_comment_spanning_a_chunk_boundary_is_attributed_to_the_chunk_containing_its_start() {
    use crate::intel::chunking::chunk_source;
    let source = "fn a() {}\n/* one long block comment that cannot fit in a single small chunk */\nfn b() {}\n";
    let Ok(language) = crate::get_language("rust") else {
        return;
    };
    let Some(tree) = parse("rust", source) else { return };
    let comment_start = source.find("/*").unwrap();
    let comment_end = source.find("*/").unwrap() + 2;
    let chunks = chunk_source(source, "rust", 30, &language, &tree);
    assert!(
        chunks
            .iter()
            .any(|c| c.start_byte < comment_end && c.end_byte > comment_end),
        "premise: the comment must straddle a boundary: {chunks:?}"
    );
    let owners: Vec<usize> = chunks
        .iter()
        .filter(|c| !c.metadata.comments.is_empty())
        .map(|c| c.metadata.chunk_index)
        .collect();
    let owner = chunks
        .iter()
        .find(|c| c.start_byte <= comment_start && comment_start < c.end_byte)
        .unwrap();
    assert_eq!(owners, vec![owner.metadata.chunk_index]);
    assert_eq!(owner.metadata.comments[0].text, &source[comment_start..comment_end]);
}

/// A Python docstring that straddles a chunk boundary is reported by the chunk holding its
/// first byte, whole and exactly once, instead of by none.
#[test]
fn a_docstring_spanning_a_chunk_boundary_is_attributed_to_the_chunk_containing_its_start() {
    use crate::intel::chunking::chunk_source;
    let source =
        "def f():\n    \"\"\"A long docstring that cannot fit in a single small chunk at all.\"\"\"\n    return 1\n";
    let Ok(language) = crate::get_language("python") else {
        return;
    };
    let Some(tree) = parse("python", source) else { return };
    let doc_start = source.find("\"\"\"").unwrap();
    let doc_end = source.rfind("\"\"\"").unwrap() + 3;
    let chunks = chunk_source(source, "python", 30, &language, &tree);
    assert!(
        chunks
            .iter()
            .any(|c| c.start_byte < doc_end && c.end_byte > doc_start && c.end_byte < doc_end),
        "premise: the docstring must straddle a boundary: {chunks:?}"
    );
    let owners: Vec<usize> = chunks
        .iter()
        .filter(|c| !c.metadata.docstrings.is_empty())
        .map(|c| c.metadata.chunk_index)
        .collect();
    let owner = chunks
        .iter()
        .find(|c| c.start_byte <= doc_start && doc_start < c.end_byte)
        .unwrap();
    assert_eq!(owners, vec![owner.metadata.chunk_index]);
    assert_eq!(owner.metadata.docstrings[0].text, &source[doc_start..doc_end]);
}
