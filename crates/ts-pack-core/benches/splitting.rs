// Same rationale as benchmarks.rs: a bench has no caller to hand a `Result` to. ~keep
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::print_stderr)]

//! Scaling benchmarks for `process()` with every extractor and chunking enabled.
//!
//! Each generated shape (see `tests/support/shapes.rs`) runs at 1x/2x/4x/8x input size and
//! at several chunk sizes, with throughput reported in bytes, so a super-linear pass shows
//! up as falling throughput down the size axis. Smoke run:
//! `cargo bench -p tree-sitter-language-pack --bench splitting -- --test`.

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use tree_sitter_language_pack::{ProcessConfig, get_language, process};

#[path = "../tests/support/shapes.rs"]
mod shapes;

const BASE_BYTES: usize = 64 * 1024;
const CHUNK_SIZES: &[usize] = &[20, 100, 1000];
const SCALES: &[usize] = &[1, 2, 4, 8];

fn bench_shapes(c: &mut Criterion) {
    for shape in shapes::SHAPES {
        if get_language(shape.lang).is_err() {
            eprintln!("SKIPPED grammar '{}' is not available", shape.lang);
            continue;
        }
        let mut group = c.benchmark_group(format!("split/{}", shape.name));
        group.sample_size(10);
        for &chunk in CHUNK_SIZES {
            let cfg = ProcessConfig::new(shape.lang).all().with_chunking(chunk);
            for &scale in SCALES {
                let source = shapes::generate(shape, BASE_BYTES * scale);
                group.throughput(Throughput::Bytes(source.len() as u64));
                group.bench_with_input(BenchmarkId::new(format!("chunk{chunk}"), scale), &source, |b, src| {
                    b.iter(|| black_box(process(black_box(src), &cfg).unwrap()));
                });
            }
        }
        group.finish();
    }
}

criterion_group!(benches, bench_shapes);
criterion_main!(benches);
