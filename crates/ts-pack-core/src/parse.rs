use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::ControlFlow;
use std::sync::{LazyLock, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use ahash::AHashMap;

use crate::Error;

thread_local! {
    /// One `tree_sitter::Parser` per language, per thread.
    ///
    /// Unbounded and never evicted, by design: an entry is a parser plus its
    /// grown-to-high-water-mark scratch buffers, and the alternative — dropping
    /// parsers a hot loop is about to ask for again — trades a bounded, per-thread
    /// footprint for repeated reallocation. The ceiling is
    /// `threads x languages-that-thread-parsed`, which for the intended usage
    /// (a worker pool over a handful of languages) is tens of entries.
    ///
    /// It is *not* bounded by the number of languages in the pack unless the
    /// program actually parses them all: the measured 20.4 MB -> 398.4 MB growth
    /// for all 377 grammars is dominated by the loaded shared libraries and
    /// compiled queries, not by this map. Retained `Tree`s are the other large
    /// consumer and belong to the caller — a 23.5 KB source retains ~620 KB of
    /// tree, so holding 100 of them costs ~62 MB no cache can reclaim. ~keep
    static PARSER_CACHE: RefCell<AHashMap<String, tree_sitter::Parser>> = RefCell::new(AHashMap::new());
}

/// Language names whose external scanner keeps mutable process-global state, and therefore need
/// parses of that language serialized across threads. Every language *not* in this list parses
/// fully in parallel — no lock is taken for it at all.
///
/// Derived by auditing all 199 external-scanner sources (`scanner.c`/`.cc`/`.cpp`/`.h`) under
/// `parsers/` and `grammars/`, checking four shapes: single-line non-const statics, multi-line
/// static initializers, non-static file-scope declarations, and non-const static arrays.
/// `properties`'s `scanner.c` is the only one that genuinely mutates a static during scanning
/// (`reached_eof`, read and written across `scan`); every other candidate found — purescript,
/// unison, and idris's `res_cont`/`res_fail` sentinels, and haskell's `debug_parse_metadata` — is
/// returned by value or read-only, never assigned, and is benign.
///
/// Must be kept sorted (checked in tests) and re-derived whenever grammars are updated — repo
/// task #50 tracks 45 stale grammars that have not been re-audited against this list. ~keep
const STATEFUL_SCANNER_LANGUAGES: &[&str] = &["properties"];

/// One `Mutex<()>` per entry in [`STATEFUL_SCANNER_LANGUAGES`], built once on first use.
///
/// A `HashMap` behind a `LazyLock` was chosen over one hand-written `static Mutex` per language:
/// the allowlist is a slice today, not a fixed set of named identifiers, so this scales to a
/// longer list without a matching `match` arm per entry, and — since the map is never mutated
/// after `LazyLock` builds it — no outer lock is needed to guard its structure, only the `Mutex`
/// each entry holds. ~keep
static STATEFUL_SCANNER_LOCKS: LazyLock<HashMap<&'static str, Mutex<()>>> = LazyLock::new(|| {
    STATEFUL_SCANNER_LANGUAGES
        .iter()
        .map(|&name| (name, Mutex::new(())))
        .collect()
});

/// Take the per-language lock for `language_name`, if that language needs one.
///
/// Returns `None` for every language outside [`STATEFUL_SCANNER_LANGUAGES`] — the overwhelming
/// majority — so that no lock at all stands between concurrent threads parsing different (or the
/// same) non-stateful language.
pub(crate) fn lock_for_stateful_scanner(language_name: &str) -> Option<MutexGuard<'static, ()>> {
    let lock = STATEFUL_SCANNER_LOCKS.get(language_name)?;
    Some(
        lock.lock()
            .unwrap_or_else(|poisoned| crate::recover_poisoned_lock("parse", poisoned)),
    )
}

/// Parse source code with a pre-loaded `Language`, using the thread-local
/// parser cache, cancelling the parse if it exceeds `timeout_ms` milliseconds
/// of wall clock.
///
/// Taking the `Language` avoids a redundant registry lookup when the caller
/// already has one (e.g. from `LanguageRegistry::get_language`).
///
/// `None` means no budget, which is the default everywhere so that existing
/// callers are unaffected.
///
/// Cancellation is granular to tree-sitter's progress-callback interval rather
/// than exact: the callback fires periodically during the parse, so a parse can
/// overrun the budget by up to one interval before it is stopped.
///
/// Serializes against other parses of the same language only when `language_name` is in
/// [`STATEFUL_SCANNER_LANGUAGES`]; every other language runs this function fully in parallel
/// across threads. ~keep
pub(crate) fn parse_with_language_limited(
    language_name: &str,
    language: &tree_sitter::Language,
    source: &[u8],
    timeout_ms: Option<u64>,
) -> Result<tree_sitter::Tree, Error> {
    let _guard = lock_for_stateful_scanner(language_name);
    PARSER_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(parser) = cache.get_mut(language_name) {
            return run_parse(parser, source, None, timeout_ms);
        }
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(language)
            .map_err(|e| Error::ParserSetup(format!("{e}")))?;
        let tree = run_parse(&mut parser, source, None, timeout_ms)?;
        cache.insert(language_name.to_string(), parser);
        Ok(tree)
    })
}

/// Run one parse on an already-configured parser, honouring an optional budget.
pub(crate) fn run_parse(
    parser: &mut tree_sitter::Parser,
    source: &[u8],
    old_tree: Option<&tree_sitter::Tree>,
    timeout_ms: Option<u64>,
) -> Result<tree_sitter::Tree, Error> {
    let len = source.len();
    // ~keep Same reader `tree_sitter::Parser::parse` builds internally. The chunk-callback entry
    // ~keep point is the only one that also takes `ParseOptions`, which carries the cancellation
    // ~keep hook; `set_timeout_micros` no longer exists in tree-sitter 0.26.
    let mut read = |offset: usize, _: tree_sitter::Point| {
        if offset < len {
            &source[offset..]
        } else {
            Default::default()
        }
    };

    let deadline = match timeout_ms {
        Some(budget_ms) => Instant::now().checked_add(Duration::from_millis(budget_ms)),
        None => None,
    };

    let Some(deadline) = deadline else {
        // ~keep No budget configured, or a budget so large its deadline is unrepresentable.
        return parser
            .parse_with_options(&mut read, old_tree, None)
            .ok_or(Error::ParseFailed);
    };

    let mut on_progress = |_: &tree_sitter::ParseState| {
        if Instant::now() < deadline {
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(())
        }
    };
    let options = tree_sitter::ParseOptions::new().progress_callback(&mut on_progress);

    match parser.parse_with_options(&mut read, old_tree, Some(options)) {
        Some(tree) => Ok(tree),
        None if Instant::now() >= deadline => {
            let budget_ms = timeout_ms.unwrap_or_default();
            tracing::warn!(
                timeout_ms = budget_ms,
                source_bytes = len,
                "parse cancelled: exceeded the configured wall-clock budget"
            );
            Err(Error::ParseTimeout { timeout_ms: budget_ms })
        }
        None => Err(Error::ParseFailed),
    }
}

#[cfg(test)]
pub(crate) fn cached_parser_count_for_tests() -> usize {
    PARSER_CACHE.with(|cache| cache.borrow().len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn skip_if_no_languages() -> bool {
        crate::available_languages().is_empty()
    }

    fn parse_for_test(language_name: &str, source: &[u8]) -> Result<tree_sitter::Tree, Error> {
        let language = crate::get_language(language_name)?;
        parse_with_language_limited(language_name, &language, source, None)
    }

    #[test]
    fn test_parse_with_language_success() {
        if skip_if_no_languages() {
            return;
        }
        let langs = crate::available_languages();
        let first = &langs[0];
        let tree = parse_for_test(first, b"x");
        assert!(tree.is_ok(), "parse_with_language should succeed for '{first}'");
    }

    #[test]
    fn test_get_language_invalid_language() {
        let result = crate::get_language("nonexistent_xyz");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_language_reuses_cache() {
        if skip_if_no_languages() {
            return;
        }
        let langs = crate::available_languages();
        let first = &langs[0];
        let lang = crate::get_language(first).unwrap();
        let _ = parse_with_language_limited(first, &lang, b"x", None).unwrap();
        let after_first = cached_parser_count_for_tests();
        let _ = parse_with_language_limited(first, &lang, b"y", None).unwrap();
        let after_second = cached_parser_count_for_tests();
        assert_eq!(after_first, after_second, "second call should reuse cached parser");
    }

    #[test]
    fn test_different_languages_get_separate_cache_entries() {
        let langs = crate::available_languages();
        if langs.len() < 2 {
            return;
        }
        let before = cached_parser_count_for_tests();
        let _ = parse_for_test(&langs[0], b"x").unwrap();
        let _ = parse_for_test(&langs[1], b"x").unwrap();
        let after = cached_parser_count_for_tests();
        assert!(
            after >= before + 2,
            "different languages should create separate cache entries"
        );
    }

    #[test]
    fn should_keep_parsing_after_a_panic_poisons_a_stateful_scanner_lock() {
        let stateful_language = "properties";
        if crate::get_language(stateful_language).is_err() {
            // ~keep No-ops without the `properties` grammar linked in; there is nothing else in
            // ~keep STATEFUL_SCANNER_LANGUAGES to poison-test against.
            return;
        }
        let language = crate::get_language(stateful_language).expect("language should be loadable");
        let lock = STATEFUL_SCANNER_LOCKS
            .get(stateful_language)
            .expect("stateful_language must have an entry in STATEFUL_SCANNER_LOCKS");

        // ~keep The stateful-scanner locks guard no data of their own (they only serialize
        // ~keep third-party scanner global state), so poisoning one should never brick parsing.
        // ~keep Poisoning is process-wide and sticky (std::sync::Mutex never un-poisons), which
        // ~keep is safe to do here only because every acquisition site recovers via
        // ~keep `recover_poisoned_lock` — this test leaves the `properties` lock poisoned for
        // ~keep the rest of the process on purpose, and every sibling test that parses
        // ~keep `properties` afterward exercises that same recovery path.
        let poison_result = std::panic::catch_unwind(|| {
            let _guard = lock.lock().expect("lock should not already be poisoned");
            panic!("intentional panic to poison the stateful-scanner lock for this test");
        });
        assert!(poison_result.is_err(), "the intentional panic should have unwound");
        assert!(lock.is_poisoned(), "the lock should be poisoned after the panic");

        let tree = parse_with_language_limited(stateful_language, &language, b"key = value", None);
        assert!(
            tree.is_ok(),
            "parse_with_language_limited must recover a poisoned stateful-scanner lock instead of \
             failing forever, got: {:?}",
            tree.err()
        );
    }

    #[test]
    fn should_not_serialize_when_two_different_non_stateful_languages_parse_concurrently() {
        // ~keep Proves absence of serialization structurally rather than by timing: if a future
        // ~keep regression made these two languages share a lock, thread A would hold it across
        // ~keep `barrier.wait()` while blocked on thread B, and thread B could never acquire the
        // ~keep lock to reach its own `barrier.wait()` — a real deadlock, not a race. The
        // ~keep `recv_timeout` below turns that deadlock into a clean test failure instead of a
        // ~keep hung test run.
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let barrier_a = std::sync::Arc::clone(&barrier);
        let barrier_b = std::sync::Arc::clone(&barrier);

        let handle_a = std::thread::spawn(move || {
            let _guard = lock_for_stateful_scanner("python");
            barrier_a.wait();
        });
        let handle_b = std::thread::spawn(move || {
            let _guard = lock_for_stateful_scanner("rust");
            barrier_b.wait();
        });

        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let _ = handle_a.join();
            let _ = handle_b.join();
            let _ = tx.send(());
        });

        assert!(
            rx.recv_timeout(Duration::from_secs(5)).is_ok(),
            "two different non-stateful languages must not serialize on a shared lock: both \
             threads need to reach the barrier while still holding their own guard, which a \
             shared lock would deadlock"
        );
    }

    #[test]
    fn should_serialize_two_threads_parsing_the_same_stateful_scanner_language() {
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let intervals: std::sync::Arc<Mutex<Vec<(Instant, Instant)>>> = std::sync::Arc::new(Mutex::new(Vec::new()));
        const HOLD_TIME: Duration = Duration::from_millis(50);

        let spawn_holder = || {
            let barrier = std::sync::Arc::clone(&barrier);
            let intervals = std::sync::Arc::clone(&intervals);
            std::thread::spawn(move || {
                barrier.wait();
                let _guard = lock_for_stateful_scanner("properties");
                let start = Instant::now();
                std::thread::sleep(HOLD_TIME);
                let end = Instant::now();
                intervals
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .push((start, end));
            })
        };

        let handle_a = spawn_holder();
        let handle_b = spawn_holder();
        handle_a.join().expect("holder thread A should not panic");
        handle_b.join().expect("holder thread B should not panic");

        let recorded = intervals.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        assert_eq!(recorded.len(), 2, "both threads should have recorded an interval");
        let (start_a, end_a) = recorded[0];
        let (start_b, end_b) = recorded[1];
        let overlap = start_a < end_b && start_b < end_a;
        assert!(
            !overlap,
            "same-language stateful-scanner parses must be serialized, but intervals overlapped: \
             {start_a:?}..{end_a:?} vs {start_b:?}..{end_b:?}"
        );
    }

    #[test]
    fn should_keep_stateful_scanner_languages_sorted_and_known() {
        let mut sorted = STATEFUL_SCANNER_LANGUAGES.to_vec();
        sorted.sort_unstable();
        assert_eq!(
            STATEFUL_SCANNER_LANGUAGES,
            sorted.as_slice(),
            "STATEFUL_SCANNER_LANGUAGES must be kept sorted"
        );

        if skip_if_no_languages() {
            // ~keep The "known language" half of this test no-ops without any grammar linked in.
            return;
        }
        for &name in STATEFUL_SCANNER_LANGUAGES {
            assert!(
                crate::get_language(name).is_ok(),
                "STATEFUL_SCANNER_LANGUAGES entry '{name}' is not a known language — check for a \
                 typo or a removed grammar"
            );
        }
    }
}
