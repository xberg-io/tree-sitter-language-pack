use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use tree_sitter_language_pack::{
    ProcessConfig, detect_language_from_content, detect_language_from_extension, detect_language_from_path,
    parse_string, process,
};

// ---------------------------------------------------------------------------
// Fixture data (embedded at compile time)
// ---------------------------------------------------------------------------

const PYTHON_SMALL: &str = include_str!("../../../fixtures/bench/python/small.py");
const PYTHON_MEDIUM: &str = include_str!("../../../fixtures/bench/python/medium.py");
const PYTHON_LARGE: &str = include_str!("../../../fixtures/bench/python/large.py");

const TYPESCRIPT_SMALL: &str = include_str!("../../../fixtures/bench/typescript/small.ts");
const TYPESCRIPT_MEDIUM: &str = include_str!("../../../fixtures/bench/typescript/medium.ts");
const TYPESCRIPT_LARGE: &str = include_str!("../../../fixtures/bench/typescript/large.tsx");

const RUST_SMALL: &str = include_str!("../../../fixtures/bench/rust/small.rs");
const RUST_MEDIUM: &str = include_str!("../../../fixtures/bench/rust/medium.rs");
const RUST_LARGE: &str = include_str!("../../../fixtures/bench/rust/large.rs");

const GO_SMALL: &str = include_str!("../../../fixtures/bench/go/small.go");
const GO_MEDIUM: &str = include_str!("../../../fixtures/bench/go/medium.go");
const GO_LARGE: &str = include_str!("../../../fixtures/bench/go/large.go");

// ---------------------------------------------------------------------------
// 1. parse — parse_string() across 4 languages x 3 sizes
// ---------------------------------------------------------------------------

fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");

    let cases: &[(&str, &str, &str)] = &[
        ("python/small", "python", PYTHON_SMALL),
        ("python/medium", "python", PYTHON_MEDIUM),
        ("python/large", "python", PYTHON_LARGE),
        ("typescript/small", "typescript", TYPESCRIPT_SMALL),
        ("typescript/medium", "typescript", TYPESCRIPT_MEDIUM),
        ("typescript/large", "tsx", TYPESCRIPT_LARGE),
        ("rust/small", "rust", RUST_SMALL),
        ("rust/medium", "rust", RUST_MEDIUM),
        ("rust/large", "rust", RUST_LARGE),
        ("go/small", "go", GO_SMALL),
        ("go/medium", "go", GO_MEDIUM),
        ("go/large", "go", GO_LARGE),
    ];

    for &(id, lang, source) in cases {
        group.bench_function(id, |b| {
            b.iter(|| parse_string(black_box(lang), black_box(source.as_bytes())).unwrap());
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// 2. process — all flags vs minimal, Python medium/large
// ---------------------------------------------------------------------------

fn bench_process(c: &mut Criterion) {
    let mut group = c.benchmark_group("process");

    group.bench_function("python/medium/all", |b| {
        let config = ProcessConfig::new("python").all();
        b.iter(|| process(black_box(PYTHON_MEDIUM), black_box(&config)).unwrap());
    });

    group.bench_function("python/medium/minimal", |b| {
        let config = ProcessConfig::new("python").minimal();
        b.iter(|| process(black_box(PYTHON_MEDIUM), black_box(&config)).unwrap());
    });

    group.bench_function("python/large/all", |b| {
        let config = ProcessConfig::new("python").all();
        b.iter(|| process(black_box(PYTHON_LARGE), black_box(&config)).unwrap());
    });

    group.bench_function("python/large/minimal", |b| {
        let config = ProcessConfig::new("python").minimal();
        b.iter(|| process(black_box(PYTHON_LARGE), black_box(&config)).unwrap());
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// 3. text_splitter — process() with chunking, Python medium
// ---------------------------------------------------------------------------

fn bench_text_splitter(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_splitter");

    group.bench_function("python/medium/chunk_1000", |b| {
        let config = ProcessConfig::new("python").all().with_chunking(1000);
        b.iter(|| process(black_box(PYTHON_MEDIUM), black_box(&config)).unwrap());
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// 4. language_detection — extension, path, content detection
// ---------------------------------------------------------------------------

fn bench_language_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("language_detection");

    group.bench_function("from_extension", |b| {
        b.iter(|| detect_language_from_extension(black_box("py")));
    });

    group.bench_function("from_path", |b| {
        b.iter(|| detect_language_from_path(black_box("src/main.rs")));
    });

    group.bench_function("from_content", |b| {
        b.iter(|| detect_language_from_content(black_box("#!/usr/bin/env python3\n")));
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Criterion harness
// ---------------------------------------------------------------------------

criterion_group!(
    benches,
    bench_parse,
    bench_process,
    bench_text_splitter,
    bench_language_detection,
);
criterion_main!(benches);
