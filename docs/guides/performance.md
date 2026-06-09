---
description: "Run and interpret the Criterion benchmarks for tree-sitter-language-pack."
---

# Performance and benchmarks

The Rust core ships a [Criterion](https://bheisler.github.io/criterion.rs/book/) benchmark
suite in `crates/ts-pack-core/benches/benchmarks.rs`. This guide explains how to run them
and what each group measures.

## Running benchmarks

```bash
cargo bench -p tree-sitter-language-pack
```

Criterion writes HTML reports to `target/criterion/`. Open `target/criterion/report/index.html`
in a browser to see throughput charts across runs.

To run a single group:

```bash
# Just the parse benchmarks
cargo bench -p tree-sitter-language-pack -- parse

# Just language detection
cargo bench -p tree-sitter-language-pack -- language_detection
```

## Benchmark groups

The suite covers four groups. Fixtures compile in from `fixtures/bench/`, with small (~11
lines), medium (~97 lines), and large (~723 lines) variants for each of four languages:
Python, TypeScript, Rust, and Go.

### `parse`

Measures `get_parser(...).parse(...)` across all four languages at all three sizes. This is
the baseline — a single tree-sitter parse with no post-processing.

12 cases: `python/small`, `python/medium`, `python/large`, `typescript/small`,
`typescript/medium`, `typescript/large`, `rust/small`, `rust/medium`, `rust/large`,
`go/small`, `go/medium`, `go/large`.

### `process`

Measures `process()` with `ProcessConfig::all()` vs `ProcessConfig::minimal()`, on Python
medium and large fixtures. Shows the cost of enabling all analysis features vs. extracting
nothing.

### `text_splitter`

Measures `process()` with chunking enabled (`chunk_size = 1000` bytes, Python medium). Shows
the overhead of the syntax-aware chunking pass on top of process.

### `language_detection`

Measures the three detection entry points:

| Function                                                   | Fixture                   |
| ---------------------------------------------------------- | ------------------------- |
| `detect_language_from_extension("py")`                     | extension lookup          |
| `detect_language_from_path("src/main.rs")`                 | path → extension → lookup |
| `detect_language_from_content("#!/usr/bin/env python3\n")` | shebang scan              |

All three are near-zero cost (hash table or memchr scan).

## Reading Criterion output

Criterion prints mean, standard deviation, and change vs. the previous run. A result
like:

```text
parse/python/medium  time: [   1.23 µs    1.31 µs    1.41 µs]
```

Means the 95% confidence interval for the mean is 1.23–1.41 µs. On the first run there is
no baseline, so criterion does not show a change percentage.

## Comparing across machines

Criterion stores its baselines in `target/criterion/`. Those files do not commit to the
repository. To share results, redirect bench output to a file and compare manually, or use
[Bencher](https://bencher.dev) for CI-level tracking.

## Profiling

For detailed profiling, build a benchmark binary in profile mode:

```bash
cargo bench -p tree-sitter-language-pack --no-run
# Find the binary
ls target/release/deps/benchmarks-*
# Then run with a profiler, e.g. samply or cargo-flamegraph
```
