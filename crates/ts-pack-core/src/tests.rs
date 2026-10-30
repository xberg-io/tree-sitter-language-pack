use super::*;

#[test]
fn test_available_languages() {
    let langs = available_languages();
    let _ = langs;
}

#[test]
fn test_has_language() {
    let langs = available_languages();
    if !langs.is_empty() {
        assert!(has_language(&langs[0]));
    }
    assert!(!has_language("nonexistent_language_xyz"));
}

#[test]
fn test_get_language_invalid() {
    let result = get_language("nonexistent_language_xyz");
    assert!(result.is_err());
}

#[test]
#[ignore = "loads all 372 dynamic libraries — run with --ignored"]
fn test_get_language_and_parse() {
    let langs = available_languages();
    for lang_name in &langs {
        let lang =
            get_language(lang_name.as_str()).unwrap_or_else(|e| panic!("Failed to load language '{lang_name}': {e}"));
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&lang)
            .unwrap_or_else(|e| panic!("Failed to set language '{lang_name}': {e}"));
        let tree = parser.parse("x", None);
        assert!(tree.is_some(), "Parser for '{lang_name}' should parse a string");
    }
}

#[test]
fn test_get_parser() {
    let langs = available_languages();
    if let Some(first) = langs.first() {
        let parser = get_parser(first.as_str());
        assert!(parser.is_ok(), "get_parser should succeed for '{first}'");
    }
}

#[test]
fn test_pack_config_default() {
    let config = PackConfig::default();
    assert!(config.cache_dir.is_none());
    assert!(config.languages.is_none());
    assert!(config.groups.is_none());
}

#[cfg(feature = "download")]
#[test]
fn should_select_known_but_not_downloaded_languages_for_download() {
    // ~keep Regression for #201: an empty libs dir makes `get_language` fail for a
    // ~keep known language while `has_language` still reports it present (a download
    // ~keep build recognises the whole manifest), which is exactly the gap that made
    // ~keep `download([...])` a no-op. Selection must key on on-disk loadability.
    let empty = std::env::temp_dir().join("tslp-201-nonexistent-libs");
    let registry = LanguageRegistry::with_libs_dir(empty);
    let is_static = |name: &str| crate::registry::STATIC_LANGUAGES.iter().any(|&(s, _)| s == name);
    if let Some(&(linked, _)) = crate::registry::STATIC_LANGUAGES.first() {
        assert!(
            languages_needing_download(&registry, [linked]).is_empty(),
            "a statically linked language must never be selected for download"
        );
    }
    // ~keep Statically linked grammars are loadable without a download, so the target
    // ~keep must be one this build did not compile in (TSLP_LANGUAGES varies per run).
    // ~keep A build linking every grammar has nothing left to download.
    let Some(target) = crate::registry::KNOWN_LANGUAGES
        .iter()
        .copied()
        .find(|name| !is_static(name))
    else {
        return;
    };
    assert!(
        registry.has_language(target),
        "a download-enabled build must report canonical manifest languages as known"
    );
    let selected = languages_needing_download(&registry, [target]);
    assert_eq!(
        selected,
        vec![target],
        "a known-but-not-on-disk language must be selected for download"
    );
}

#[cfg(feature = "download")]
#[test]
fn should_keep_configuring_after_a_panic_poisons_the_download_cache_lock() {
    // ~keep DOWNLOAD_CACHE_LOCK guards no data (it only serializes the
    // ~keep read-download-register sequence), so poisoning it should never brick every
    // ~keep subsequent call. Poisoning is process-wide and sticky (std::sync::Mutex never
    // ~keep un-poisons); safe to leave poisoned for the rest of the process here only
    // ~keep because every acquisition site recovers via `recover_poisoned_lock`. This test
    // ~keep does not set `cache_dir`, so unlike a reconfiguration test it cannot corrupt
    // ~keep any sibling test's view of the cache directory.
    let poison_result = std::panic::catch_unwind(|| {
        let _guard = DOWNLOAD_CACHE_LOCK
            .lock()
            .expect("DOWNLOAD_CACHE_LOCK should not already be poisoned");
        panic!("intentional panic to poison DOWNLOAD_CACHE_LOCK for this test");
    });
    assert!(poison_result.is_err(), "the intentional panic should have unwound");
    assert!(
        DOWNLOAD_CACHE_LOCK.is_poisoned(),
        "DOWNLOAD_CACHE_LOCK should be poisoned after the panic"
    );

    let result = configure(&PackConfig {
        cache_dir: None,
        languages: None,
        groups: None,
    });
    assert!(
        result.is_ok(),
        "configure must recover a poisoned DOWNLOAD_CACHE_LOCK instead of failing forever, got: {result:?}"
    );
}

#[cfg(feature = "download")]
#[test]
#[ignore = "mutates the process-wide CUSTOM_CACHE_DIR/REGISTERED_CACHE_DIR statics; run \
            alone, e.g. `cargo test -- --ignored --test-threads=1`, not inside the default \
            parallel suite"]
fn should_register_the_new_cache_dir_after_configure_changes_it() {
    // ~keep Regression for item 1: the old `CACHE_REGISTERED` boolean could get stuck
    // ~keep `true` forever after a race with `configure()`, permanently registering only
    // ~keep the OLD cache dir. The path-CAS fix in `ensure_cache_registered` compares the
    // ~keep actual registered path on every call, so the very next call anywhere after a
    // ~keep reconfiguration observes and registers the new path — this asserts exactly
    // ~keep that, deterministically, without needing to reproduce the original race.
    let first_base = std::env::temp_dir().join("tslp-item1-regression-first");
    let second_base = std::env::temp_dir().join("tslp-item1-regression-second");
    // ~keep `cache_dir` is a BASE directory (#101 H1): the registered path is suffixed.
    let first_dir = DownloadManager::cache_dir_from_base(&first_base, env!("CARGO_PKG_VERSION"));
    let second_dir = DownloadManager::cache_dir_from_base(&second_base, env!("CARGO_PKG_VERSION"));

    configure(&PackConfig {
        cache_dir: Some(first_base.clone()),
        languages: None,
        groups: None,
    })
    .expect("configure should accept the first cache dir");
    let _ = has_language("definitely_not_a_real_language_xyz");
    {
        let registered = REGISTERED_CACHE_DIR.read().expect("read lock should not be poisoned");
        assert_eq!(registered.as_deref(), Some(first_dir.as_path()));
    }

    configure(&PackConfig {
        cache_dir: Some(second_base.clone()),
        languages: None,
        groups: None,
    })
    .expect("configure should accept the second cache dir");
    let _ = has_language("definitely_not_a_real_language_xyz");
    {
        let registered = REGISTERED_CACHE_DIR.read().expect("read lock should not be poisoned");
        assert_eq!(
            registered.as_deref(),
            Some(second_dir.as_path()),
            "reconfiguring the cache dir must be observed by the very next call, \
             not stuck on the old path forever"
        );
    }
}

#[cfg(feature = "download")]
#[test]
#[ignore = "mutates the process-wide CUSTOM_CACHE_DIR static; run alone, e.g. \
            `cargo test -- --ignored --test-threads=1`, not inside the default parallel suite"]
fn should_root_effective_cache_dir_under_the_configured_base_when_cache_dir_is_custom() {
    // ~keep Regression for #101 H1: `PackConfig::cache_dir` used to be forwarded
    // ~keep verbatim, so `DownloadManager::version_cache_dir()`
    // ~keep (`self.cache_dir.parent()`) resolved OUTSIDE the caller's configured
    // ~keep directory entirely — `manifest.json`, `bundles/`, and `.download.lock` all
    // ~keep ended up in the *parent* of whatever the caller configured.
    let base = std::env::temp_dir().join("tslp-h1-regression-base");

    configure(&PackConfig {
        cache_dir: Some(base.clone()),
        languages: None,
        groups: None,
    })
    .expect("configure should accept a custom base directory");

    let dir = effective_cache_dir().expect("effective cache dir should resolve");

    assert!(
        dir.starts_with(&base),
        "effective cache dir {} must live under the configured base {}",
        dir.display(),
        base.display()
    );
    assert_eq!(
        dir,
        base.join("tree-sitter-language-pack")
            .join(format!("v{}", env!("CARGO_PKG_VERSION")))
            .join("libs")
    );
}
