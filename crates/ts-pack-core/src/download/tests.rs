// ~keep Test assertions legitimately use unwrap/expect; production code stays
// ~keep covered by the crate-wide `unwrap_used`/`expect_used` deny in Cargo.toml.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use super::cache::*;
use super::*;

fn temp_cache_dir() -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix("tslp-cache-")
        .tempdir()
        .expect("temporary cache directory should be created")
}

fn manager_for_temp_dir(temp_dir: &tempfile::TempDir) -> DownloadManager {
    DownloadManager::with_cache_dir("test", temp_dir.path().join("libs"))
}

fn compressed_tar(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let encoder = zstd::Encoder::new(Vec::new(), 0).expect("zstd encoder should initialize");
    let mut builder = tar::Builder::new(encoder);

    for (path, contents) in entries {
        let mut header = tar::Header::new_gnu();
        header.set_size(contents.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        builder
            .append_data(&mut header, *path, *contents)
            .expect("tar entry should append");
    }

    let encoder = builder.into_inner().expect("tar builder should finish");
    encoder.finish().expect("zstd encoder should finish")
}

#[test]
fn cache_base_prefers_env_override_over_system_cache_dir() {
    let base = cache_base_from(Some("/custom/cache"), Some(PathBuf::from("/system/cache")))
        .expect("an explicit override must resolve");
    assert_eq!(base, PathBuf::from("/custom/cache"));
}

#[test]
fn cache_base_falls_through_when_env_override_is_empty() {
    for empty in ["", "   "] {
        let base = cache_base_from(Some(empty), Some(PathBuf::from("/system/cache")))
            .expect("a blank override must fall through, not fail");
        assert_eq!(base, PathBuf::from("/system/cache"), "{empty:?} must not win");
    }
}

#[test]
fn cache_base_uses_system_cache_dir_when_env_override_is_absent() {
    let base = cache_base_from(None, Some(PathBuf::from("/system/cache"))).expect("system cache dir must resolve");
    assert_eq!(base, PathBuf::from("/system/cache"));
}

#[test]
fn should_error_instead_of_falling_back_to_temp_dir_when_no_cache_dir_is_available() {
    // ~keep Regression for #101 H2: this used to silently degrade to
    // ~keep `std::env::temp_dir()`, a world-writable directory this crate then
    // ~keep dlopens shared libraries out of. It must fail closed instead.
    for (env_override, label) in [(None, "absent"), (Some(""), "blank")] {
        let error = cache_base_from(env_override, None)
            .expect_err(&format!("a {label} override with no system cache dir must fail closed"));
        assert!(
            matches!(error, Error::Download(_)),
            "unexpected error variant: {error:?}"
        );
        let message = error.to_string();
        assert!(
            message.contains(CACHE_DIR_ENV),
            "error must name the override env var so the user knows how to fix it: {message}"
        );
    }
}

#[test]
fn default_cache_dir_succeeds_and_is_rooted_at_the_resolved_base() {
    let dir = DownloadManager::default_cache_dir("9.9.9").expect("default cache dir must not fail");
    assert!(
        dir.ends_with("tree-sitter-language-pack/v9.9.9/libs"),
        "{}",
        dir.display()
    );
}

#[test]
fn cache_dir_from_base_appends_the_version_and_libs_suffix() {
    let base = PathBuf::from("/configured/base");
    let dir = DownloadManager::cache_dir_from_base(&base, "9.9.9");
    assert_eq!(dir, base.join("tree-sitter-language-pack/v9.9.9/libs"));
}

#[test]
fn should_root_version_cache_dir_under_the_configured_base_not_its_parent() {
    // ~keep Regression for #101 H1: a custom `cache_dir` used to be forwarded
    // ~keep verbatim, so `version_cache_dir()` (`self.cache_dir.parent()`)
    // ~keep resolved OUTSIDE the caller's configured directory entirely.
    let temp_dir = temp_cache_dir();
    let base = temp_dir.path().join("configured-base");
    let libs_dir = DownloadManager::cache_dir_from_base(&base, "9.9.9");
    let manager = DownloadManager::with_cache_dir("9.9.9", libs_dir.clone());

    let version_dir = manager
        .version_cache_dir()
        .expect("version cache dir should resolve")
        .to_path_buf();

    assert!(
        version_dir.starts_with(&base),
        "manifest/bundles/lock directory {} must live under the configured base {}",
        version_dir.display(),
        base.display()
    );
    assert_eq!(version_dir, base.join("tree-sitter-language-pack/v9.9.9"));
    assert_eq!(libs_dir, version_dir.join("libs"));
}

#[test]
fn should_reject_new_when_version_is_empty() {
    let error = validate_version("").expect_err("empty version must be rejected");
    assert!(matches!(error, Error::Download(_)));
}

#[test]
fn should_reject_new_when_version_contains_a_path_separator() {
    for version in ["1.0/../etc", "a/b", "a\\b"] {
        let error = validate_version(version).expect_err(&format!("{version:?} must be rejected"));
        assert!(matches!(error, Error::Download(_)), "{version:?}: {error:?}");
    }
}

#[test]
fn should_reject_new_when_version_contains_dot_dot() {
    let error = validate_version("..").expect_err("'..' must be rejected");
    assert!(matches!(error, Error::Download(_)));
}

#[test]
fn should_accept_new_when_version_is_a_well_formed_semver_ish_string() {
    for version in ["1.2.3", "1.2.3-rc.1", "1.2.3+build.7", "test", "concurrent-test"] {
        validate_version(version).unwrap_or_else(|e| panic!("{version:?} should be accepted, got {e:?}"));
    }
}

#[test]
fn bundle_cache_path_uses_version_cache_dir() {
    let temp_dir = temp_cache_dir();
    let cache_dir = temp_dir.path().join("libs");
    let manager = manager_for_temp_dir(&temp_dir);
    let sha256 = "a".repeat(64);

    let path = manager
        .bundle_cache_path("macos-arm64", &sha256)
        .expect("bundle cache path should resolve");

    assert_eq!(
        path,
        cache_dir
            .parent()
            .unwrap()
            .join(format!("bundles/macos-arm64-{sha256}.tar.zst"))
    );
}

#[test]
fn should_reject_bundle_cache_path_when_sha256_is_not_hex() {
    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);

    let error = manager
        .bundle_cache_path("macos-arm64", "not-a-valid-sha")
        .expect_err("non-hex sha256 must be rejected");

    assert!(
        matches!(error, Error::Download(_)),
        "unexpected error variant: {error:?}"
    );
}

#[test]
fn should_reject_bundle_cache_path_when_sha256_is_a_path_traversal_attempt() {
    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);

    let error = manager
        .bundle_cache_path("macos-arm64", "../../../home/user/.ssh/id_ed25519")
        .expect_err("path-traversal sha256 must be rejected");

    assert!(
        matches!(error, Error::Download(_)),
        "unexpected error variant: {error:?}"
    );
}

#[test]
fn should_reject_bundle_cache_path_when_sha256_is_shorter_than_64_chars() {
    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);
    let short_sha = "a".repeat(63);

    let error = manager
        .bundle_cache_path("macos-arm64", &short_sha)
        .expect_err("short sha256 must be rejected");

    assert!(
        matches!(error, Error::Download(_)),
        "unexpected error variant: {error:?}"
    );
}

#[test]
fn should_accept_bundle_cache_path_when_sha256_is_well_formed_hex() {
    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);
    let sha256 = "0123456789abcdef".repeat(4);

    let path = manager
        .bundle_cache_path("macos-arm64", &sha256)
        .expect("well-formed hex sha256 should be accepted");

    assert!(path.ends_with(format!("bundles/macos-arm64-{sha256}.tar.zst")));
}

#[test]
fn should_return_same_path_when_lib_path_is_given_an_alias() {
    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);

    assert_eq!(
        manager.lib_path("shell"),
        manager.lib_path("bash"),
        "alias and canonical name must resolve to the same cache path"
    );
}

#[test]
fn should_report_cached_when_is_cached_is_given_an_alias_for_a_cached_language() {
    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);
    let canonical_path = manager.lib_path("bash");
    fs::create_dir_all(canonical_path.parent().unwrap()).expect("cache dir should be created");
    fs::write(&canonical_path, b"stub-library-bytes").expect("stub library should be written");

    assert!(
        manager.is_cached("shell"),
        "alias lookup must see the file cached under the canonical name"
    );
    assert!(manager.is_cached("bash"), "canonical lookup must see the same file");
}

#[test]
fn should_report_no_languages_installed_when_cache_dir_is_absent() {
    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);
    assert_eq!(manager.installed_languages(), Vec::<String>::new());
}

#[test]
fn should_report_alias_alongside_canonical_name_when_cache_contains_the_canonical_library() {
    // ~keep Regression for #107: `installed_languages()` used to report canonical
    // ~keep names only, so it could never agree with `available_languages()`.
    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);
    let canonical_path = manager.lib_path("bash");
    fs::create_dir_all(canonical_path.parent().unwrap()).expect("cache dir should be created");
    fs::write(&canonical_path, b"stub-library-bytes").expect("stub library should be written");

    let installed = manager.installed_languages();

    assert_eq!(
        installed,
        vec!["bash".to_string(), "shell".to_string()],
        "must report both the canonical name and every alias that resolves to it"
    );
}

#[cfg(unix)]
/// Whether the current test process is running as root, which bypasses Unix
/// discretionary access control entirely. Permission-denial regression tests
/// below must skip (not fail) under root, since `read_dir`/`stat` cannot be
/// made to fail for root by revoking permission bits.
fn running_as_root_for_tests() -> bool {
    // SAFETY: getuid() takes no arguments, performs no I/O, and cannot fail.
    unsafe { getuid() == 0 }
}

#[cfg(unix)]
#[test]
fn should_report_no_languages_installed_when_cache_dir_cannot_be_read() {
    use std::os::unix::fs::PermissionsExt;
    if running_as_root_for_tests() {
        return;
    }

    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);
    let canonical_path = manager.lib_path("python");
    fs::create_dir_all(canonical_path.parent().unwrap()).expect("cache dir should be created");
    fs::write(&canonical_path, b"stub").expect("stub library should be written");
    fs::set_permissions(manager.cache_dir(), fs::Permissions::from_mode(0o300))
        .expect("removing read permission should succeed");

    let installed = manager.installed_languages();

    fs::set_permissions(manager.cache_dir(), fs::Permissions::from_mode(0o700))
        .expect("restoring permissions should succeed");

    assert_eq!(
        installed,
        Vec::<String>::new(),
        "an unreadable cache dir must degrade to an empty result, not panic"
    );
}

#[cfg(unix)]
#[test]
fn should_return_an_error_when_the_final_library_count_scan_cannot_read_the_cache_dir() {
    use std::os::unix::fs::PermissionsExt;
    if running_as_root_for_tests() {
        return;
    }

    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);
    fs::create_dir_all(manager.cache_dir()).expect("cache dir should be created");
    // ~keep Empty archive: the extraction phase is a no-op, so a failure can only
    // ~keep come from the trailing recount scan this regression targets.
    let archive = compressed_tar(&[]);
    fs::set_permissions(manager.cache_dir(), fs::Permissions::from_mode(0o300))
        .expect("removing read permission should succeed");

    let result = manager.extract_all_libs(&archive);

    fs::set_permissions(manager.cache_dir(), fs::Permissions::from_mode(0o700))
        .expect("restoring permissions should succeed");

    let error = result
        .expect_err("a successful (no-op) extraction phase must not silently report 0 when the recount itself fails");
    assert!(
        matches!(error, Error::Download(_)),
        "unexpected error variant: {error:?}"
    );
}

#[cfg(unix)]
#[test]
fn should_create_new_cache_dir_with_owner_only_permissions() {
    use std::os::unix::fs::PermissionsExt;
    let temp_dir = temp_cache_dir();
    let target = temp_dir.path().join("secure-cache");

    ensure_secure_cache_dir(&target).expect("secure creation should succeed");

    let mode = fs::metadata(&target)
        .expect("metadata should read")
        .permissions()
        .mode();
    assert_eq!(
        mode & 0o777,
        0o700,
        "cache dir must be created owner-only, got {mode:o}"
    );
}

#[cfg(unix)]
#[test]
fn should_reject_existing_cache_dir_that_is_group_or_other_writable() {
    use std::os::unix::fs::PermissionsExt;
    if running_as_root_for_tests() {
        return;
    }

    let temp_dir = temp_cache_dir();
    let target = temp_dir.path().join("loose-cache");
    fs::create_dir_all(&target).expect("dir should be created");
    fs::set_permissions(&target, fs::Permissions::from_mode(0o770)).expect("chmod should succeed");

    let error = ensure_secure_cache_dir(&target).expect_err("group-writable cache dir must be refused");
    assert!(matches!(error, Error::Download(_)));
    assert!(error.to_string().contains("writable"), "{error}");
}

#[cfg(unix)]
#[test]
fn should_accept_existing_cache_dir_that_is_already_owner_only() {
    use std::os::unix::fs::PermissionsExt;
    let temp_dir = temp_cache_dir();
    let target = temp_dir.path().join("already-secure");
    fs::create_dir_all(&target).expect("dir should be created");
    fs::set_permissions(&target, fs::Permissions::from_mode(0o700)).expect("chmod should succeed");

    ensure_secure_cache_dir(&target).expect("owner-only pre-existing dir should be accepted");
}

#[cfg(unix)]
#[test]
fn should_trust_self_and_root_owners_but_reject_every_other_uid() {
    // ~keep Regression for #202: a build-time, root-owned, non-writable cache (read-only
    // ~keep container image) must be accepted, while a directory owned by some *other*
    // ~keep local user must still be refused. The uid/mode read cannot be forged in a
    // ~keep test for the root case, so the decision is a pure function here.
    assert!(is_trusted_cache_owner(1000, 1000), "own uid must be trusted");
    assert!(is_trusted_cache_owner(0, 1000), "root-owned must be trusted");
    assert!(
        !is_trusted_cache_owner(1001, 1000),
        "another user's uid must be refused"
    );
    assert!(
        !is_trusted_cache_owner(1000, 0),
        "non-root owner must be refused when running as root"
    );
}

#[cfg(unix)]
#[test]
fn verify_cache_dir_if_present_is_a_noop_when_the_directory_is_absent() {
    let temp_dir = temp_cache_dir();
    let missing = temp_dir.path().join("does-not-exist");
    verify_cache_dir_if_present(&missing).expect("a missing directory has nothing to verify");
}

// ~keep Windows-gated: exercises `ensure_secure_cache_dir`/`verify_cache_dir_if_present`'s
// ~keep `#[cfg(windows)]` bodies and the pure `is_outside_profile_cache_root` helper they
// ~keep call. Cannot compile or run on this (or any non-Windows) host — see #109 report.
#[cfg(windows)]
#[test]
fn is_outside_profile_cache_root_flags_a_path_not_nested_under_the_profile_root() {
    assert!(is_outside_profile_cache_root(
        Path::new(r"D:\Shared\tree-sitter-language-pack"),
        r"C:\Users\alice\AppData\Local"
    ));
}

#[cfg(windows)]
#[test]
fn is_outside_profile_cache_root_accepts_a_path_nested_under_the_profile_root() {
    assert!(!is_outside_profile_cache_root(
        Path::new(r"C:\Users\alice\AppData\Local\tree-sitter-language-pack\v9.9.9\libs"),
        r"C:\Users\alice\AppData\Local"
    ));
}

#[cfg(windows)]
#[test]
fn should_create_new_cache_dir_under_windows() {
    let temp_dir = temp_cache_dir();
    let target = temp_dir.path().join("secure-cache");

    ensure_secure_cache_dir(&target).expect("directory creation should succeed");

    assert!(target.is_dir(), "cache dir must exist after creation");
}

#[cfg(windows)]
#[test]
fn ensure_secure_cache_dir_accepts_an_already_existing_directory_under_windows() {
    let temp_dir = temp_cache_dir();
    let target = temp_dir.path().join("already-there");
    fs::create_dir_all(&target).expect("dir should be created");

    ensure_secure_cache_dir(&target).expect("a pre-existing directory should be accepted, never refused");
}

#[cfg(windows)]
#[test]
fn verify_cache_dir_if_present_is_a_noop_when_the_directory_is_absent_under_windows() {
    let temp_dir = temp_cache_dir();
    let missing = temp_dir.path().join("does-not-exist");
    verify_cache_dir_if_present(&missing).expect("a missing directory has nothing to verify");
}

#[cfg(windows)]
#[test]
fn verify_cache_dir_if_present_never_refuses_an_existing_directory_under_windows() {
    // ~keep Regression guard: the Windows path is advisory-only by design (see
    // ~keep `ensure_secure_cache_dir`'s doc comment) — it must never return `Err`,
    // ~keep unlike the Unix overload it mirrors.
    let temp_dir = temp_cache_dir();
    let target = temp_dir.path().join("present");
    fs::create_dir_all(&target).expect("dir should be created");

    verify_cache_dir_if_present(&target).expect("Windows verification must never refuse");
}

#[test]
fn bundle_read_cap_uses_the_declared_size_when_under_the_ceiling() {
    assert_eq!(bundle_read_cap(1024), 1024);
}

#[test]
fn bundle_read_cap_clamps_to_the_absolute_ceiling_when_declared_size_is_larger() {
    assert_eq!(bundle_read_cap(u64::MAX), MAX_BUNDLE_DOWNLOAD_BYTES);
}

#[test]
fn verified_bundle_cache_returns_matching_archive_bytes() {
    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);
    let data = b"verified archive bytes";
    let sha256 = DownloadManager::sha256_hex(data);
    let cache_path = manager
        .bundle_cache_path("macos-arm64", &sha256)
        .expect("bundle cache path should resolve");
    fs::create_dir_all(cache_path.parent().unwrap()).expect("bundle cache directory should be created");
    fs::write(&cache_path, data).expect("bundle cache file should be written");

    let cached =
        DownloadManager::load_verified_cached_bundle(&cache_path, &sha256).expect("verified cache read should succeed");

    assert_eq!(cached, Some(data.to_vec()));
    assert!(cache_path.exists());
}

#[test]
fn verified_bundle_cache_removes_hash_mismatch() {
    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);
    let expected_sha256 = "0".repeat(64);
    let cache_path = manager
        .bundle_cache_path("macos-arm64", &expected_sha256)
        .expect("bundle cache path should resolve");
    fs::create_dir_all(cache_path.parent().unwrap()).expect("bundle cache directory should be created");
    fs::write(&cache_path, b"corrupt archive bytes").expect("bundle cache file should be written");

    let cached = DownloadManager::load_verified_cached_bundle(&cache_path, &expected_sha256)
        .expect("corrupt cache should be removed");

    assert_eq!(cached, None);
    assert!(!cache_path.exists());
}

#[test]
fn extract_languages_writes_requested_library() {
    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);
    let filename = manager
        .lib_path("python")
        .file_name()
        .expect("library path should have filename")
        .to_string_lossy()
        .into_owned();
    let archive = compressed_tar(&[(&filename, b"library-bytes")]);

    manager
        .extract_languages(&archive, &["python"])
        .expect("requested library should extract");

    let extracted = fs::read(manager.lib_path("python")).expect("extracted library should be readable");
    assert_eq!(extracted, b"library-bytes");
}

#[test]
fn extract_languages_errors_when_requested_library_is_absent() {
    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);
    let archive = compressed_tar(&[("libtree_sitter_javascript.dylib", b"library-bytes")]);

    let error = manager
        .extract_languages(&archive, &["python"])
        .expect_err("missing requested library should error");

    assert!(error.to_string().contains("python"));
}

#[test]
fn clean_cache_removes_libraries_bundles_and_manifest() {
    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);
    let version_cache_dir = manager
        .version_cache_dir()
        .expect("cache directory should have a parent")
        .to_path_buf();
    let library_path = manager.lib_path("python");
    let bundle_path = version_cache_dir.join("bundles/macos-arm64-abc123.tar.zst");
    let manifest_path = version_cache_dir.join("manifest.json");
    let unrelated_path = version_cache_dir.join("unrelated.txt");

    fs::create_dir_all(library_path.parent().unwrap()).expect("library cache directory should be created");
    fs::create_dir_all(bundle_path.parent().unwrap()).expect("bundle cache directory should be created");
    fs::write(&library_path, b"library").expect("library cache file should be written");
    fs::write(&bundle_path, b"bundle").expect("bundle cache file should be written");
    fs::write(&manifest_path, b"{}").expect("manifest cache file should be written");
    fs::write(&unrelated_path, b"keep").expect("unrelated cache file should be written");

    manager.clean_cache().expect("cache cleanup should succeed");

    assert!(!manager.cache_dir().exists());
    assert!(!version_cache_dir.join("bundles").exists());
    assert!(!manifest_path.exists());
    assert!(
        unrelated_path.exists(),
        "cleanup should not remove unrelated sibling files"
    );
}

#[test]
fn clean_cache_is_idempotent_and_safe_for_concurrent_callers() {
    let temp_dir = temp_cache_dir();
    let manager = Arc::new(manager_for_temp_dir(&temp_dir));
    let library_path = manager.lib_path("python");
    fs::create_dir_all(library_path.parent().unwrap()).expect("library cache directory should be created");
    fs::write(&library_path, b"library").expect("library cache file should be written");

    std::thread::scope(|scope| {
        for _ in 0..8 {
            let manager = Arc::clone(&manager);
            scope.spawn(move || manager.clean_cache().expect("concurrent cleanup should succeed"));
        }
    });

    assert!(!manager.cache_dir().exists());
}

// ~keep Env-var-mutating tests share one mutex so default parallel `cargo test` cannot race TLS env state.
static TLS_ENV_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());

struct EnvVarGuard {
    key: &'static str,
    previous: Option<String>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let previous = std::env::var(key).ok();
        // ~keep SAFETY: TLS_ENV_GUARD serializes tests that observe or mutate this env var.
        unsafe { std::env::set_var(key, value) };
        Self { key, previous }
    }

    fn unset(key: &'static str) -> Self {
        let previous = std::env::var(key).ok();
        // ~keep SAFETY: see set() above.
        unsafe { std::env::remove_var(key) };
        Self { key, previous }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        // ~keep SAFETY: see set() above.
        unsafe {
            match &self.previous {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }
}

#[test]
fn resolve_tls_roots_returns_explicit_override_when_provided() {
    let _guard = TLS_ENV_GUARD.lock().expect("env guard should not be poisoned");
    let _env = EnvVarGuard::set(TLS_ROOTS_ENV, "webpki");
    assert_eq!(resolve_tls_roots(Some(TlsRootsMode::Platform)), TlsRootsMode::Platform);
    assert_eq!(resolve_tls_roots(Some(TlsRootsMode::WebPki)), TlsRootsMode::WebPki);
}

#[test]
fn resolve_tls_roots_reads_env_var_platform() {
    let _guard = TLS_ENV_GUARD.lock().expect("env guard should not be poisoned");
    let _env = EnvVarGuard::set(TLS_ROOTS_ENV, "platform");
    assert_eq!(resolve_tls_roots(None), TlsRootsMode::Platform);
}

#[test]
fn resolve_tls_roots_reads_env_var_webpki() {
    let _guard = TLS_ENV_GUARD.lock().expect("env guard should not be poisoned");
    let _env = EnvVarGuard::set(TLS_ROOTS_ENV, "webpki");
    assert_eq!(resolve_tls_roots(None), TlsRootsMode::WebPki);
}

#[test]
fn resolve_tls_roots_is_case_insensitive_and_trims_whitespace() {
    let _guard = TLS_ENV_GUARD.lock().expect("env guard should not be poisoned");
    let _env = EnvVarGuard::set(TLS_ROOTS_ENV, "  WebPKI  ");
    assert_eq!(resolve_tls_roots(None), TlsRootsMode::WebPki);
}

#[test]
fn resolve_tls_roots_falls_back_to_default_when_env_unset() {
    let _guard = TLS_ENV_GUARD.lock().expect("env guard should not be poisoned");
    let _env = EnvVarGuard::unset(TLS_ROOTS_ENV);
    assert_eq!(resolve_tls_roots(None), TlsRootsMode::default());
    // ~keep Default must remain Platform: that is the user-facing fix for #125.
    assert_eq!(TlsRootsMode::default(), TlsRootsMode::Platform);
}

#[test]
fn resolve_tls_roots_falls_back_to_default_when_env_is_garbage() {
    let _guard = TLS_ENV_GUARD.lock().expect("env guard should not be poisoned");
    // ~keep Unknown TLS root values fall back to default instead of panicking during download.
    let _env = EnvVarGuard::set(TLS_ROOTS_ENV, "not-a-mode");
    assert_eq!(resolve_tls_roots(None), TlsRootsMode::default());
}

#[test]
fn build_agent_platform_mode_constructs_an_agent() {
    // ~keep Platform verifier errors only when the request reaches the network, not when constructing the agent.
    let _agent = build_agent(TlsRootsMode::Platform);
}

#[test]
fn build_agent_webpki_mode_constructs_an_agent() {
    let _agent = build_agent(TlsRootsMode::WebPki);
}

// ~keep Manifest URL env tests use the same mutex pattern to avoid concurrent env mutation flakes.
static MANIFEST_URL_ENV_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn resolve_manifest_url_defaults_to_github_release() {
    let _guard = MANIFEST_URL_ENV_GUARD.lock().expect("env guard should not be poisoned");
    let _env = EnvVarGuard::unset(MANIFEST_URL_ENV);
    assert_eq!(
        resolve_manifest_url("1.2.3"),
        "https://github.com/xberg-io/tree-sitter-language-pack/releases/download/v1.2.3/parsers.json"
    );
}

#[test]
fn resolve_manifest_url_honours_env_override_http() {
    let _guard = MANIFEST_URL_ENV_GUARD.lock().expect("env guard should not be poisoned");
    let _env = EnvVarGuard::set(MANIFEST_URL_ENV, "https://mirror.example.com/parsers.json");
    assert_eq!(resolve_manifest_url("1.2.3"), "https://mirror.example.com/parsers.json");
}

#[test]
fn resolve_manifest_url_honours_env_override_file_url() {
    let _guard = MANIFEST_URL_ENV_GUARD.lock().expect("env guard should not be poisoned");
    let _env = EnvVarGuard::set(MANIFEST_URL_ENV, "file:///tmp/local-parsers.json");
    assert_eq!(resolve_manifest_url("1.2.3"), "file:///tmp/local-parsers.json");
}

#[test]
fn resolve_manifest_url_falls_back_when_env_is_empty_or_whitespace() {
    let _guard = MANIFEST_URL_ENV_GUARD.lock().expect("env guard should not be poisoned");
    // ~keep Empty/whitespace manifest URL means unset, not an intentional empty fetch URL.
    let _env = EnvVarGuard::set(MANIFEST_URL_ENV, "   ");
    assert!(resolve_manifest_url("1.2.3").starts_with(GITHUB_RELEASE_BASE));
}

#[test]
fn read_file_url_returns_body_for_existing_file() {
    let _guard = MANIFEST_URL_ENV_GUARD.lock().expect("env guard should not be poisoned");
    let temp_dir = temp_cache_dir();
    let path = temp_dir.path().join("parsers.json");
    let body = r#"{"version":"9.9.9","platforms":{},"languages":{},"groups":{}}"#;
    fs::write(&path, body).expect("seed file should be written");
    let url = format!("file://{}", path.display());

    let result = read_file_url(&url).expect("file:// read should succeed");
    assert_eq!(result, body);
}

#[test]
fn read_file_url_errors_on_missing_file() {
    let _guard = MANIFEST_URL_ENV_GUARD.lock().expect("env guard should not be poisoned");
    let temp_dir = temp_cache_dir();
    let url = format!("file://{}", temp_dir.path().join("nope.json").display());
    let err = read_file_url(&url).expect_err("missing file should error");
    assert!(matches!(err, Error::Download(_)));
}

#[test]
fn read_file_url_errors_on_non_file_scheme() {
    let _guard = MANIFEST_URL_ENV_GUARD.lock().expect("env guard should not be poisoned");
    let err = read_file_url("https://example.com/parsers.json").expect_err("non-file URL should error");
    assert!(matches!(err, Error::Download(_)));
}

#[test]
fn fetch_manifest_reads_from_file_url_when_env_is_set() {
    let _guard = MANIFEST_URL_ENV_GUARD.lock().expect("env guard should not be poisoned");
    let temp_dir = temp_cache_dir();
    let manifest_src = temp_dir.path().join("local-parsers.json");
    let body = r#"{"version":"local-test","platforms":{},"languages":{},"groups":{}}"#;
    fs::write(&manifest_src, body).expect("seed manifest should be written");
    let _env = EnvVarGuard::set(MANIFEST_URL_ENV, &format!("file://{}", manifest_src.display()));

    let manager = DownloadManager::with_cache_dir("local-test", temp_dir.path().join("libs"));
    let manifest = manager
        .fetch_manifest_inner_locked()
        .expect("file:// manifest fetch should succeed");
    assert_eq!(manifest.version, "local-test");
}

#[test]
fn download_manager_constructor_honours_explicit_tls_override() {
    let temp_dir = temp_cache_dir();
    // ~keep Closed-port calls prove each TLS mode builds an agent instead of panicking on TLS config.
    for mode in [TlsRootsMode::Platform, TlsRootsMode::WebPki] {
        let dm = DownloadManager::with_cache_dir_and_tls("test", temp_dir.path().join("libs"), Some(mode));
        let result = dm.agent.get("http://127.0.0.1:1/never").call();
        assert!(
            result.is_err(),
            "agent should fail to connect to a closed port in mode {mode:?}"
        );
    }
}

/// Every read of an atomically-written file must see either the old content,
/// the new content, or "not found" — never partial bytes.
///
/// Uses 256 KB payloads (one rough page boundary) to make non-atomic writes
/// visibly tear under concurrent reads. Writers loop 50 times to widen the
/// race window.
#[test]
fn atomic_write_visible_or_not_at_all() {
    use std::sync::Barrier;

    const PAYLOAD_SIZE: usize = 256 * 1024;
    let old_payload: Arc<Vec<u8>> = Arc::new(vec![0xAA_u8; PAYLOAD_SIZE]);
    let new_payload: Arc<Vec<u8>> = Arc::new(vec![0x55_u8; PAYLOAD_SIZE]);

    let temp_dir = temp_cache_dir();
    let path = temp_dir.path().join("libs").join("target.bin");
    fs::create_dir_all(path.parent().unwrap()).unwrap();

    atomic_write(&path, &old_payload).expect("seed write should succeed");

    let path = Arc::new(path);
    let barrier = Arc::new(Barrier::new(8));

    std::thread::scope(|scope| {
        for i in 0..8_usize {
            let path = Arc::clone(&path);
            let barrier = Arc::clone(&barrier);
            let old_payload = Arc::clone(&old_payload);
            let new_payload = Arc::clone(&new_payload);
            scope.spawn(move || {
                barrier.wait();
                if i % 2 == 0 {
                    for _ in 0..50 {
                        atomic_write(&path, &new_payload).expect("atomic write should succeed");
                    }
                } else {
                    // ~keep Readers may see old/new/not-found, but never mixed-byte partial writes.
                    for _ in 0..50 {
                        match fs::read(path.as_ref()) {
                            Ok(data) => {
                                // ~keep Any mixed-byte pattern is a torn write.
                                if !data.is_empty() {
                                    let first = data[0];
                                    let last = *data.last().unwrap();
                                    let all_same = data.iter().all(|&b| b == first);
                                    assert!(
                                        all_same && first == last,
                                        "reader observed a torn (mixed-byte) write: \
                                             first=0x{first:02X} last=0x{last:02X} len={}",
                                        data.len()
                                    );
                                    assert!(
                                        data == *old_payload || data == *new_payload,
                                        "reader observed unexpected content: \
                                             first=0x{first:02X} len={}",
                                        data.len()
                                    );
                                }
                            }
                            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                                // ~keep NotFound is acceptable when rename races with read between exists() and open().
                            }
                            Err(e) => panic!("unexpected read error: {e}"),
                        }
                    }
                }
            });
        }
    });
}

/// Orphaned `.tmp.*` files in the libs directory must not appear as parsed
/// languages in `installed_languages()` and must not block a real extraction.
#[test]
fn orphan_tmp_files_ignored() {
    let temp_dir = temp_cache_dir();
    let manager = manager_for_temp_dir(&temp_dir);
    let libs_dir = manager.cache_dir();
    fs::create_dir_all(libs_dir).expect("libs dir should be created");
    // ~keep Pin the mode explicitly rather than relying on the ambient umask: the
    // ~keep later `extract_languages` call runs `ensure_secure_cache_dir`, which
    // ~keep refuses a group/other-writable directory (#101 H2) — a umask of 0 would
    // ~keep otherwise make this test environment-dependent.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(libs_dir, fs::Permissions::from_mode(0o700)).expect("chmod should succeed");
    }

    // ~keep Dot-prefixed orphan temp files must not pass `lang_from_lib_filename`.
    let orphan = libs_dir.join(".libtree_sitter_python.dylib.tmp.99999.0");
    fs::write(&orphan, b"corrupt-partial").expect("orphan write should succeed");

    let installed = manager.installed_languages();
    assert!(
        !installed.contains(&"python".to_string()),
        "orphan tmp file must not register as a language; got: {installed:?}"
    );

    let filename = manager
        .lib_path("python")
        .file_name()
        .expect("lib_path has filename")
        .to_string_lossy()
        .into_owned();
    let archive = compressed_tar(&[(&filename, b"real-library-bytes")]);

    manager
        .extract_languages(&archive, &["python"])
        .expect("extraction over existing orphan should succeed");

    let extracted = fs::read(manager.lib_path("python")).expect("extracted library should be readable");
    assert_eq!(
        extracted, b"real-library-bytes",
        "canonical library should contain real bytes"
    );

    // ~keep Orphan temp files are harmless and must differ from canonical library paths.
    assert_ne!(
        manager.lib_path("python"),
        orphan,
        "canonical lib path must not match orphan tmp path"
    );
}

/// Eight threads racing `extract_languages` against the same cache dir must
/// all return `Ok` and the final file content must exactly match the archive.
#[test]
fn concurrent_threads_share_cache() {
    let temp_dir = temp_cache_dir();
    let manager = Arc::new(manager_for_temp_dir(&temp_dir));

    let filename = manager
        .lib_path("python")
        .file_name()
        .expect("lib_path has filename")
        .to_string_lossy()
        .into_owned();
    let expected: &[u8] = b"concurrent-safe-library-bytes";
    let archive = Arc::new(compressed_tar(&[(&filename, expected)]));

    std::thread::scope(|scope| {
        for _ in 0..8 {
            let manager = Arc::clone(&manager);
            let archive = Arc::clone(&archive);
            scope.spawn(move || {
                manager
                    .extract_languages(&archive, &["python"])
                    .expect("concurrent extraction should succeed");
            });
        }
    });

    let final_content = fs::read(manager.lib_path("python")).expect("extracted library should be readable");
    assert_eq!(
        final_content,
        expected,
        "final extracted content must exactly match archive; got {} bytes",
        final_content.len()
    );
}
