use std::collections::{HashMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

/// Monotonically increasing counter that makes sibling-tmp paths unique within
/// the current process even when `SystemTime::now()` returns the same value for
/// two threads (possible on systems where the wall clock has coarse resolution).
static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

use fd_lock::RwLock as FdRwLock;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::Error;
use crate::pack_config::TlsRootsMode;

const GITHUB_RELEASE_BASE: &str = "https://github.com/kreuzberg-dev/tree-sitter-language-pack/releases/download";
const CACHE_REMOVE_RETRIES: usize = 5;
const CACHE_REMOVE_RETRY_DELAY: Duration = Duration::from_millis(10);
const HTTP_TIMEOUT: Duration = Duration::from_secs(60);
const TLS_ROOTS_ENV: &str = "TREE_SITTER_LANGUAGE_PACK_TLS_ROOTS";
const MANIFEST_URL_ENV: &str = "TREE_SITTER_LANGUAGE_PACK_MANIFEST_URL";
const LOCK_FILE_NAME: &str = ".download.lock";

/// Resolve the URL used to fetch the parser manifest.
///
/// Layered override: `TREE_SITTER_LANGUAGE_PACK_MANIFEST_URL` env var (if set
/// and non-empty) wins over the compile-time GitHub release URL. Allows tests,
/// air-gapped deployments, and private mirrors to redirect manifest fetches
/// without recompiling. Supports both `http(s)://` (over the ureq agent) and
/// `file://` (read straight from disk) schemes.
fn resolve_manifest_url(version: &str) -> String {
    if let Ok(url) = std::env::var(MANIFEST_URL_ENV)
        && !url.trim().is_empty()
    {
        return url;
    }
    format!("{GITHUB_RELEASE_BASE}/v{version}/parsers.json")
}

/// Read a `file://` URL as a UTF-8 string.
///
/// Returns the body bytes decoded as UTF-8. Errors are wrapped in
/// `Error::Download` so callers see the same error variant whether the
/// manifest was fetched over HTTP or read from disk.
fn read_file_url(url: &str) -> Result<String, Error> {
    let path = url
        .strip_prefix("file://")
        .ok_or_else(|| Error::Download(format!("not a file:// URL: {url}")))?;
    fs::read_to_string(path).map_err(|e| Error::Download(format!("Failed to read manifest from {url}: {e}")))
}

/// Sibling tmp path for atomic writes: `<dest_dir>/.<name>.tmp.<pid>.<seq>`.
/// Lives in the same directory as `dest` so `fs::rename` stays on the same
/// filesystem (cross-FS rename returns `EXDEV`).
///
/// The `<seq>` component is a per-process monotonic counter that makes the
/// path unique even when `SystemTime::now()` resolves to the same instant for
/// two threads (possible on hosts where the wall clock has coarse resolution).
fn sibling_tmp_path(dest: &Path) -> Result<PathBuf, Error> {
    let parent = dest
        .parent()
        .ok_or_else(|| Error::CacheLock(format!("destination has no parent dir: {}", dest.display())))?;
    let name = dest
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| Error::CacheLock(format!("destination has no filename: {}", dest.display())))?;
    let seq = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    Ok(parent.join(format!(".{}.tmp.{}.{}", name, std::process::id(), seq)))
}

/// Write `data` atomically to `dest`: write to a sibling tmp file then rename.
/// On any error the tmp file is removed. Concurrent readers see either the old
/// version, the new version, or no file — never partial bytes.
fn atomic_write(dest: &Path, data: &[u8]) -> Result<(), Error> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = sibling_tmp_path(dest)?;
    let write_result = (|| -> io::Result<()> {
        let mut f = File::create(&tmp)?;
        f.write_all(data)?;
        f.sync_all()?;
        Ok(())
    })();
    if let Err(e) = write_result {
        let _ = fs::remove_file(&tmp);
        return Err(e.into());
    }
    if let Err(e) = fs::rename(&tmp, dest) {
        let _ = fs::remove_file(&tmp);
        return Err(e.into());
    }
    Ok(())
}

/// Stream from `src` into a sibling tmp file and rename atomically to `dest`.
/// Avoids the double allocation of `read_to_end + atomic_write` for tar entries.
///
/// If the buffered flush fails, the tmp file is removed and no rename occurs;
/// the destination is unchanged. The explicit `flush()` before `into_inner()`
/// surfaces flush errors at a clear call site before the sync.
fn atomic_copy_from_reader<R: Read>(dest: &Path, src: &mut R) -> Result<(), Error> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = sibling_tmp_path(dest)?;
    let copy_result = (|| -> io::Result<()> {
        let f = File::create(&tmp)?;
        let mut writer = BufWriter::new(f);
        io::copy(src, &mut writer)?;
        // Explicit flush before into_inner surfaces buffered-write errors with a
        // clearer call site for debugging, before we attempt sync_all.
        writer.flush()?;
        let f = writer.into_inner().map_err(|e| e.into_error())?;
        f.sync_all()?;
        Ok(())
    })();
    if let Err(e) = copy_result {
        let _ = fs::remove_file(&tmp);
        return Err(e.into());
    }
    if let Err(e) = fs::rename(&tmp, dest) {
        let _ = fs::remove_file(&tmp);
        return Err(e.into());
    }
    Ok(())
}

/// Cross-process exclusive lock guarding mutations to a download cache directory.
///
/// Backed by `fd_lock` (`flock` on Unix, `LockFileEx` on Windows). The lock
/// file (`<cache_dir>/.download.lock`) is created lazily on first acquisition
/// and is permanent infrastructure — `clean_cache` does *not* remove it, so
/// in-flight downloaders racing a cleanup remain serialized.
///
/// The intra-process `DOWNLOAD_CACHE_LOCK` mutex in `lib.rs` is layered on top
/// of this file lock as a cheap pre-filter (µs vs ms cost).
pub(crate) struct DownloadCacheLock {
    inner: FdRwLock<File>,
}

impl DownloadCacheLock {
    /// Open (or create) the lock file under `cache_dir`. Does not block; the
    /// returned value must be locked via [`Self::lock_exclusive`].
    pub(crate) fn open(cache_dir: &Path) -> Result<Self, Error> {
        fs::create_dir_all(cache_dir)
            .map_err(|e| Error::CacheLock(format!("create cache dir {}: {e}", cache_dir.display())))?;
        let lock_path = cache_dir.join(LOCK_FILE_NAME);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&lock_path)
            .map_err(|e| Error::CacheLock(format!("open lock file {}: {e}", lock_path.display())))?;
        Ok(Self {
            inner: FdRwLock::new(file),
        })
    }

    /// Block until the exclusive cross-process lock is acquired. The returned
    /// guard releases the lock on drop. No retry/backoff: callers bubble up
    /// any error cleanly to avoid TOCTOU loops.
    pub(crate) fn lock_exclusive(&mut self) -> Result<fd_lock::RwLockWriteGuard<'_, File>, Error> {
        self.inner
            .write()
            .map_err(|e| Error::CacheLock(format!("acquire exclusive download lock: {e}")))
    }
}

/// Resolve which CA root set the downloader's TLS client should trust.
///
/// Layered override: `tls_roots` struct field (if `Some`) > `TREE_SITTER_LANGUAGE_PACK_TLS_ROOTS`
/// environment variable (`platform` or `webpki`, case-insensitive) > compile-time default.
fn resolve_tls_roots(override_mode: Option<TlsRootsMode>) -> TlsRootsMode {
    if let Some(mode) = override_mode {
        return mode;
    }
    match std::env::var(TLS_ROOTS_ENV)
        .ok()
        .as_deref()
        .map(str::trim)
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("webpki") => TlsRootsMode::WebPki,
        Some("platform") => TlsRootsMode::Platform,
        // Empty / unset / any other value → compile-time default. Unknown
        // values are *not* hard errors because PackConfig is the supported
        // override path for picky callers.
        _ => TlsRootsMode::default(),
    }
}

/// Build a configured ureq `Agent` whose TLS root set follows the given mode.
fn build_agent(mode: TlsRootsMode) -> ureq::Agent {
    let root_certs = match mode {
        TlsRootsMode::Platform => ureq::tls::RootCerts::PlatformVerifier,
        TlsRootsMode::WebPki => ureq::tls::RootCerts::WebPki,
    };
    ureq::Agent::config_builder()
        .tls_config(ureq::tls::TlsConfig::builder().root_certs(root_certs).build())
        .timeout_global(Some(HTTP_TIMEOUT))
        // Honor standard proxy environment variables (HTTPS_PROXY / HTTP_PROXY /
        // ALL_PROXY, incl. socks5://). Without this the agent always connects
        // directly, so on networks where the GitHub release host is only
        // reachable through a proxy the grammar download stalls until the
        // global timeout fires (and pre-timeout builds hung forever). Proxy is
        // applied only when the env vars are set; otherwise this is a no-op.
        .proxy(ureq::Proxy::try_from_env())
        .build()
        .new_agent()
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
    /// Named language groups (e.g., `"web"`, `"systems"`), each mapping to a list of language names.
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
    pub fn new(version: &str) -> Result<Self, Error> {
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
    #[cfg_attr(alef, alef(skip))]
    pub fn default_cache_dir(version: &str) -> Result<PathBuf, Error> {
        let base = dirs::cache_dir()
            .ok_or_else(|| Error::Download("Could not determine system cache directory".to_string()))?;
        Ok(base
            .join("tree-sitter-language-pack")
            .join(format!("v{version}"))
            .join("libs"))
    }

    /// Return the path to the libs cache directory.
    #[cfg_attr(alef, alef(skip))]
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// List languages that are already downloaded and cached.
    pub fn installed_languages(&self) -> Vec<String> {
        let mut langs = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                // Extract language name from library filename: libtree_sitter_<name>.so
                if let Some(lang) = Self::lang_from_lib_filename(&name) {
                    langs.push(lang);
                }
            }
        }
        langs.sort();
        langs
    }

    /// Extract language name from a shared library filename.
    ///
    /// Reverses the `c_symbol_for` mapping: e.g. `libtree_sitter_c_sharp.dylib` → `"csharp"`.
    fn lang_from_lib_filename(filename: &str) -> Option<String> {
        let name = filename.strip_prefix("lib").unwrap_or(filename);
        let name = name
            .strip_prefix("tree_sitter_")
            .or_else(|| name.strip_prefix("tree-sitter-"))?;
        let name = name
            .strip_suffix(".so")
            .or_else(|| name.strip_suffix(".dylib"))
            .or_else(|| name.strip_suffix(".dll"))?;
        // Reverse c_symbol mapping: if the stripped name is a c_symbol value,
        // return the original language name instead.
        Some(crate::registry::lang_name_for_symbol(name).to_string())
    }

    /// Ensure the specified languages are available in the cache.
    /// Downloads the platform bundle if any requested languages are missing.
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
        // FAST PATH: lock-free check — readers must never block on writers.
        let missing: Vec<&str> = names.iter().filter(|name| !self.is_cached(name)).copied().collect();
        if missing.is_empty() {
            return Ok(());
        }

        // SLOW PATH: acquire cross-process lock for the mutation window.
        // No retry/backoff: bubble error cleanly to avoid TOCTOU retry loops.
        let mut lock = DownloadCacheLock::open(self.version_cache_dir()?)?;
        let _guard = lock.lock_exclusive()?;
        self.ensure_languages_locked(names)
    }

    /// Inner implementation of `ensure_languages`; caller must hold the
    /// `.download.lock` cross-process exclusive lock.
    fn ensure_languages_locked(&self, names: &[&str]) -> Result<(), Error> {
        // DOUBLE-CHECK: another process may have completed while we waited.
        let missing: Vec<&str> = names.iter().filter(|name| !self.is_cached(name)).copied().collect();
        if missing.is_empty() {
            return Ok(());
        }

        // Fetch manifest if not already loaded (caller holds file lock).
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

        // Verify requested languages exist in manifest
        for name in &missing {
            if !manifest.languages.contains_key(*name) {
                return Err(Error::Download(format!(
                    "Language '{}' not available for download. Available groups: {:?}",
                    name,
                    manifest.groups.keys().collect::<Vec<_>>()
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

        // Extract only the requested languages
        self.extract_languages(&archive_data, &missing)?;

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
        // Resolve group → [language names] using the lock-free fast path when
        // the manifest is already cached; acquires the lock only if a network
        // fetch is required.
        let group_langs = self.group_languages_fast(group)?;

        // FAST PATH: all languages cached — no lock needed.
        let any_missing = group_langs.iter().any(|n| !self.is_cached(n));
        if !any_missing {
            return Ok(());
        }

        // SLOW PATH: acquire cross-process lock once, then call the locked inner.
        // Do NOT call ensure_languages() here — that would attempt a second
        // fd_lock acquisition on the same file descriptor, which is not
        // reentrant on Linux (flock) and would deadlock on Windows (LockFileEx).
        let mut lock = DownloadCacheLock::open(self.version_cache_dir()?)?;
        let _guard = lock.lock_exclusive()?;
        self.ensure_languages_locked(&group_langs.iter().map(String::as_str).collect::<Vec<_>>())
    }

    /// Check if a language library is already in the cache.
    fn is_cached(&self, name: &str) -> bool {
        self.lib_path(name).exists()
    }

    /// Get the expected path for a language's shared library in the cache.
    #[cfg_attr(alef, alef(skip))]
    pub fn lib_path(&self, name: &str) -> PathBuf {
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
        // Public entry-point: acquire the lock so the network fetch + atomic
        // write are serialized against concurrent processes.
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
        let data = fs::read_to_string(&manifest_path)?;
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
        // Re-check disk cache under the lock — another process may have already
        // written the manifest while we were waiting.
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

        // Cache the manifest atomically — caller holds the download cache lock.
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
        // Try the in-memory cache first (cheapest path, no I/O).
        {
            let guard = self.manifest.lock().map_err(|e| Error::LockPoisoned(e.to_string()))?;
            if let Some(ref manifest) = *guard {
                return Self::resolve_group(manifest, group);
            }
        }

        // Try reading from disk without the file lock (pure read, no write).
        if let Some(manifest) = self.read_cached_manifest()? {
            let names = Self::resolve_group(&manifest, group)?;
            // Populate in-memory cache for subsequent calls.
            *self.manifest.lock().map_err(|e| Error::LockPoisoned(e.to_string()))? = Some(manifest);
            return Ok(names);
        }

        // Manifest is absent or stale — must fetch from the network under the
        // file lock so the write is serialized against concurrent processes.
        let mut lock = DownloadCacheLock::open(self.version_cache_dir()?)?;
        let _guard = lock.lock_exclusive()?;

        // Double-check after acquiring the lock — another process may have
        // written the manifest while we waited.
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
    fn bundle_cache_path(&self, platform_key: &str, sha256: &str) -> Result<PathBuf, Error> {
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

        // Re-check cache first — another process may have written the bundle
        // while we waited for the lock.
        if let Some(data) = Self::load_verified_cached_bundle(&cache_path, &bundle.sha256)? {
            return Ok(data);
        }

        let data = self.download_bundle(&bundle.url)?;
        let actual_hash = Self::sha256_hex(&data);
        if actual_hash != bundle.sha256 {
            return Err(Error::ChecksumMismatch {
                file: bundle.url.clone(),
                expected: bundle.sha256.clone(),
                actual: actual_hash,
            });
        }

        // Write atomically — concurrent readers see complete bundle or nothing.
        atomic_write(&cache_path, &data)?;
        Ok(data)
    }

    fn load_verified_cached_bundle(cache_path: &Path, expected_sha256: &str) -> Result<Option<Vec<u8>>, Error> {
        if !cache_path.exists() {
            return Ok(None);
        }

        let data = fs::read(cache_path)?;
        let actual_hash = Self::sha256_hex(&data);
        if actual_hash == expected_sha256 {
            return Ok(Some(data));
        }

        fs::remove_file(cache_path)?;
        Ok(None)
    }

    /// Download a bundle archive from the given URL.
    fn download_bundle(&self, url: &str) -> Result<Vec<u8>, Error> {
        if let Some(path) = url.strip_prefix("file://") {
            return fs::read(path).map_err(|e| Error::Download(format!("Failed to read bundle from {url}: {e}")));
        }

        let response = self
            .agent
            .get(url)
            .call()
            .map_err(|e| Error::Download(format!("Failed to download {}: {}", url, e)))?;

        let mut data = Vec::new();
        response
            .into_body()
            .into_reader()
            .read_to_end(&mut data)
            .map_err(|e| Error::Download(format!("Failed to read download body: {}", e)))?;

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
        fs::create_dir_all(&self.cache_dir)?;

        let decoder = zstd::Decoder::new(archive_data)
            .map_err(|e| Error::Download(format!("Failed to decompress archive: {}", e)))?;
        let mut archive = tar::Archive::new(decoder);

        // Build a set of expected filenames for the requested languages.
        // lib_path always ends in "{prefix}{lib_name}.{ext}", so file_name()
        // is always Some; return an error if the invariant is somehow violated.
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
                // Atomic copy: write to sibling tmp then rename so concurrent
                // readers see the complete library or nothing.
                atomic_copy_from_reader(&dest, &mut entry)
                    .map_err(|e| Error::Download(format!("Failed to extract {}: {}", filename, e)))?;
                extracted_files.insert(filename);
            }
        }

        // Verify each not-just-extracted language is still on disk. Under
        // concurrent test workloads (e.g. JUnit SmokeTest spanning 306
        // languages running alongside DownloadTest.testDownloadCleanCache),
        // a bare `.exists()` here races with cache cleanup and falsely
        // reports as missing a file that another worker is about to re-extract.
        // Retry the existence check a few times with short backoff to close
        // that window without restructuring the lock.
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
        // Acquire cross-process lock FIRST — the manifest fetch + write and the
        // bundle download + extract must all be inside the locked region to
        // prevent TOCTOU races between concurrent processes.
        let mut lock = DownloadCacheLock::open(self.version_cache_dir()?)?;
        let _guard = lock.lock_exclusive()?;
        self.download_all_best_effort_locked()
    }

    /// Inner implementation of `download_all_best_effort`; caller must hold the
    /// `.download.lock` cross-process exclusive lock.
    fn download_all_best_effort_locked(&self) -> Result<usize, Error> {
        // Load or fetch the manifest under the lock.
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
    fn extract_all_libs(&self, archive_data: &[u8]) -> Result<usize, Error> {
        fs::create_dir_all(&self.cache_dir)?;

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
                // Short-circuit: skip files that are already present; atomic
                // rename on every file would be wasteful during download_all.
                if !dest.exists() {
                    // Atomic copy: write to sibling tmp then rename so concurrent
                    // readers see the complete library or nothing.
                    atomic_copy_from_reader(&dest, &mut entry)
                        .map_err(|e| Error::Download(format!("Failed to extract {}: {}", filename, e)))?;
                }
            }
        }

        // Count all library files now in the cache directory.
        let count = fs::read_dir(&self.cache_dir)
            .map(|entries| {
                entries
                    .flatten()
                    .filter(|e| {
                        let name = e.file_name().to_string_lossy().into_owned();
                        name.ends_with(&format!(".{lib_ext}"))
                    })
                    .count()
            })
            .unwrap_or(0);

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
        // Ensure the version cache dir and lock file exist before opening.
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
        // NOTE: Do NOT remove LOCK_FILE_NAME — it is permanent infrastructure.
        // Deleting it while another process holds the fd would silently break
        // mutual exclusion (new opener gets a fresh inode, old holder retains
        // the deleted inode, flock no longer serializes them).
        Ok(())
    }

    fn remove_dir_if_exists(path: &Path) -> Result<(), Error> {
        for attempt in 0..=CACHE_REMOVE_RETRIES {
            match fs::remove_dir_all(path) {
                Ok(()) => return Ok(()),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
                // Retry on DirectoryNotEmpty (concurrent writer still active) and
                // PermissionDenied (Windows sharing-violation: a reader has a
                // `.dll` open via `LoadLibrary` while we attempt removal).
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
mod tests {
    use std::sync::Arc;

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
    fn bundle_cache_path_uses_version_cache_dir() {
        let temp_dir = temp_cache_dir();
        let cache_dir = temp_dir.path().join("libs");
        let manager = manager_for_temp_dir(&temp_dir);

        let path = manager
            .bundle_cache_path("macos-arm64", "abc123")
            .expect("bundle cache path should resolve");

        assert_eq!(
            path,
            cache_dir.parent().unwrap().join("bundles/macos-arm64-abc123.tar.zst")
        );
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

        let cached = DownloadManager::load_verified_cached_bundle(&cache_path, &sha256)
            .expect("verified cache read should succeed");

        assert_eq!(cached, Some(data.to_vec()));
        assert!(cache_path.exists());
    }

    #[test]
    fn verified_bundle_cache_removes_hash_mismatch() {
        let temp_dir = temp_cache_dir();
        let manager = manager_for_temp_dir(&temp_dir);
        let cache_path = manager
            .bundle_cache_path("macos-arm64", "expected-hash")
            .expect("bundle cache path should resolve");
        fs::create_dir_all(cache_path.parent().unwrap()).expect("bundle cache directory should be created");
        fs::write(&cache_path, b"corrupt archive bytes").expect("bundle cache file should be written");

        let cached = DownloadManager::load_verified_cached_bundle(&cache_path, "expected-hash")
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

    // ----- TLS root selection (#125) -----
    //
    // Env-var-mutating tests share a single mutex guard so they run serially
    // even under `cargo test` default parallelism. Without this, concurrent
    // tests racing on `TREE_SITTER_LANGUAGE_PACK_TLS_ROOTS` would flake.
    static TLS_ENV_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());

    struct EnvVarGuard {
        key: &'static str,
        previous: Option<String>,
    }

    impl EnvVarGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let previous = std::env::var(key).ok();
            // SAFETY: tests serialised via TLS_ENV_GUARD; no other test thread
            // is concurrently observing or mutating the same env var.
            unsafe { std::env::set_var(key, value) };
            Self { key, previous }
        }

        fn unset(key: &'static str) -> Self {
            let previous = std::env::var(key).ok();
            // SAFETY: see set() above.
            unsafe { std::env::remove_var(key) };
            Self { key, previous }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            // SAFETY: see set() above.
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
        // Env var should be ignored when the caller passes an explicit override.
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
        // Default must be Platform: that is the user-facing fix for #125.
        assert_eq!(TlsRootsMode::default(), TlsRootsMode::Platform);
    }

    #[test]
    fn resolve_tls_roots_falls_back_to_default_when_env_is_garbage() {
        let _guard = TLS_ENV_GUARD.lock().expect("env guard should not be poisoned");
        // Unknown values are not hard errors — fall back to the default rather than
        // panicking deep inside the downloader at first use.
        let _env = EnvVarGuard::set(TLS_ROOTS_ENV, "not-a-mode");
        assert_eq!(resolve_tls_roots(None), TlsRootsMode::default());
    }

    #[test]
    fn build_agent_platform_mode_constructs_an_agent() {
        // Smoke test: building the agent in platform-verifier mode succeeds on
        // every supported host (the verifier itself only fails when the request
        // actually reaches the network, which is covered by the integration test).
        let _agent = build_agent(TlsRootsMode::Platform);
    }

    #[test]
    fn build_agent_webpki_mode_constructs_an_agent() {
        let _agent = build_agent(TlsRootsMode::WebPki);
    }

    // ----- Manifest URL override (TREE_SITTER_LANGUAGE_PACK_MANIFEST_URL) -----
    //
    // Shares the TLS env-var test guard pattern: env-mutating tests serialise
    // on a single mutex so concurrent `cargo test` runs do not flake.
    static MANIFEST_URL_ENV_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn resolve_manifest_url_defaults_to_github_release() {
        let _guard = MANIFEST_URL_ENV_GUARD.lock().expect("env guard should not be poisoned");
        let _env = EnvVarGuard::unset(MANIFEST_URL_ENV);
        assert_eq!(
            resolve_manifest_url("1.2.3"),
            "https://github.com/kreuzberg-dev/tree-sitter-language-pack/releases/download/v1.2.3/parsers.json"
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
        // An empty or whitespace-only env value should not override the default URL —
        // the env var is "unset" semantically, not a request to fetch from the
        // empty URL (which would deterministically fail with a confusing error).
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
        // Construct in both modes; assert the agent field is populated by
        // attempting a call against a non-existent localhost port — both modes
        // must produce a connection error (not a panic, not a TLS-config panic).
        for mode in [TlsRootsMode::Platform, TlsRootsMode::WebPki] {
            let dm = DownloadManager::with_cache_dir_and_tls("test", temp_dir.path().join("libs"), Some(mode));
            let result = dm.agent.get("http://127.0.0.1:1/never").call();
            assert!(
                result.is_err(),
                "agent should fail to connect to a closed port in mode {mode:?}"
            );
        }
    }

    // ----- Phase 4: atomic-write + concurrency safety tests -----

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
        // Distinct fill bytes so a mixed read is unambiguous.
        let old_payload: Arc<Vec<u8>> = Arc::new(vec![0xAA_u8; PAYLOAD_SIZE]);
        let new_payload: Arc<Vec<u8>> = Arc::new(vec![0x55_u8; PAYLOAD_SIZE]);

        let temp_dir = temp_cache_dir();
        let path = temp_dir.path().join("libs").join("target.bin");
        fs::create_dir_all(path.parent().unwrap()).unwrap();

        // Seed with OLD content so readers have a valid baseline.
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
                        // Writer: overwrite with NEW content atomically, 50 times.
                        for _ in 0..50 {
                            atomic_write(&path, &new_payload).expect("atomic write should succeed");
                        }
                    } else {
                        // Reader: any content seen must be a complete, known value —
                        // no partial (mixed-byte) buffers.
                        for _ in 0..50 {
                            match fs::read(path.as_ref()) {
                                Ok(data) => {
                                    // Every byte in the read buffer must be the same value.
                                    // A uniform all-0xAA or all-0x55 buffer is a complete
                                    // OLD or NEW write; any other pattern is a torn write.
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
                                    // Acceptable: rename raced with read between exists() and open().
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

        // Plant an orphan tmp file — dot-prefixed, so `lang_from_lib_filename`
        // must skip it (strip_prefix("lib") leaves ".tree_sitter_python..." which
        // does NOT match "tree_sitter_" or "tree-sitter-" prefixes).
        let orphan = libs_dir.join(".libtree_sitter_python.dylib.tmp.99999.0");
        fs::write(&orphan, b"corrupt-partial").expect("orphan write should succeed");

        // installed_languages should not include the orphan.
        let installed = manager.installed_languages();
        assert!(
            !installed.contains(&"python".to_string()),
            "orphan tmp file must not register as a language; got: {installed:?}"
        );

        // A real extraction of python should still succeed and overwrite / create
        // the canonical library file.
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

        // Orphan is still present (we don't clean it up — it's harmless).
        // The canonical path must differ from the orphan path.
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
}
