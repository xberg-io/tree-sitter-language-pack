---
description: "Run and interpret the Criterion benchmarks for tree-sitter-language-pack."
---

# Performance and benchmarks

The Rust core ships a [Criterion](https://bheisler.github.io/criterion.rs/book/) benchmark suite in `crates/ts-pack-core/benches/benchmarks.rs`. This guide explains how to run them and what each group measures.

## Running benchmarks

```bash
cargo bench -p tree-sitter-language-pack
```

Criterion writes HTML reports to `target/criterion/`. Open `target/criterion/report/index.html` in a browser to see throughput charts across runs.

To run a single group:

```bash
# Just the parse benchmarks
cargo bench -p tree-sitter-language-pack -- parse

# Just language detection
cargo bench -p tree-sitter-language-pack -- language_detection
```

## Benchmark groups

The suite covers nine groups. Fixtures compile in from `fixtures/bench/`, with small (~11 lines), medium (~97 lines), and large (~723 lines) variants for each of four languages: Python, TypeScript, Rust, and Go.

### `parse`

Measures `parse_string()` across all four languages at all three sizes. This is the baseline — a single tree-sitter parse with no post-processing.

12 cases: `python/small`, `python/medium`, `python/large`, `typescript/small`, `typescript/medium`, `typescript/large`, `rust/small`, `rust/medium`, `rust/large`, `go/small`, `go/medium`, `go/large`.

### `process`

Measures `process()` with `ProcessConfig::all()` vs `ProcessConfig::minimal()`, on Python medium and large fixtures. Shows the cost of enabling all analysis features vs. extracting nothing.

### `run_query`

Measures a single tree-sitter query against a pre-parsed tree — function definition lookup in Python medium. Useful for understanding raw query throughput when you already have a tree.

### `extract_oneshot`

Measures `extract_patterns()` as a one-shot call (parses and extracts in one step). Baseline for custom extraction without pre-compilation.

### `extract_compiled`

Measures `CompiledExtraction::extract()` — same extraction as `extract_oneshot` but with the query pre-compiled. Shows the amortization benefit of `CompiledExtraction` across repeated calls.

### `extract_from_tree`

Measures `CompiledExtraction::extract_from_tree()` against a pre-parsed tree. Shows extraction cost when the parse is already done separately.

### `validate`

Measures `validate_extraction()` — query validation against the language grammar. A fast operation; this benchmark measures test overhead more than runtime.

### `text_splitter`

Measures `process()` with chunking enabled (`chunk_size = 1000` bytes, Python medium). Shows the overhead of the syntax-aware chunking pass on top of process.

### `language_detection`

Measures the three detection entry points:

| Function | Fixture |
|----------|---------|
| `detect_language_from_extension("py")` | extension lookup |
| `detect_language_from_path("src/main.rs")` | path → extension → lookup |
| `detect_language_from_content("#!/usr/bin/env python3\n")` | shebang scan |

All three are near-zero cost (hash table or memchr scan).

## Reading Criterion output

Criterion prints mean, standard deviation, and change vs. the previous run. A result like:

```text
parse/python/medium  time: [   1.23 µs    1.31 µs    1.41 µs]
```

Means the 95% confidence interval for the mean is 1.23–1.41 µs. On the first run there is no baseline, so criterion does not show a change percentage.

## Comparing across machines

Criterion stores its baselines in `target/criterion/`. Those files do not commit to the repository. To share results, redirect bench output to a file and compare manually, or use [Bencher](https://bencher.dev) for CI-level tracking.

## Profiling

For detailed profiling, build a benchmark binary in profile mode:

```bash
cargo bench -p tree-sitter-language-pack --no-run
# Find the binary
ls target/release/deps/benchmarks-*
# Then run with a profiler, e.g. samply or cargo-flamegraph
```
