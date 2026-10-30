use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::Error;
use crate::pack_config::TlsRootsMode;

mod cache;
mod transport;

pub(crate) use cache::{DownloadCacheLock, ensure_secure_cache_dir, verify_cache_dir_if_present};
use cache::{atomic_copy_from_reader, atomic_write, resolve_cache_base};
use transport::{build_agent, read_file_url, resolve_manifest_url, resolve_tls_roots};

const GITHUB_RELEASE_BASE: &str = "https://github.com/xberg-io/tree-sitter-language-pack/releases/download";
const CACHE_REMOVE_RETRIES: usize = 5;
const CACHE_REMOVE_RETRY_DELAY: Duration = Duration::from_millis(10);
const HTTP_TIMEOUT: Duration = Duration::from_secs(60);
const TLS_ROOTS_ENV: &str = "TREE_SITTER_LANGUAGE_PACK_TLS_ROOTS";
const MANIFEST_URL_ENV: &str = "TREE_SITTER_LANGUAGE_PACK_MANIFEST_URL";
const CACHE_DIR_ENV: &str = "TREE_SITTER_LANGUAGE_PACK_CACHE_DIR";
const LOCK_FILE_NAME: &str = ".download.lock";
/// Absolute ceiling on a downloaded platform-bundle archive, applied on top of
/// the manifest-declared [`PlatformBundle::size`] so a compromised or malformed
/// manifest cannot itself request an unbounded read. 2 GiB comfortably covers
/// every compiled-grammar bundle this crate has published; a legitimate bundle
/// that grows past it fails cleanly instead of exhausting memory. See #101 L1. ~keep
const MAX_BUNDLE_DOWNLOAD_BYTES: u64 = 2 * 1024 * 1024 * 1024;

/// Cap a bundle download at the manifest-declared size, itself bounded by
/// [`MAX_BUNDLE_DOWNLOAD_BYTES`].
fn bundle_read_cap(expected_size: u64) -> u64 {
    expected_size.min(MAX_BUNDLE_DOWNLOAD_BYTES)
}

/// Number of hex characters in a SHA-256 digest (32 bytes x 2 hex chars/byte). ~keep
const SHA256_HEX_LEN: usize = 64;

/// Reject a manifest-supplied `sha256` value that is not a well-formed 64-character
/// hex digest before it is used to build a cache path.
///
/// The manifest is untrusted input (fetched over HTTP, or read from a `file://`
/// URL a caller controls), so a hostile or corrupt `sha256` — e.g. containing
/// `../` — must never reach a path join: `load_verified_cached_bundle` both reads
/// and, on a hash mismatch, unlinks whatever path it resolves to. Mirrors the
/// equivalent guard in `build.rs` for the parser-source tarball checksum. ~keep
fn validate_sha256_hex(sha256: &str) -> Result<(), Error> {
    if sha256.len() == SHA256_HEX_LEN && sha256.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Ok(());
    }
    Err(Error::Download(format!(
        "Manifest sha256 '{sha256}' is not a well-formed {SHA256_HEX_LEN}-character hex digest"
    )))
}

/// Characters permitted in a crate version string: ASCII alphanumerics plus
/// `.`, `-`, `+` (a semver-ish charset).
fn is_valid_version_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '+')
}

/// Reject a version string that is empty, contains a path-traversal sequence, or
/// contains a character outside the semver-ish allowlist.
///
/// `version` is interpolated into both a manifest/bundle URL
/// ([`resolve_manifest_url`]) and a filesystem path ([`DownloadManager::default_cache_dir`],
/// and — via `self.version_cache_dir()` — the `remove_dir_all` target of
/// [`DownloadManager::clean_cache`]). [`DownloadManager::new`] is reachable directly
/// from every language binding's constructor with a caller-supplied version string,
/// so this must fail closed rather than trust the input. See #101 M2. ~keep
fn validate_version(version: &str) -> Result<(), Error> {
    if version.is_empty() || version.contains("..") || !version.chars().all(is_valid_version_char) {
        return Err(Error::Download(format!(
            "Invalid version '{version}': must be a non-empty string of ASCII alphanumerics, \
             '.', '-', or '+' only, and must not contain '..'"
        )));
    }
    Ok(())
}

/// Manifest describing available parser downloads for a specific version.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserManifest {
    /// Crate version this manifest was published for.
    pub version: String,
    /// Per-platform download bundle metadata, keyed by target triple.
    pub platforms: HashMap<String, PlatformBundle>,
    /// Per-language metadata, keyed by language name.
    pub languages: HashMap<String, LanguageInfo>,
    /// Named language groups, each mapping to a list of language names. The published
    /// manifest currently defines exactly one group, `"all"`.
    pub groups: HashMap<String, Vec<String>>,
}

/// Download metadata for a single platform's parser bundle.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformBundle {
    /// URL of the bundle archive.
    pub url: String,
    /// Expected SHA-256 hex digest of the bundle archive.
    pub sha256: String,
    /// Size of the bundle archive in bytes.
    pub size: u64,
}

/// Metadata for a single language's parser entry in the manifest.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageInfo {
    /// Name of the language group this parser belongs to.
    pub group: String,
    /// Size of the parser shared library in bytes.
    pub size: u64,
}

/// Manages downloading and caching of pre-built parser shared libraries.
pub struct DownloadManager {
    version: String,
    cache_dir: PathBuf,
    manifest: Mutex<Option<ParserManifest>>,
    agent: ureq::Agent,
}

impl DownloadManager {
    /// Create a new download manager for the given version.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Download`] if `version` is empty, contains a path
    /// separator or `..`, or contains a character outside `[A-Za-z0-9.+-]` — see
    /// [`validate_version`] — or if no cache directory can be resolved (see
    /// [`Self::default_cache_dir`]).
    pub fn new(version: &str) -> Result<Self, Error> {
        validate_version(version)?;
        let cache_dir = Self::default_cache_dir(version)?;
        Ok(Self::with_cache_dir_and_tls(version, cache_dir, None))
    }

    /// Create a download manager with a custom cache directory.
    #[cfg_attr(alef, alef(skip))]
    pub fn with_cache_dir(version: &str, cache_dir: PathBuf) -> Self {
        Self::with_cache_dir_and_tls(version, cache_dir, None)
    }

    /// Create a download manager with a custom cache directory and explicit TLS roots mode.
    ///
    /// Passing `None` for `tls_roots` falls back to the
    /// `TREE_SITTER_LANGUAGE_PACK_TLS_ROOTS` environment variable, then the
    /// compile-time default ([`TlsRootsMode::Platform`]).
    ///
    /// Rust-only. Bindings should rely on `TREE_SITTER_LANGUAGE_PACK_TLS_ROOTS`
    /// to override the default mode, since `TlsRootsMode` is intentionally not
    /// exported across the binding boundary (see `pack_config.rs`).
    #[cfg_attr(alef, alef(skip))]
    pub fn with_cache_dir_and_tls(version: &str, cache_dir: PathBuf, tls_roots: Option<TlsRootsMode>) -> Self {
        let mode = resolve_tls_roots(tls_roots);
        Self {
            version: version.to_string(),
            cache_dir,
            manifest: Mutex::new(None),
            agent: build_agent(mode),
        }
    }

    /// Default cache directory: `~/.cache/tree-sitter-language-pack/v{version}/libs/`
    ///
    /// The base is resolved in layers: the `TREE_SITTER_LANGUAGE_PACK_CACHE_DIR`
    /// environment variable (if set and non-empty), else the platform cache
    /// directory.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Download`] if `version` fails [`validate_version`], or if
    /// no cache directory can be resolved — the platform reported none and
    /// `TREE_SITTER_LANGUAGE_PACK_CACHE_DIR` is unset (see [`resolve_cache_base`]).
    /// This crate no longer falls back to the temporary directory for this: see
    /// #101 H2.
    #[cfg_attr(alef, alef(skip))]
    pub fn default_cache_dir(version: &str) -> Result<PathBuf, Error> {
        validate_version(version)?;
        let base = resolve_cache_base()?;
        Ok(Self::cache_dir_from_base(&base, version))
    }

    /// Append the crate-owned `tree-sitter-language-pack/v{version}/libs` suffix to `base`.
    ///
    /// Shared by [`Self::default_cache_dir`] (`base` = the resolved platform/env
    /// cache directory) and by `effective_cache_dir` in `lib.rs` (`base` = a
    /// caller-supplied [`crate::pack_config::PackConfig::cache_dir`]), so a custom
    /// cache directory is always treated as a BASE, never as the final libs path.
    ///
    /// Before this helper existed, a custom `cache_dir` was forwarded to
    /// [`Self::with_cache_dir`] verbatim, so [`Self::version_cache_dir`]
    /// (`self.cache_dir.parent()`) resolved to the *parent* of the caller's
    /// configured directory — `manifest.json`, `bundles/`, and `.download.lock`
    /// all ended up outside it, and `clean_cache` removed sibling directories the
    /// caller never configured. See #101 H1. ~keep
    pub(crate) fn cache_dir_from_base(base: &Path, version: &str) -> PathBuf {
        base.join("tree-sitter-language-pack")
            .join(format!("v{version}"))
            .join("libs")
    }

    /// Return the path to the libs cache directory.
    #[cfg_attr(alef, alef(skip))]
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// List languages that are already downloaded and cached.
    ///
    /// Derived from the on-disk cache filenames, one canonical name per file, plus
    /// every alias that resolves to it (via [`crate::registry::aliases_for`]) — so
    /// this list agrees with the user-facing
    /// [`LanguageRegistry::available_languages`](crate::LanguageRegistry::available_languages)
    /// about which names are "available"; both report `"shell"` once `bash` is
    /// cached, for example. A previous version reported canonical names only,
    /// which could never agree with `available_languages()`. See #107.
    ///
    /// Returns an empty list if the cache directory does not exist. If it exists
    /// but cannot be read (e.g. a permission error), also returns an empty list —
    /// changing this to a `Result` would be a breaking change across every
    /// language binding — but logs a `tracing::warn!` so the failure is not
    /// silently indistinguishable from "nothing installed". ~keep
    pub fn installed_languages(&self) -> Vec<String> {
        let mut langs = Vec::new();
        match fs::read_dir(&self.cache_dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    if let Some(lang) = crate::registry::lang_name_from_lib_filename(&name) {
                        for alias in crate::registry::aliases_for(&lang) {
                            langs.push(alias.to_string());
                        }
                        langs.push(lang);
                    }
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                tracing::warn!(
                    cache_dir = %self.cache_dir.display(),
                    %error,
                    "failed to list the download cache directory; reporting no languages installed"
                );
            }
        }
        langs.sort();
        langs.dedup();
        langs
    }

    /// Ensure the specified languages are available in the cache.
    /// Downloads the platform bundle if any requested languages are missing.
    ///
    /// Accepts language names or aliases (e.g. `"shell"` resolves to `"bash"`)
    /// interchangeably — every name is resolved to its canonical form before the
    /// cache and manifest are consulted, since the manifest is keyed by
    /// canonical name only.
    ///
    /// Cross-process safety: acquires the `.download.lock` file lock for the
    /// mutation window only. Readers are never blocked — the fast path returns
    /// immediately if all languages are already cached.
    ///
    /// **NFS limitation**: `flock` semantics are unreliable on NFS. If
    /// `XDG_CACHE_HOME` points to an NFS mount, callers should serialize at the
    /// application layer or use a local-FS cache path.
    #[cfg_attr(alef, alef(skip))]
    pub fn ensure_languages(&self, names: &[&str]) -> Result<(), Error> {
        // ~keep Resolve aliases once at the boundary: the manifest is keyed by canonical
        // ~keep name only, and `resolve_alias` is a single-pass fixed point (pinned by a
        // ~keep test in registry.rs), so every name downstream can be treated as canonical.
        let resolved: Vec<&str> = names.iter().map(|name| crate::registry::resolve_alias(name)).collect();

        // ~keep Fast path is lock-free so readers never block on writers.
        let missing: Vec<&str> = resolved.iter().filter(|name| !self.is_cached(name)).copied().collect();
        if missing.is_empty() {
            return Ok(());
        }

        // ~keep Acquire the cross-process lock only for the mutation window; avoid TOCTOU retry loops.
        let mut lock = DownloadCacheLock::open(self.version_cache_dir()?)?;
        let _guard = lock.lock_exclusive()?;
        self.ensure_languages_locked(&resolved)
    }

    /// Inner implementation of `ensure_languages`; caller must hold the
    /// `.download.lock` cross-process exclusive lock.
    fn ensure_languages_locked(&self, names: &[&str]) -> Result<(), Error> {
        // ~keep Double-check after locking: another process may have completed while we waited.
        let missing: Vec<&str> = names.iter().filter(|name| !self.is_cached(name)).copied().collect();
        if missing.is_empty() {
            return Ok(());
        }

        // ~keep Fetch manifest only while the caller holds the file lock.
        {
            let mut guard = self.manifest.lock().map_err(|e| Error::LockPoisoned(e.to_string()))?;
            if guard.is_none() {
                *guard = Some(self.fetch_manifest_inner_locked()?);
            }
        }

        let guard = self.manifest.lock().map_err(|e| Error::LockPoisoned(e.to_string()))?;
        let manifest = guard
            .as_ref()
            .ok_or_else(|| Error::LockPoisoned("manifest was not loaded after fetch".to_string()))?;

        for name in &missing {
            if !manifest.languages.contains_key(*name) {
                return Err(Error::Download(format!(
                    "Language '{}' is not in the download manifest, which lists {} language(s). \
                     Call `manifest_languages()` to enumerate the names that exist.",
                    name,
                    manifest.languages.len()
                )));
            }
        }

        let platform_key = Self::platform_key();
        let bundle = manifest.platforms.get(&platform_key).ok_or_else(|| {
            Error::Download(format!(
                "No pre-built parsers available for platform '{}'. Available: {:?}",
                platform_key,
                manifest.platforms.keys().collect::<Vec<_>>()
            ))
        })?;

        let archive_data = self.load_or_download_bundle(&platform_key, bundle)?;

        self.extract_languages(&archive_data, &missing)?;
        tracing::debug!(count = missing.len(), "extracted requested grammars");

        Ok(())
    }

    /// Ensure all languages in a named group are available.
    ///
    /// Acquires the cross-process lock once and delegates to
    /// `ensure_languages_locked` to avoid re-entrant fd_lock acquisition
    /// (`flock` on the same fd is not reentrant on Linux).
    ///
    /// The manifest is resolved via `group_languages_fast` which reads the
    /// on-disk cache without locking when possible; the file lock is only
    /// acquired for the actual download mutation (or if the manifest itself
    /// must be fetched from the network).
    #[cfg_attr(alef, alef(skip))]
    pub fn ensure_group(&self, group: &str) -> Result<(), Error> {
        // ~keep Resolve group names lock-free when the manifest is cached; lock only for network fetch.
        let group_langs = self.group_languages_fast(group)?;

        // ~keep Fast path: all languages cached, no lock needed.
        let any_missing = group_langs.iter().any(|n| !self.is_cached(n));
        if !any_missing {
            return Ok(());
        }

        // ~keep Do not call `ensure_languages()` here; fd locks are not reentrant and can deadlock.
        let mut lock = DownloadCacheLock::open(self.version_cache_dir()?)?;
        let _guard = lock.lock_exclusive()?;
        self.ensure_languages_locked(&group_langs.iter().map(String::as_str).collect::<Vec<_>>())
    }

    /// Check if a language library is already in the cache.
    ///
    /// Accepts a language name or alias; resolution happens in [`Self::lib_path`].
    fn is_cached(&self, name: &str) -> bool {
        self.lib_path(name).exists()
    }

    /// Get the expected path for a language's shared library in the cache.
    ///
    /// Accepts a language name or alias (e.g. `"shell"` resolves to `"bash"`
    /// before the C symbol lookup), so every public entry point on this type
    /// agrees on the same path for an alias and its canonical target.
    #[cfg_attr(alef, alef(skip))]
    pub fn lib_path(&self, name: &str) -> PathBuf {
        let name = crate::registry::resolve_alias(name);
        let lib_name = format!("tree_sitter_{}", crate::registry::c_symbol_for(name));
        let (prefix, ext) = if cfg!(target_os = "macos") {
            ("lib", "dylib")
        } else if cfg!(target_os = "windows") {
            ("", "dll")
        } else {
            ("lib", "so")
        };
        self.cache_dir.join(format!("{prefix}{lib_name}.{ext}"))
    }

    /// Fetch the parser manifest from GitHub Releases.
    #[cfg_attr(alef, alef(skip))]
    pub fn fetch_manifest(&self) -> Result<ParserManifest, Error> {
        // ~keep Serialize the public manifest network fetch and atomic write against concurrent processes.
        let mut lock = DownloadCacheLock::open(self.version_cache_dir()?)?;
        let _guard = lock.lock_exclusive()?;
        self.fetch_manifest_inner_locked()
    }

    /// Read the on-disk cached manifest without acquiring the file lock and
    /// without performing any network request.
    ///
    /// Returns `Some(manifest)` if the cached file exists and its version field
    /// matches `self.version`; returns `None` otherwise (absent or stale).
    fn read_cached_manifest(&self) -> Result<Option<ParserManifest>, Error> {
        let manifest_path = match self.cache_dir.parent() {
            Some(p) => p.join("manifest.json"),
            None => return Ok(None),
        };
        if !manifest_path.exists() {
            return Ok(None);
        }
        let data = fs::read_to_string(&manifest_path).map_err(|e| {
            Error::Download(format!(
                "Failed to read cached manifest {}: {e}",
                manifest_path.display()
            ))
        })?;
        let manifest: ParserManifest = serde_json::from_str(&data)?;
        if manifest.version == self.version {
            Ok(Some(manifest))
        } else {
            Ok(None)
        }
    }

    /// Internal manifest fetcher; caller **must** hold the `.download.lock`
    /// cross-process exclusive lock before calling this.
    ///
    /// Tries the on-disk cache first; falls back to a network fetch and writes
    /// the result atomically to disk.
    fn fetch_manifest_inner_locked(&self) -> Result<ParserManifest, Error> {
        // ~keep Re-check disk cache under the lock; another process may have written the manifest.
        if let Some(manifest) = self.read_cached_manifest()? {
            return Ok(manifest);
        }

        let url = resolve_manifest_url(&self.version);

        let body = if url.starts_with("file://") {
            read_file_url(&url)?
        } else {
            self.agent
                .get(&url)
                .call()
                .map_err(|e| Error::Download(format!("Failed to fetch manifest from {url}: {e}")))?
                .into_body()
                .read_to_string()
                .map_err(|e| Error::Download(format!("Failed to read manifest body: {e}")))?
        };

        let manifest: ParserManifest = serde_json::from_str(&body)?;

        // ~keep Caller holds the download cache lock, so the manifest cache write can be atomic.
        let manifest_path = self.cache_dir.parent().map(|p| p.join("manifest.json"));
        if let Some(ref path) = manifest_path {
            atomic_write(path, body.as_bytes())?;
        }

        Ok(manifest)
    }

    /// Resolve the language names belonging to `group`, acquiring the file lock
    /// only if the manifest is not yet cached on disk.
    ///
    /// This is the lock-free fast path for `ensure_group`: it reads the manifest
    /// from disk without locking when the file is already present, and falls back
    /// to the locked network-fetch path when the manifest is absent or stale.
    fn group_languages_fast(&self, group: &str) -> Result<Vec<String>, Error> {
        // ~keep Try the in-memory manifest cache first; it avoids I/O and locking.
        {
            let guard = self.manifest.lock().map_err(|e| Error::LockPoisoned(e.to_string()))?;
            if let Some(ref manifest) = *guard {
                return Self::resolve_group(manifest, group);
            }
        }

        // ~keep Reading the cached manifest from disk is lock-free because it performs no writes.
        if let Some(manifest) = self.read_cached_manifest()? {
            let names = Self::resolve_group(&manifest, group)?;
            // ~keep Populate in-memory cache for subsequent calls.
            *self.manifest.lock().map_err(|e| Error::LockPoisoned(e.to_string()))? = Some(manifest);
            return Ok(names);
        }

        // ~keep Absent/stale manifests must be fetched under the file lock to serialize the write.
        let mut lock = DownloadCacheLock::open(self.version_cache_dir()?)?;
        let _guard = lock.lock_exclusive()?;

        // ~keep Double-check after acquiring the lock; another process may have written the manifest.
        let manifest = {
            let mut mem_guard = self.manifest.lock().map_err(|e| Error::LockPoisoned(e.to_string()))?;
            if let Some(ref existing) = *mem_guard {
                return Self::resolve_group(existing, group);
            }
            let fetched = self.fetch_manifest_inner_locked()?;
            *mem_guard = Some(fetched.clone());
            fetched
        };

        Self::resolve_group(&manifest, group)
    }

    /// Extract the list of language names for `group` from a manifest, or
    /// return an error if the group is absent.
    fn resolve_group(manifest: &ParserManifest, group: &str) -> Result<Vec<String>, Error> {
        manifest
            .groups
            .get(group)
            .ok_or_else(|| {
                Error::Download(format!(
                    "Group '{}' not found. Available: {:?}",
                    group,
                    manifest.groups.keys().collect::<Vec<_>>()
                ))
            })
            .cloned()
    }

    /// Return the cache path for a verified platform bundle archive.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Download`] if `sha256` is not a well-formed 64-character
    /// hex digest — see [`validate_sha256_hex`].
    fn bundle_cache_path(&self, platform_key: &str, sha256: &str) -> Result<PathBuf, Error> {
        validate_sha256_hex(sha256)?;
        Ok(self
            .version_cache_dir()?
            .join("bundles")
            .join(format!("{platform_key}-{sha256}.tar.zst")))
    }

    fn version_cache_dir(&self) -> Result<&Path, Error> {
        self.cache_dir
            .parent()
            .ok_or_else(|| Error::Download("Cache directory has no parent".to_string()))
    }

    /// Load a verified platform bundle from cache, or download and cache it.
    /// Caller must hold the `.download.lock` cross-process exclusive lock.
    fn load_or_download_bundle(&self, platform_key: &str, bundle: &PlatformBundle) -> Result<Vec<u8>, Error> {
        let cache_path = self.bundle_cache_path(platform_key, &bundle.sha256)?;

        // ~keep Re-check bundle cache first; another process may have written it while we waited.
        if let Some(data) = Self::load_verified_cached_bundle(&cache_path, &bundle.sha256)? {
            return Ok(data);
        }

        let data = self.download_bundle(&bundle.url, bundle.size)?;
        let actual_hash = Self::sha256_hex(&data);
        if actual_hash != bundle.sha256 {
            return Err(Error::ChecksumMismatch {
                file: bundle.url.clone(),
                expected: bundle.sha256.clone(),
                actual: actual_hash,
            });
        }

        // ~keep Atomic write ensures concurrent readers see the complete bundle or nothing.
        atomic_write(&cache_path, &data)?;
        Ok(data)
    }

    fn load_verified_cached_bundle(cache_path: &Path, expected_sha256: &str) -> Result<Option<Vec<u8>>, Error> {
        if !cache_path.exists() {
            return Ok(None);
        }

        let data = fs::read(cache_path)
            .map_err(|e| Error::Download(format!("Failed to read cached bundle {}: {e}", cache_path.display())))?;
        let actual_hash = Self::sha256_hex(&data);
        if actual_hash == expected_sha256 {
            return Ok(Some(data));
        }

        fs::remove_file(cache_path).map_err(|e| {
            Error::Download(format!(
                "Failed to remove corrupt cached bundle {}: {e}",
                cache_path.display()
            ))
        })?;
        Ok(None)
    }

    /// Download a bundle archive from the given URL, reading at most
    /// [`bundle_read_cap`]`(expected_size)` bytes.
    ///
    /// `expected_size` comes from the manifest's [`PlatformBundle::size`] — an
    /// untrusted, network-fetched value — so the cap it produces is itself
    /// clamped to [`MAX_BUNDLE_DOWNLOAD_BYTES`]. Without a cap, a hostile or
    /// malfunctioning server could stream an unbounded response and exhaust
    /// process memory before [`Self::sha256_hex`] ever runs over the data. See
    /// #101 L1. ~keep
    fn download_bundle(&self, url: &str, expected_size: u64) -> Result<Vec<u8>, Error> {
        if let Some(path) = url.strip_prefix("file://") {
            return fs::read(path).map_err(|e| Error::Download(format!("Failed to read bundle from {url}: {e}")));
        }

        tracing::info!(url, expected_size, "downloading parser bundle");
        let response = self
            .agent
            .get(url)
            .call()
            .map_err(|e| Error::Download(format!("Failed to download {}: {}", url, e)))?;

        let cap = bundle_read_cap(expected_size);
        let mut data = Vec::new();
        // ~keep Read one byte past `cap` so an oversized body is detected explicitly
        // ~keep below instead of being silently truncated and left to a confusing
        // ~keep checksum-mismatch error.
        response
            .into_body()
            .into_reader()
            .take(cap.saturating_add(1))
            .read_to_end(&mut data)
            .map_err(|e| Error::Download(format!("Failed to read download body: {}", e)))?;

        if data.len() as u64 > cap {
            return Err(Error::Download(format!(
                "Downloaded bundle from {url} exceeds the expected size of {cap} bytes; aborting"
            )));
        }

        Ok(data)
    }

    /// Extract specific languages from a zstd-compressed tar archive into
    /// the cache directory.
    ///
    /// Writes each library file atomically via a sibling-tmp-then-rename so
    /// concurrent readers always see either the old version or the new version,
    /// never partial bytes.
    ///
    /// **Precondition**: caller must hold the `.download.lock` cross-process
    /// exclusive lock (via [`DownloadCacheLock`]) when multiple processes may
    /// write to the same cache directory simultaneously. Exposing this method
    /// publicly would allow callers to bypass the lock entirely.
    pub(crate) fn extract_languages(&self, archive_data: &[u8], names: &[&str]) -> Result<(), Error> {
        ensure_secure_cache_dir(&self.cache_dir)?;

        let decoder = zstd::Decoder::new(archive_data)
            .map_err(|e| Error::Download(format!("Failed to decompress archive: {}", e)))?;
        let mut archive = tar::Archive::new(decoder);

        let mut expected_files: HashMap<String, &str> = HashMap::with_capacity(names.len());
        for name in names {
            let path = self.lib_path(name);
            let filename = path
                .file_name()
                .ok_or_else(|| Error::Download(format!("lib_path for '{name}' has no filename")))?
                .to_string_lossy()
                .to_string();
            expected_files.insert(filename, name);
        }
        let mut extracted_files = HashSet::with_capacity(expected_files.len());

        for entry in archive
            .entries()
            .map_err(|e| Error::Download(format!("Failed to read archive entries: {}", e)))?
        {
            let mut entry = entry.map_err(|e| Error::Download(format!("Failed to read archive entry: {}", e)))?;
            let path = entry
                .path()
                .map_err(|e| Error::Download(format!("Failed to read entry path: {}", e)))?;

            let filename = path
                .file_name()
                .map(|f| f.to_string_lossy().into_owned())
                .unwrap_or_default();

            if expected_files.contains_key(&filename) {
                let dest = self.cache_dir.join(&filename);
                // ~keep Atomic copy makes concurrent readers see the complete library or nothing.
                atomic_copy_from_reader(&dest, &mut entry)
                    .map_err(|e| Error::Download(format!("Failed to extract {}: {}", filename, e)))?;
                extracted_files.insert(filename);
            }
        }

        // ~keep Retry existence checks to tolerate concurrent cache cleanup/re-extraction races.
        let mut missing_languages: Vec<&str> = expected_files
            .iter()
            .filter_map(|(filename, name)| {
                if extracted_files.contains(filename) {
                    return None;
                }
                let path = self.cache_dir.join(filename);
                for _ in 0..3 {
                    if path.exists() {
                        return None;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
                (!path.exists()).then_some(*name)
            })
            .collect();
        missing_languages.sort_unstable();

        if !missing_languages.is_empty() {
            return Err(Error::Download(format!(
                "Downloaded archive did not contain parser libraries for: {}",
                missing_languages.join(", ")
            )));
        }

        Ok(())
    }

    /// Thin public re-export of `extract_languages` gated on the
    /// `test-internals` feature (or `#[cfg(test)]`).
    ///
    /// Integration tests that need to call `extract_languages` directly (e.g.
    /// the cross-process concurrency test) should use this wrapper so they do
    /// not bypass the cross-process file lock invisibly. The name makes the
    /// test-only nature obvious.
    #[cfg(any(test, feature = "test-internals"))]
    #[cfg_attr(alef, alef(skip))]
    pub fn _testing_extract_languages(&self, archive_data: &[u8], names: &[&str]) -> Result<(), Error> {
        self.extract_languages(archive_data, names)
    }

    /// Download the platform bundle and extract every library file it contains.
    ///
    /// Unlike [`Self::ensure_languages`], this does not check the manifest language list
    /// against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
    /// from the bundle. Languages in the manifest that are missing from the archive
    /// are silently ignored rather than returning an error.
    ///
    /// Returns the number of library files extracted (including those already cached).
    pub fn download_all_best_effort(&self) -> Result<usize, Error> {
        // ~keep Keep manifest fetch/write and bundle download/extract inside one lock to prevent TOCTOU races.
        let mut lock = DownloadCacheLock::open(self.version_cache_dir()?)?;
        let _guard = lock.lock_exclusive()?;
        self.download_all_best_effort_locked()
    }

    /// Inner implementation of `download_all_best_effort`; caller must hold the
    /// `.download.lock` cross-process exclusive lock.
    fn download_all_best_effort_locked(&self) -> Result<usize, Error> {
        // ~keep Load or fetch the manifest under the download lock.
        {
            let mut guard = self.manifest.lock().map_err(|e| Error::LockPoisoned(e.to_string()))?;
            if guard.is_none() {
                *guard = Some(self.fetch_manifest_inner_locked()?);
            }
        }

        let (platform_key, bundle) = {
            let guard = self.manifest.lock().map_err(|e| Error::LockPoisoned(e.to_string()))?;
            let manifest = guard
                .as_ref()
                .ok_or_else(|| Error::LockPoisoned("manifest was not loaded after fetch".to_string()))?;
            let platform_key = Self::platform_key();
            let bundle = manifest.platforms.get(&platform_key).ok_or_else(|| {
                Error::Download(format!(
                    "No pre-built parsers available for platform '{}'. Available: {:?}",
                    platform_key,
                    manifest.platforms.keys().collect::<Vec<_>>()
                ))
            })?;
            (platform_key, bundle.clone())
        };

        let archive_data = self.load_or_download_bundle(&platform_key, &bundle)?;
        self.extract_all_libs(&archive_data)
    }

    /// Extract every library file from a zstd-compressed tar archive into the cache directory.
    ///
    /// Files are matched by extension (`.so`, `.dylib`, `.dll`) — no per-language
    /// verification is performed. Returns the count of files now present in the cache dir.
    /// Caller must hold the `.download.lock` cross-process exclusive lock.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Download`] if extraction fails, or if the final on-disk
    /// count cannot be read — a previous version swallowed that count failure as
    /// `0`, silently reporting "nothing extracted" after a successful extraction.
    /// See #107. ~keep
    fn extract_all_libs(&self, archive_data: &[u8]) -> Result<usize, Error> {
        ensure_secure_cache_dir(&self.cache_dir)?;

        let (lib_prefix, lib_ext) = if cfg!(target_os = "macos") {
            ("lib", "dylib")
        } else if cfg!(target_os = "windows") {
            ("", "dll")
        } else {
            ("lib", "so")
        };

        let decoder = zstd::Decoder::new(archive_data)
            .map_err(|e| Error::Download(format!("Failed to decompress archive: {}", e)))?;
        let mut archive = tar::Archive::new(decoder);

        for entry in archive
            .entries()
            .map_err(|e| Error::Download(format!("Failed to read archive entries: {}", e)))?
        {
            let mut entry = entry.map_err(|e| Error::Download(format!("Failed to read archive entry: {}", e)))?;
            let path = entry
                .path()
                .map_err(|e| Error::Download(format!("Failed to read entry path: {}", e)))?;

            let filename = path
                .file_name()
                .map(|f| f.to_string_lossy().into_owned())
                .unwrap_or_default();

            let is_lib = filename.ends_with(&format!(".{lib_ext}"))
                && (lib_prefix.is_empty() || filename.starts_with(lib_prefix));

            if is_lib {
                let dest = self.cache_dir.join(&filename);
                // ~keep Skip cached files in download_all; atomic rename on every library is wasteful.
                if !dest.exists() {
                    // ~keep Atomic copy makes concurrent readers see the complete library or nothing.
                    atomic_copy_from_reader(&dest, &mut entry)
                        .map_err(|e| Error::Download(format!("Failed to extract {}: {}", filename, e)))?;
                }
            }
        }

        let count = fs::read_dir(&self.cache_dir)
            .map_err(|e| {
                Error::Download(format!(
                    "Extraction succeeded but failed to count cached libraries in {}: {e}",
                    self.cache_dir.display()
                ))
            })?
            .flatten()
            .filter(|e| {
                let name = e.file_name().to_string_lossy().into_owned();
                name.ends_with(&format!(".{lib_ext}"))
            })
            .count();

        Ok(count)
    }

    /// Remove all cached parser libraries.
    ///
    /// Acquires the cross-process lock so `clean_cache` cannot race a concurrent
    /// downloader (avoids Windows sharing-violation errors against an in-flight
    /// bundle write). The `.download.lock` file itself is **not** removed — it is
    /// permanent infrastructure; deleting it could allow a concurrent process that
    /// already opened the file to continue holding a stale lock handle while a new
    /// process opens a fresh inode, breaking the mutual-exclusion guarantee.
    pub fn clean_cache(&self) -> Result<(), Error> {
        let version_cache_dir = self.version_cache_dir()?;
        let mut lock = DownloadCacheLock::open(version_cache_dir)?;
        let _guard = lock.lock_exclusive()?;
        self.clean_cache_locked()
    }

    /// Inner implementation of `clean_cache`; caller must hold the
    /// `.download.lock` cross-process exclusive lock.
    fn clean_cache_locked(&self) -> Result<(), Error> {
        Self::remove_dir_if_exists(&self.cache_dir)?;
        let version_cache_dir = self.version_cache_dir()?;
        let bundle_dir = version_cache_dir.join("bundles");
        Self::remove_dir_if_exists(&bundle_dir)?;
        let manifest_path = version_cache_dir.join("manifest.json");
        Self::remove_file_if_exists(&manifest_path)?;
        // ~keep Never remove LOCK_FILE_NAME; deleting it breaks flock exclusion across old/new inodes.
        Ok(())
    }

    fn remove_dir_if_exists(path: &Path) -> Result<(), Error> {
        for attempt in 0..=CACHE_REMOVE_RETRIES {
            match fs::remove_dir_all(path) {
                Ok(()) => return Ok(()),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
                // ~keep Retry DirectoryNotEmpty and Windows PermissionDenied from concurrent cache readers/writers.
                Err(error)
                    if (error.kind() == std::io::ErrorKind::DirectoryNotEmpty
                        || error.kind() == std::io::ErrorKind::PermissionDenied)
                        && attempt < CACHE_REMOVE_RETRIES =>
                {
                    thread::sleep(CACHE_REMOVE_RETRY_DELAY);
                }
                Err(error) => return Err(error.into()),
            }
        }
        Ok(())
    }

    fn remove_file_if_exists(path: &Path) -> Result<(), Error> {
        match fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    /// Compute SHA-256 hex digest.
    fn sha256_hex(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        hash.iter().fold(String::with_capacity(hash.len() * 2), |mut s, byte| {
            use std::fmt::Write as _;
            let _ = write!(s, "{byte:02x}");
            s
        })
    }

    /// Platform key for the current OS/arch, e.g. "linux-x86_64", "macos-arm64".
    fn platform_key() -> String {
        let os = if cfg!(target_os = "macos") {
            "macos"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else {
            "linux"
        };

        let arch = if cfg!(target_arch = "aarch64") {
            if cfg!(target_os = "macos") { "arm64" } else { "aarch64" }
        } else if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else {
            std::env::consts::ARCH
        };

        format!("{os}-{arch}")
    }
}

#[cfg(test)]
mod tests;
