use super::*;
use crate::process_config::ProcessConfig;

fn first_available_lang() -> Option<String> {
    let registry = LanguageRegistry::new();
    let langs = registry.available_languages();
    langs.into_iter().next()
}

#[cfg(all(feature = "dynamic-loading", not(target_arch = "wasm32")))]
mod abi_gate {
    use super::super::dynamic::is_supported_abi;

    #[test]
    fn should_accept_every_abi_the_linked_runtime_declares_compatible() {
        for abi in tree_sitter::MIN_COMPATIBLE_LANGUAGE_VERSION..=tree_sitter::LANGUAGE_VERSION {
            assert!(
                is_supported_abi(abi),
                "runtime declares {abi} compatible but the gate rejects it"
            );
        }
    }

    #[test]
    fn should_reject_an_abi_below_the_minimum_compatible_version() {
        let below = tree_sitter::MIN_COMPATIBLE_LANGUAGE_VERSION - 1;
        assert!(
            !is_supported_abi(below),
            "ABI {below} is below the runtime minimum and must be rejected"
        );
    }

    #[test]
    fn should_reject_an_abi_above_the_runtime_version() {
        let above = tree_sitter::LANGUAGE_VERSION + 1;
        assert!(
            !is_supported_abi(above),
            "ABI {above} is newer than the runtime and must be rejected"
        );
    }
}

#[cfg(any(feature = "dynamic-loading", feature = "download"))]
#[test]
fn lib_filename_maps_c_symbol_back_to_language_name() {
    assert_eq!(
        lang_name_from_lib_filename("libtree_sitter_c_sharp.so").as_deref(),
        Some("csharp")
    );
    assert_eq!(
        lang_name_from_lib_filename("libtree_sitter_TSQL.dylib").as_deref(),
        Some("tsql")
    );
    assert_eq!(
        lang_name_from_lib_filename("tree_sitter_apache_avro.dll").as_deref(),
        Some("avro")
    );
    assert_eq!(
        lang_name_from_lib_filename("libtree-sitter-nu.so").as_deref(),
        Some("nushell")
    );
    assert_eq!(
        lang_name_from_lib_filename("libtree_sitter_rust.so").as_deref(),
        Some("rust")
    );
    assert_eq!(lang_name_from_lib_filename("README.md"), None);
    assert_eq!(lang_name_from_lib_filename("libtree_sitter_rust.txt"), None);
}

#[cfg(any(feature = "dynamic-loading", feature = "download"))]
#[test]
fn library_file_name_applies_the_c_symbol_override() {
    assert!(
        library_file_name("csharp").contains("tree_sitter_c_sharp."),
        "{}",
        library_file_name("csharp")
    );
    assert!(
        library_file_name("rust").contains("tree_sitter_rust."),
        "{}",
        library_file_name("rust")
    );
}

#[cfg(feature = "dynamic-loading")]
#[test]
fn available_languages_reports_language_names_not_c_symbols_for_extra_dirs() {
    let dir = tempfile::Builder::new()
        .prefix("tslp-extra-libs-")
        .tempdir()
        .expect("temporary libs directory should be created");
    for symbol in ["c_sharp", "TSQL", "apache_avro", "embedded_template", "nu", "vb_dotnet"] {
        std::fs::write(dir.path().join(format!("libtree_sitter_{symbol}.so")), b"")
            .expect("stub parser library should be written");
    }

    let registry = LanguageRegistry::new();
    registry
        .try_add_extra_libs_dir(dir.path().to_path_buf())
        .expect("registering an extra libs dir should succeed");
    let langs = registry.available_languages();

    for (symbol, language) in [
        ("c_sharp", "csharp"),
        ("TSQL", "tsql"),
        ("apache_avro", "avro"),
        ("embedded_template", "embeddedtemplate"),
        ("nu", "nushell"),
        ("vb_dotnet", "vb"),
    ] {
        assert!(langs.iter().any(|l| l == language), "missing language name {language}");
        assert!(
            !langs.iter().any(|l| l == symbol),
            "leaked raw C symbol {symbol} into available_languages()"
        );
    }
    assert_eq!(registry.language_count(), langs.len());
}

#[cfg(all(feature = "dynamic-loading", not(target_arch = "wasm32")))]
#[test]
fn should_distinguish_an_absent_library_from_an_unloadable_one() {
    // ~keep get_language falls through to the extra lib dirs on LanguageNotFound ONLY, so that a
    // ~keep relocated build can still be satisfied by the download cache. If `load` ever reports an
    // ~keep absent file as DynamicLoad — or an unloadable one as LanguageNotFound — that fallthrough
    // ~keep silently stops working and a corrupt parser starts reading as "no such language".
    let dir = tempfile::Builder::new()
        .prefix("tslp-load-semantics-")
        .tempdir()
        .expect("temporary libs directory should be created");
    let loader = super::dynamic::DynamicLoader::new(dir.path().to_path_buf(), vec!["python"]);

    match loader.load("python") {
        Err(Error::LanguageNotFound(_)) => {}
        Ok(_) => panic!("an absent library must not load successfully"),
        Err(error) => panic!("an absent library must report LanguageNotFound, got: {error}"),
    }

    std::fs::write(super::lib_path_in(dir.path(), "python"), b"not a shared library")
        .expect("stub parser library should be written");

    match loader.load("python") {
        Err(Error::DynamicLoad(_)) => {}
        Ok(_) => panic!("a truncated library must not load successfully"),
        Err(error) => panic!("a present-but-unloadable library must report DynamicLoad, got: {error}"),
    }
}

#[cfg(feature = "dynamic-loading")]
#[test]
fn should_recover_a_poisoned_read_but_propagate_a_poisoned_write_of_extra_lib_dirs() {
    // ~keep A fresh, local `LanguageRegistry` — this test never touches process-wide
    // ~keep statics, so it is safe to run in any order and in parallel with every
    // ~keep other test in the suite.
    let registry = LanguageRegistry::new();

    // ~keep Poison `extra_lib_dirs` by panicking while holding its write guard.
    let dirs_arc = Arc::clone(&registry.extra_lib_dirs);
    let poison_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = dirs_arc.write().expect("lock should not already be poisoned");
        panic!("intentional panic to poison extra_lib_dirs for this test");
    }));
    assert!(poison_result.is_err(), "the intentional panic should have unwound");
    assert!(
        registry.extra_lib_dirs.is_poisoned(),
        "extra_lib_dirs should be poisoned after the panic"
    );

    // ~keep Item 4: the read side recovers — `extra_lib_dirs` only ever grows, so a
    // ~keep poisoned read is still coherent, and reporting a language absent because of
    // ~keep an unrelated panic elsewhere would be a wrong answer, not a safe failure.
    let snapshot = registry.extra_lib_dirs_snapshot();
    assert!(
        snapshot.is_empty(),
        "no directories were ever registered, so the recovered snapshot must be empty"
    );

    // ~keep Item 2: the write side propagates instead of silently doing nothing — a
    // ~keep caller registering a cache directory must be able to tell registration failed.
    let result = registry.try_add_extra_libs_dir(PathBuf::from("/tmp/should-not-be-registered"));
    assert!(
        matches!(result, Err(Error::LockPoisoned(_))),
        "try_add_extra_libs_dir must surface a poisoned lock as an error, got: {result:?}"
    );
}

#[cfg(all(feature = "dynamic-loading", not(target_arch = "wasm32")))]
#[test]
fn should_keep_loading_dynamic_languages_after_a_panic_poisons_the_language_load_lock() {
    let registry = LanguageRegistry::new();
    let Some(language_name) = registry.dynamic_loader.dynamic_names.first().copied() else {
        return;
    };

    // ~keep LANGUAGE_LOAD_LOCK guards no data (it only serializes not-yet-loaded dynamic
    // ~keep library loads), so poisoning it should never brick `get_language`. Poisoning
    // ~keep is process-wide and sticky, which is safe here only because every acquisition
    // ~keep site recovers via `recover_poisoned_lock`; this test leaves the lock poisoned
    // ~keep for the rest of the process on purpose.
    let poison_result = std::panic::catch_unwind(|| {
        let _guard = LANGUAGE_LOAD_LOCK
            .lock()
            .expect("LANGUAGE_LOAD_LOCK should not already be poisoned");
        panic!("intentional panic to poison LANGUAGE_LOAD_LOCK for this test");
    });
    assert!(poison_result.is_err(), "the intentional panic should have unwound");
    assert!(
        LANGUAGE_LOAD_LOCK.is_poisoned(),
        "LANGUAGE_LOAD_LOCK should be poisoned after the panic"
    );

    let result = registry.get_language(language_name);
    assert!(
        result.is_ok(),
        "get_language must recover a poisoned LANGUAGE_LOAD_LOCK instead of failing forever, got: {result:?}"
    );
}

#[cfg(feature = "download")]
#[test]
fn known_languages_is_sorted() {
    assert!(
        KNOWN_LANGUAGES.windows(2).all(|pair| pair[0] < pair[1]),
        "KNOWN_LANGUAGES must stay sorted: has_language() binary-searches it"
    );
}

#[cfg(feature = "download")]
#[test]
fn known_language_matches_linear_scan() {
    for name in KNOWN_LANGUAGES {
        assert!(known_language(name), "binary search missed known language {name}");
    }
    assert!(!known_language("nonexistent_lang_xyz"));
    assert!(!known_language(""));
}

#[test]
fn test_registry_process() {
    let Some(lang) = first_available_lang() else { return };
    let registry = LanguageRegistry::new();
    let config = ProcessConfig::new(&lang);
    let result = registry.process("x", &config);
    assert!(result.is_ok(), "registry.process() should succeed");
    let intel = result.unwrap();
    assert_eq!(intel.language, lang);
    assert!(intel.metrics.total_lines >= 1);
}

#[test]
fn test_registry_process_with_chunking() {
    let Some(lang) = first_available_lang() else { return };
    let registry = LanguageRegistry::new();
    let config = ProcessConfig::new(&lang).with_chunking(1000);
    let result = registry.process("x", &config);
    assert!(result.is_ok(), "registry.process() with chunking should succeed");
    let intel = result.unwrap();
    assert_eq!(intel.language, lang);
    assert!(!intel.chunks.is_empty());
}

#[test]
fn test_registry_process_invalid_language() {
    let registry = LanguageRegistry::new();
    let config = ProcessConfig::new("nonexistent_lang_xyz");
    let result = registry.process("x", &config);
    assert!(result.is_err());
}

#[test]
fn test_registry_has_language_and_count() {
    let registry = LanguageRegistry::new();
    let langs = registry.available_languages();
    assert_eq!(registry.language_count(), langs.len());
    if let Some(lang) = langs.first() {
        assert!(registry.has_language(lang));
    }
    assert!(!registry.has_language("nonexistent_lang_xyz"));
}

#[cfg(all(feature = "dynamic-loading", not(target_arch = "wasm32")))]
#[test]
fn test_dynamic_language_survives_registry_drop() {
    let language_name = {
        let registry = LanguageRegistry::new();
        let Some(name) = registry.dynamic_loader.dynamic_names.first() else {
            return;
        };
        (*name).to_string()
    };
    let language = {
        let registry = LanguageRegistry::new();
        let Ok(language) = registry.get_language(&language_name) else {
            return;
        };
        language
    };

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&language)
        .expect("language should remain valid after registry drop");
    let tree = parser.parse("x", None).expect("parser should work after registry drop");
    assert!(tree.root_node().end_byte() <= 1);
}

#[cfg(all(feature = "dynamic-loading", not(target_arch = "wasm32")))]
#[test]
fn test_dynamic_library_is_loaded_once_across_registries() {
    let registry = LanguageRegistry::new();
    let Some(language_name) = registry.dynamic_loader.dynamic_names.first().copied() else {
        return;
    };
    drop(registry);

    let first = LanguageRegistry::new();
    let lib_path = lib_path_in(&first.dynamic_loader.libs_dir, language_name);
    let first_language = first
        .get_language(language_name)
        .expect("first registry should load dynamic language");
    assert!(
        dynamic::library_is_loaded_for_tests(&lib_path),
        "first registry should have populated the process-wide cache for {}",
        lib_path.display()
    );

    let second = LanguageRegistry::new();
    let second_language = second
        .get_language(language_name)
        .expect("second registry should reuse process-loaded dynamic library");

    // ~keep Asserted per-path, not on the global library count. LOADED_LIBRARIES is keyed by
    // ~keep path, so a redundant dlopen would REPLACE the same key and leave the count
    // ~keep unchanged — the previous count comparison could not observe the property this test
    // ~keep is named for, and raced with every other test that loads a grammar.
    assert!(
        dynamic::library_is_loaded_for_tests(&lib_path),
        "second registry must reuse the cached library rather than evict it"
    );
    assert_eq!(first_language.abi_version(), second_language.abi_version());
}

#[cfg(all(feature = "dynamic-loading", not(target_arch = "wasm32")))]
#[test]
fn should_keep_loaded_libraries_present_after_the_owning_registry_is_dropped() {
    // ~keep LOADED_LIBRARIES has exactly one mutation site — the guarded `insert` in
    // ~keep `language_from_process_library` — and no `remove`/`clear` anywhere in the
    // ~keep crate. That absence is the invariant that lets a `Language` (which borrows
    // ~keep code out of the loaded `libloading::Library`) outlive the `LanguageRegistry`
    // ~keep scope that produced it. This test cannot observe the absence of a `remove`
    // ~keep call directly, so it asserts the externally visible consequence instead:
    // ~keep dropping every registry that loaded a library must never shrink the map.
    let registry = LanguageRegistry::new();
    let Some(language_name) = registry.dynamic_loader.dynamic_names.first().copied() else {
        return;
    };
    let lib_path = lib_path_in(&registry.dynamic_loader.libs_dir, language_name);
    let Ok(_language) = registry.get_language(language_name) else {
        return;
    };
    assert!(
        dynamic::library_is_loaded_for_tests(&lib_path),
        "library should be loaded after get_language"
    );

    drop(registry);

    assert!(
        dynamic::library_is_loaded_for_tests(&lib_path),
        "LOADED_LIBRARIES must retain the entry after every owning registry is dropped \
         — nothing in this crate removes from it"
    );
}

#[test]
fn test_global_process_is_safe_from_multiple_threads() {
    let languages: Vec<String> = ["python", "rust", "javascript", "go"]
        .into_iter()
        .filter(|name| crate::has_language(name))
        .map(str::to_string)
        .collect();
    if languages.is_empty() {
        return;
    }

    std::thread::scope(|scope| {
        for language in &languages {
            scope.spawn(move || {
                for _ in 0..16 {
                    let config = ProcessConfig::new(language);
                    let result = crate::process("x", &config).expect("process should be thread-safe");
                    assert_eq!(result.language, *language);
                }
            });
        }
    });
}

#[cfg(feature = "serde")]
#[test]
fn test_process_result_serde_roundtrip() {
    let Some(lang) = first_available_lang() else { return };
    let registry = LanguageRegistry::new();
    let source = "x";
    let config = ProcessConfig::new(&lang);
    let intel = registry.process(source, &config).unwrap();
    let json = serde_json::to_string(&intel).expect("serialize should succeed");
    let deserialized: crate::intel::types::ProcessResult =
        serde_json::from_str(&json).expect("deserialize should succeed");
    assert_eq!(deserialized.language, intel.language);
    assert_eq!(deserialized.metrics.total_lines, intel.metrics.total_lines);
    assert_eq!(deserialized.metrics.total_bytes, intel.metrics.total_bytes);
}

#[cfg(any(feature = "dynamic-loading", feature = "download"))]
#[test]
fn should_resolve_every_alias_to_a_stable_canonical_name() {
    // ~keep The download manifest is keyed by canonical name only, so any caller
    // taking user input must resolve before consulting it. `download` shipped
    // without doing so and rejected `shell` as an unknown language.
    for &(alias, target) in LANGUAGE_ALIASES {
        assert_eq!(
            resolve_alias(alias),
            target,
            "alias '{alias}' must resolve to '{target}'"
        );
        assert_eq!(
            resolve_alias(target),
            target,
            "canonical name '{target}' must be a fixed point, or resolving twice would corrupt it"
        );
        assert!(
            !LANGUAGE_ALIASES.iter().any(|&(a, _)| a == target),
            "alias target '{target}' must not itself be an alias — resolution is single-pass"
        );
    }
}

#[cfg(any(feature = "dynamic-loading", feature = "download"))]
#[test]
fn should_leave_unknown_and_canonical_names_untouched() {
    assert_eq!(resolve_alias("rust"), "rust");
    assert_eq!(resolve_alias("definitely_not_a_language"), "definitely_not_a_language");
    assert_eq!(resolve_alias(""), "");
}

#[cfg(feature = "download")]
#[test]
fn should_report_every_alias_that_resolves_to_a_canonical_target() {
    assert_eq!(aliases_for("bash"), vec!["shell"]);
    assert_eq!(aliases_for("starlark"), vec!["bazel"]);
    assert_eq!(
        aliases_for("definitely_not_a_language"),
        Vec::<&str>::new(),
        "a target with no alias must report an empty list, not panic"
    );
}

#[cfg(feature = "download")]
#[test]
fn should_agree_with_resolve_alias_for_every_alias_in_the_table() {
    // ~keep aliases_for is the reverse of resolve_alias; every alias must round-trip.
    for &(alias, target) in LANGUAGE_ALIASES {
        assert!(
            aliases_for(target).contains(&alias),
            "aliases_for({target:?}) must contain {alias:?}"
        );
    }
}
