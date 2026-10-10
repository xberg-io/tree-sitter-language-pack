//! Scaling and cross-build equivalence tests through the public `process()` API.
//!
//! - `process_scales_linearly_with_every_extractor_and_chunking_enabled` (ignored; run in release) is the regression
//!   guard: each generated shape at 1x and 8x must grow far closer to 8x than to the 64x a
//!   quadratic pass would. Ratios carry generous slack because CI machines are noisy.
//! - `scaling_table` (ignored; `SPLIT_SHAPE=<name>` limits it to one shape) prints the 1x/2x/4x/8x timings for every shape and chunk size.
//! - `dump_output_hashes` (ignored) writes a digest of the full `process()` output for the
//!   corpus and the generators to `$SPLIT_HASH_OUT`, so two builds can be diffed.
//!
//! Grammars that are not linked are reported as `SKIPPED` and the shape is skipped.

#![allow(clippy::print_stdout, clippy::print_stderr, clippy::unwrap_used, clippy::expect_used)]

use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

use tree_sitter_language_pack::{ProcessConfig, process};

#[path = "support/shapes.rs"]
mod shapes;

#[path = "support/corpus.rs"]
mod corpus;

const BASE_BYTES: usize = 64 * 1024;
const CHUNK_SIZES: &[usize] = &[20, 100, 1000, 8000];

fn config(lang: &str, chunk: usize) -> ProcessConfig {
    ProcessConfig::new(lang).all().with_chunking(chunk)
}

fn available(lang: &str) -> bool {
    if tree_sitter_language_pack::get_language(lang).is_ok() {
        return true;
    }
    eprintln!("SKIPPED grammar '{lang}' is not available");
    false
}

fn best_of(reps: usize, source: &str, cfg: &ProcessConfig) -> Duration {
    (0..reps)
        .map(|_| {
            let start = Instant::now();
            let result = process(source, cfg).unwrap();
            let elapsed = start.elapsed();
            std::hint::black_box(&result);
            elapsed
        })
        .min()
        .unwrap()
}

#[test]
#[ignore = "timing-sensitive; run in release with --ignored (CI does)"]
fn process_scales_linearly_with_every_extractor_and_chunking_enabled() {
    let mut worst: Vec<(f64, String)> = Vec::new();
    for shape in shapes::SHAPES {
        if !available(shape.lang) {
            continue;
        }
        let small = shapes::generate(shape, BASE_BYTES);
        let large = shapes::generate(shape, BASE_BYTES * 8);
        for &chunk in &[20usize, 1000] {
            let cfg = config(shape.lang, chunk);
            let t1 = best_of(3, &small, &cfg).as_secs_f64();
            let t8 = best_of(2, &large, &cfg).as_secs_f64();
            let ratio = t8 / t1.max(0.002);
            worst.push((
                ratio,
                format!("{} chunk={chunk} ({:.1}ms -> {:.1}ms)", shape.name, t1 * 1e3, t8 * 1e3),
            ));
            // Linear is ~8x and quadratic ~64x. The input sizes of shapes whose size is
            // depth rather than breadth (nested_*) grow by a different factor, so they get
            // a flat budget instead of a ratio.
            if shape.name.starts_with("nested_") {
                assert!(t8 < 2.0, "{} chunk={chunk}: {t8:.2}s", shape.name);
            } else {
                assert!(
                    t8 < t1.max(0.002) * 8.0 * 2.5 + 0.03,
                    "{} chunk={chunk}: 8x input took {ratio:.1}x longer ({t1:.4}s -> {t8:.4}s)",
                    shape.name
                );
            }
        }
    }
    worst.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (ratio, label) in worst.iter().take(5) {
        eprintln!("8x ratio {ratio:5.1}  {label}");
    }
}

#[test]
#[ignore = "prints a timing table; run with --ignored --nocapture"]
fn scaling_table() {
    println!(
        "{:<28}{:>7}{:>10}{:>10}{:>10}{:>10}{:>8}",
        "shape", "chunk", "1x ms", "2x ms", "4x ms", "8x ms", "8x/1x"
    );
    let only = std::env::var("SPLIT_SHAPE").ok();
    for shape in shapes::SHAPES {
        if !available(shape.lang) || only.as_deref().is_some_and(|name| name != shape.name) {
            continue;
        }
        let inputs: Vec<String> = [1, 2, 4, 8]
            .iter()
            .map(|m| shapes::generate(shape, BASE_BYTES * m))
            .collect();
        for &chunk in CHUNK_SIZES {
            let cfg = config(shape.lang, chunk);
            let t: Vec<f64> = inputs.iter().map(|s| best_of(2, s, &cfg).as_secs_f64() * 1e3).collect();
            println!(
                "{:<28}{:>7}{:>10.1}{:>10.1}{:>10.1}{:>10.1}{:>8.1}",
                shape.name,
                chunk,
                t[0],
                t[1],
                t[2],
                t[3],
                t[3] / t[0].max(0.01)
            );
        }
    }
}

#[test]
fn chunks_from_process_are_contiguous_and_consistent_with_the_source() {
    for shape in shapes::SHAPES {
        if !available(shape.lang) {
            continue;
        }
        let source = shapes::generate(shape, 24 * 1024);
        for &chunk in CHUNK_SIZES {
            let result = process(&source, &config(shape.lang, chunk)).unwrap();
            let chunks = &result.chunks;
            assert_eq!(chunks.first().map(|c| c.start_byte), Some(0), "{}", shape.name);
            assert_eq!(chunks.last().map(|c| c.end_byte), Some(source.len()), "{}", shape.name);
            for (i, c) in chunks.iter().enumerate() {
                assert_eq!(c.content, source[c.start_byte..c.end_byte]);
                assert_eq!(c.metadata.chunk_index, i);
                assert_eq!(c.metadata.total_chunks, chunks.len());
                assert_eq!(
                    c.start_line,
                    source[..c.start_byte].matches('\n').count(),
                    "{} start_line",
                    shape.name
                );
                assert!(
                    c.end_byte - c.start_byte <= chunk,
                    "{} chunk {i} over {chunk}",
                    shape.name
                );
            }
            for pair in chunks.windows(2) {
                assert_eq!(pair[0].end_byte, pair[1].start_byte, "{}", shape.name);
            }
        }
    }
}

struct Fnv(u64);

impl Hasher for Fnv {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0 ^ u64::from(b)).wrapping_mul(0x100_0000_01b3);
        }
    }
}

fn digest(text: &str) -> u64 {
    let mut h = Fnv(0xcbf2_9ce4_8422_2325);
    text.hash(&mut h);
    h.finish()
}

#[test]
#[ignore = "writes digests to $SPLIT_HASH_OUT for before/after comparison across builds"]
fn dump_output_hashes() {
    let Ok(path) = std::env::var("SPLIT_HASH_OUT") else {
        return;
    };
    let mut lines = Vec::new();
    let mut add = |label: String, lang: &str, source: &str| {
        if !available(lang) {
            return;
        }
        for &chunk in &[7usize, 20, 100, 1000, 8000] {
            if let Ok(result) = process(source, &config(lang, chunk)) {
                lines.push(format!(
                    "{label} chunk={chunk} {:016x}",
                    digest(&format!("{:?}", result.chunks))
                ));
            }
        }
    };
    for file in corpus::corpus() {
        let label = file.path.file_name().unwrap().to_string_lossy().into_owned();
        add(format!("corpus/{label}"), file.lang, &file.source);
    }
    for shape in shapes::SHAPES {
        add(
            format!("gen/{}", shape.name),
            shape.lang,
            &shapes::generate(shape, 96 * 1024),
        );
    }
    lines.sort();
    std::fs::write(path, lines.join("\n") + "\n").unwrap();
}
