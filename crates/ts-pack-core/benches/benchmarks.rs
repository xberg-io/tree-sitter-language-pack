// Benchmark setup that cannot proceed should abort the run outright — a bench has no
// caller to hand a `Result` to, and silently measuring a degraded path is worse than
// failing. The crate-wide deny exists to protect host processes, which a bench is not.
// Criterion bodies are plain fns, not `#[test]`, so clippy.toml's test exemption misses
// them. ~keep
#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Benchmarks for the language pack.
//!
//! Layout, and what each group is for:
//!
//! - `registry` — name lookups. `get_language` is the hot path every caller pays;
//!   `has_language`/`has_parser`/`available_languages`/`language_count` are the
//!   ones that can reach the filesystem.
//! - `query/cached` and `query/compile_cold` — a cache hit, and the compile it
//!   avoids. Cold compiles go through `tree_sitter::Query` directly, because a
//!   second `get_query` for the same key can never be cold again in one process.
//! - `parse` — the *unlocked* path (`Parser::parse_bytes`), one case per fixture.
//! - `process/all` and `process/extractors` — the locked path plus intelligence
//!   extraction, broken down one extractor at a time so a change to a single
//!   pipeline stage is attributable instead of averaged away.
//! - `chunking` — `process` with and without chunking; the splitter is private, so
//!   its cost is only observable as the delta between the two.
//! - `concurrency` — the same work at 1/2/4/8 threads on both paths. Both `process`
//!   and `parse_bytes` parse fully in parallel across threads, except for the small
//!   set of grammars whose external scanner keeps mutable process-global state
//!   (currently just `properties`), whose parses serialize on a per-language lock.
//!   None of `BENCH_LANGUAGES` are in that set, so both curves are expected to scale
//!   together here.

#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::hint::black_box;
use std::sync::Once;
use std::time::{Duration, Instant};

use criterion::{BenchmarkId, Criterion, SamplingMode, Throughput, criterion_group, criterion_main};
use tree_sitter::Query;
use tree_sitter_language_pack::{
    ProcessConfig, QueryKind, available_languages, detect_language_from_path, get_folds_query, get_highlights_query,
    get_indents_query, get_injections_query, get_language, get_locals_query, get_parser, get_query, get_tags_query,
    has_language, has_parser, language_count, prefetch, process,
};

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

// ~keep Gap-filling fixtures: data formats, an indentation-sensitive external scanner,
// ~keep pathological depth, non-ASCII, and input that only parses via error recovery.
const JSON_LARGE: &str = include_str!("../tests/fixtures/bench/json/large.json");
const YAML_MEDIUM: &str = include_str!("../tests/fixtures/bench/yaml/medium.yaml");
const RUBY_MEDIUM: &str = include_str!("../tests/fixtures/bench/ruby/medium.rb");
const PYTHON_DEEP: &str = include_str!("../tests/fixtures/bench/python/deep_nesting.py");
const PYTHON_UNICODE: &str = include_str!("../tests/fixtures/bench/python/unicode.py");
const PYTHON_BROKEN: &str = include_str!("../tests/fixtures/bench/python/broken.py");

/// Every grammar the suite needs, fetched once up front so no benchmark times a download.
const BENCH_LANGUAGES: &[&str] = &[
    "bash",
    "python",
    "typescript",
    "tsx",
    "rust",
    "go",
    "json",
    "yaml",
    "ruby",
    "swift",
];

/// `(id, language, source)` for every parse-shaped benchmark.
const CASES: &[(&str, &str, &str)] = &[
    ("python/small", "python", PYTHON_SMALL),
    ("python/medium", "python", PYTHON_MEDIUM),
    ("python/large", "python", PYTHON_LARGE),
    ("python/deep_nesting", "python", PYTHON_DEEP),
    ("python/unicode", "python", PYTHON_UNICODE),
    ("python/error_recovery", "python", PYTHON_BROKEN),
    ("typescript/small", "typescript", TYPESCRIPT_SMALL),
    ("typescript/medium", "typescript", TYPESCRIPT_MEDIUM),
    ("typescript/large", "tsx", TYPESCRIPT_LARGE),
    ("rust/small", "rust", RUST_SMALL),
    ("rust/medium", "rust", RUST_MEDIUM),
    ("rust/large", "rust", RUST_LARGE),
    ("go/small", "go", GO_SMALL),
    ("go/medium", "go", GO_MEDIUM),
    ("go/large", "go", GO_LARGE),
    ("json/large", "json", JSON_LARGE),
    ("yaml/medium", "yaml", YAML_MEDIUM),
    ("ruby/medium", "ruby", RUBY_MEDIUM),
];

/// Warm-up and measurement budgets. The whole suite has to stay short enough that
/// it is actually run; ~4 minutes is the target. ~keep
const WARM_UP: Duration = Duration::from_millis(500);
const MEASUREMENT: Duration = Duration::from_secs(2);
/// Sample budget for groups whose iterations cost milliseconds.
const HEAVY_SAMPLES: usize = 20;
/// Sample budget and budget extension for cold query compiles (tens of ms each).
const COLD_COMPILE_SAMPLES: usize = 10;
const COLD_COMPILE_MEASUREMENT: Duration = Duration::from_secs(5);
/// Thread counts for the lock-contention sweep.
const THREAD_COUNTS: &[usize] = &[1, 2, 4, 8];
/// Chunk sizes for the splitter sweep, in bytes.
const CHUNK_SIZES: &[usize] = &[500, 1000, 4000];
const CHUNK_SCALING_FUNCTIONS: &[usize] = &[64, 256, 1024];

/// Download and load every grammar the suite uses, once per process.
///
/// A failure is reported and not fatal: individual cases are skipped by
/// [`skip_missing`] so an offline run still measures whatever is installed. ~keep
fn prepare() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        if let Err(error) = prefetch(BENCH_LANGUAGES) {
            println!("warning: prefetch failed ({error}); cases for missing grammars are skipped");
        }
    });
}

fn skip_missing(language: &str) -> bool {
    prepare();
    if has_parser(language) {
        return false;
    }
    println!("skipping benchmarks for '{language}': parser unavailable");
    true
}

fn feature_config(language: &str, enable: fn(&mut ProcessConfig)) -> ProcessConfig {
    let mut config = ProcessConfig::new(language).minimal();
    enable(&mut config);
    config
}

/// A named extractor stage and the flag that turns it on.
type Extractor = (&'static str, fn(&mut ProcessConfig));

/// One extractor per entry, so a regression lands on the stage that caused it.
const EXTRACTORS: &[Extractor] = &[
    ("structure", |c| c.structure = true),
    ("imports", |c| c.imports = true),
    ("exports", |c| c.exports = true),
    ("comments", |c| c.comments = true),
    ("docstrings", |c| c.docstrings = true),
    ("symbols", |c| c.symbols = true),
    ("diagnostics", |c| c.diagnostics = true),
    ("data_extraction", |c| c.data_extraction = true),
];

const QUERY_KINDS: &[(&str, QueryKind)] = &[
    ("highlights", QueryKind::Highlights),
    ("injections", QueryKind::Injections),
    ("locals", QueryKind::Locals),
    ("tags", QueryKind::Tags),
    ("indents", QueryKind::Indents),
    ("folds", QueryKind::Folds),
];

/// Languages whose cold compile cost is worth tracking, cheapest first.
const COMPILE_CASES: &[(&str, QueryKind)] = &[
    ("go", QueryKind::Highlights),
    ("rust", QueryKind::Highlights),
    ("ruby", QueryKind::Highlights),
    ("swift", QueryKind::Highlights),
    ("swift", QueryKind::Tags),
];

fn bench_registry(c: &mut Criterion) {
    if skip_missing("python") {
        return;
    }
    let mut group = c.benchmark_group("registry");
    group.warm_up_time(WARM_UP).measurement_time(MEASUREMENT);

    group.bench_function("get_language/hit", |b| {
        b.iter(|| get_language(black_box("python")).unwrap());
    });
    if !skip_missing("bash") {
        // ~keep "shell" is an alias for "bash": measures the alias-resolution path.
        group.bench_function("get_language/alias", |b| {
            b.iter(|| get_language(black_box("shell")).unwrap());
        });
    }
    group.bench_function("has_language/known", |b| {
        b.iter(|| has_language(black_box("python")));
    });
    group.bench_function("has_language/unknown", |b| {
        b.iter(|| has_language(black_box("nonexistent_lang_xyz")));
    });
    group.bench_function("has_parser/known", |b| {
        b.iter(|| has_parser(black_box("python")));
    });
    group.bench_function("available_languages", |b| b.iter(available_languages));
    group.bench_function("language_count", |b| b.iter(language_count));
    group.bench_function("detect_language_from_path", |b| {
        b.iter(|| detect_language_from_path(black_box("src/main.rs")));
    });
    group.bench_function("prefetch/all_cached", |b| {
        b.iter(|| prefetch(black_box(BENCH_LANGUAGES)).is_ok());
    });

    group.finish();
}

fn bench_query_cache_hit(c: &mut Criterion) {
    if skip_missing("python") {
        return;
    }
    let mut group = c.benchmark_group("query/cached");
    group.warm_up_time(WARM_UP).measurement_time(MEASUREMENT);

    for &(name, kind) in QUERY_KINDS {
        // ~keep Warm the entry first, or the first call would time a compile.
        let _ = get_query("python", kind);
        group.bench_function(name, |b| {
            b.iter(|| get_query(black_box("python"), black_box(kind)).unwrap());
        });
    }

    group.finish();
}

/// Raw `.scm` source for a language/kind, bypassing the compiled-query cache.
fn query_source(language: &str, kind: QueryKind) -> Option<&'static str> {
    match kind {
        QueryKind::Highlights => get_highlights_query(language),
        QueryKind::Injections => get_injections_query(language),
        QueryKind::Locals => get_locals_query(language),
        QueryKind::Tags => get_tags_query(language),
        QueryKind::Indents => get_indents_query(language),
        QueryKind::Folds => get_folds_query(language),
    }
}

/// True cold compiles: what a [`get_query`] hit saves, every time.
fn bench_query_compile(c: &mut Criterion) {
    let mut group = c.benchmark_group("query/compile_cold");
    group
        .sample_size(COLD_COMPILE_SAMPLES)
        .sampling_mode(SamplingMode::Flat)
        .warm_up_time(WARM_UP)
        .measurement_time(COLD_COMPILE_MEASUREMENT);

    for &(name, kind) in COMPILE_CASES {
        if skip_missing(name) {
            continue;
        }
        let Some(source) = query_source(name, kind) else {
            continue;
        };
        let language = get_language(name).expect("prefetched grammar should load");
        let id = format!("{name}/{kind:?}");
        group.throughput(Throughput::Bytes(source.len() as u64));
        group.bench_function(id.as_str(), |b| {
            b.iter(|| Query::new(black_box(&language), black_box(source)).unwrap());
        });
    }

    group.finish();
}

/// The unlocked parse path: `Parser::parse_bytes`, no intelligence extraction.
fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");
    group
        .sampling_mode(SamplingMode::Flat)
        .warm_up_time(WARM_UP)
        .measurement_time(MEASUREMENT);

    for &(id, language, source) in CASES {
        if skip_missing(language) {
            continue;
        }
        let mut parser = get_parser(language).expect("prefetched grammar should load");
        group.throughput(Throughput::Bytes(source.len() as u64));
        group.bench_function(id, |b| {
            b.iter(|| parser.parse_bytes(black_box(source.as_bytes())).unwrap());
        });
    }

    group.finish();
}

/// The locked path with every extractor on, one case per fixture.
fn bench_process_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("process/all");
    group
        .sample_size(HEAVY_SAMPLES)
        .sampling_mode(SamplingMode::Flat)
        .warm_up_time(WARM_UP)
        .measurement_time(MEASUREMENT);

    for &(id, language, source) in CASES {
        if skip_missing(language) {
            continue;
        }
        let config = ProcessConfig::new(language).all();
        group.throughput(Throughput::Bytes(source.len() as u64));
        group.bench_function(id, |b| {
            b.iter(|| process(black_box(source), black_box(&config)).unwrap());
        });
    }

    group.finish();
}

/// Per-extractor breakdown. `minimal` is the parse-plus-metrics floor; every other
/// case is that floor plus exactly one extractor, so `all` minus `minimal` is the
/// whole extraction pipeline and each entry is one stage's share of it.
fn bench_process_extractors(c: &mut Criterion) {
    if skip_missing("python") {
        return;
    }
    let mut group = c.benchmark_group("process/extractors");
    group
        .sample_size(HEAVY_SAMPLES)
        .sampling_mode(SamplingMode::Flat)
        .warm_up_time(WARM_UP)
        .measurement_time(MEASUREMENT)
        .throughput(Throughput::Bytes(PYTHON_LARGE.len() as u64));

    let minimal = ProcessConfig::new("python").minimal();
    group.bench_function("minimal", |b| {
        b.iter(|| process(black_box(PYTHON_LARGE), black_box(&minimal)).unwrap());
    });

    for &(name, enable) in EXTRACTORS {
        let config = feature_config("python", enable);
        group.bench_function(name, |b| {
            b.iter(|| process(black_box(PYTHON_LARGE), black_box(&config)).unwrap());
        });
    }

    let all = ProcessConfig::new("python").all();
    group.bench_function("all", |b| {
        b.iter(|| process(black_box(PYTHON_LARGE), black_box(&all)).unwrap());
    });

    group.finish();
}

/// The splitter is private, so it is measured as `chunked - none`.
fn bench_chunking(c: &mut Criterion) {
    if skip_missing("python") {
        return;
    }
    let mut group = c.benchmark_group("chunking");
    group
        .sample_size(HEAVY_SAMPLES)
        .sampling_mode(SamplingMode::Flat)
        .warm_up_time(WARM_UP)
        .measurement_time(MEASUREMENT)
        .throughput(Throughput::Bytes(PYTHON_LARGE.len() as u64));

    let baseline = ProcessConfig::new("python").all();
    group.bench_function("none", |b| {
        b.iter(|| process(black_box(PYTHON_LARGE), black_box(&baseline)).unwrap());
    });

    for &size in CHUNK_SIZES {
        let config = ProcessConfig::new("python").all().with_chunking(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &config, |b, config| {
            b.iter(|| process(black_box(PYTHON_LARGE), black_box(config)).unwrap());
        });
    }

    group.finish();

    let mut scaling = c.benchmark_group("chunking_scaling");
    scaling
        .sample_size(HEAVY_SAMPLES)
        .sampling_mode(SamplingMode::Flat)
        .warm_up_time(WARM_UP)
        .measurement_time(MEASUREMENT);
    let config = ProcessConfig::new("python").minimal().with_chunking(128);
    for &function_count in CHUNK_SCALING_FUNCTIONS {
        let source: String = (0..function_count)
            .map(|index| format!("def function_{index}():\n    return {index}\n"))
            .collect();
        scaling.throughput(Throughput::Bytes(source.len() as u64));
        scaling.bench_with_input(BenchmarkId::from_parameter(function_count), &source, |b, source| {
            b.iter(|| process(black_box(source), black_box(&config)).unwrap());
        });
    }
    scaling.finish();
}

/// Wall time for `threads` workers each performing `iters` operations.
///
/// Workers are spawned once per sample and loop internally, so per-thread setup is
/// amortised and both arms pay the same spawn cost. Criterion divides by `iters`,
/// making the reported figure the time for one round of `threads` concurrent
/// operations: flat as `threads` grows means the work scales, rising means it
/// serializes. ~keep
fn threaded_round(threads: usize, iters: u64, worker: &(dyn Fn(u64) + Sync)) -> Duration {
    let start = Instant::now();
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| worker(iters));
        }
    });
    start.elapsed()
}

fn bench_concurrency(c: &mut Criterion) {
    if skip_missing("python") {
        return;
    }
    let mut group = c.benchmark_group("concurrency");
    group
        .sample_size(HEAVY_SAMPLES)
        .sampling_mode(SamplingMode::Flat)
        .warm_up_time(WARM_UP)
        .measurement_time(MEASUREMENT);

    let config = ProcessConfig::new("python").minimal();
    let locked = |iters: u64| {
        for _ in 0..iters {
            black_box(process(PYTHON_MEDIUM, &config).unwrap());
        }
    };
    // ~keep `Parser` is not `Sync`, so each worker builds its own before its loop.
    let unlocked = |iters: u64| {
        let mut parser = get_parser("python").expect("prefetched grammar should load");
        for _ in 0..iters {
            black_box(parser.parse_bytes(PYTHON_MEDIUM.as_bytes()).unwrap());
        }
    };

    for &threads in THREAD_COUNTS {
        group.throughput(Throughput::Bytes((PYTHON_MEDIUM.len() * threads) as u64));
        group.bench_function(BenchmarkId::new("process_locked", threads), |b| {
            b.iter_custom(|iters| threaded_round(threads, iters, &locked));
        });
        group.bench_function(BenchmarkId::new("parse_unlocked", threads), |b| {
            b.iter_custom(|iters| threaded_round(threads, iters, &unlocked));
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_registry,
    bench_query_cache_hit,
    bench_query_compile,
    bench_parse,
    bench_process_all,
    bench_process_extractors,
    bench_chunking,
    bench_concurrency,
);
criterion_main!(benches);
