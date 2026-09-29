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

/// Pick the base directory that the parser cache is rooted in, in priority order.
///
/// Split from [`resolve_cache_base`] so every branch — including the one where
/// the platform reports no cache directory — is reachable from unit tests
/// without mutating process-global environment state. ~keep
fn cache_base_from(env_override: Option<&str>, system_cache_dir: Option<PathBuf>) -> Result<PathBuf, Error> {
    if let Some(dir) = env_override
        && !dir.trim().is_empty()
    {
        return Ok(PathBuf::from(dir));
    }
    if let Some(dir) = system_cache_dir {
        return Ok(dir);
    }
    // ~keep #176 used to degrade to `std::env::temp_dir()` here (Windows conda-forge
    // ~keep runners with no %LOCALAPPDATA%; Unix units/containers with no $HOME or
    // ~keep $XDG_CACHE_HOME report no cache dir either). `temp_dir()` is world-writable
    // ~keep on most platforms, and this cache is later searched for shared libraries this
    // ~keep process dlopens — falling back to it lets any local user plant a payload this
    // ~keep process will load and execute. Fail closed instead. See #101 H2. ~keep
    Err(Error::Download(format!(
        "Could not determine a cache directory: the platform reported none (no $HOME or \
         $XDG_CACHE_HOME on Unix, no %LOCALAPPDATA% on Windows). Set {CACHE_DIR_ENV} to an \
         explicit, private directory, or point HOME/XDG_CACHE_HOME (Unix) / LOCALAPPDATA \
         (Windows) at one."
    )))
}

/// Resolve the base directory for the parser cache.
///
/// Layered: the `TREE_SITTER_LANGUAGE_PACK_CACHE_DIR` env var (if set and
/// non-empty), else the platform cache directory. No longer falls back to the
/// temporary directory — see [`cache_base_from`].
fn resolve_cache_base() -> Result<PathBuf, Error> {
    cache_base_from(std::env::var(CACHE_DIR_ENV).ok().as_deref(), dirs::cache_dir())
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

/// Unix permission bits applied to every directory this crate creates directly:
/// owner read/write/execute only.
#[cfg(unix)]
const CACHE_DIR_MODE: u32 = 0o700;

/// Bitmask for the Unix "group can write" and "other can write" permission bits.
#[cfg(unix)]
const GROUP_OTHER_WRITABLE_MASK: u32 = 0o022;

// ~keep Declared directly via a raw `extern "C"` block rather than pulling in the
// ~keep `libc`/`nix` crate for a single already-linked libc syscall.
#[cfg(unix)]
unsafe extern "C" {
    fn getuid() -> u32;
}

/// Whether `owner_uid` is an owner this process trusts for a cache directory it
/// dlopens a shared library out of: its own uid, or root (uid 0).
///
/// Root can write anywhere and is already trusted, so accepting a root-owned
/// directory (a read-only container image whose grammar cache was baked at build
/// time) does not widen the threat model this check exists for — no *other* local
/// user can write to it. Every non-root, non-self uid is still refused, and the
/// group-/other-writable check below still applies. OpenSSH's `StrictModes`
/// accepts the same two owners for the same reason. See #202. ~keep
#[cfg(unix)]
fn is_trusted_cache_owner(owner_uid: u32, current_uid: u32) -> bool {
    owner_uid == current_uid || owner_uid == 0
}

/// Verify that an existing `path` is owned by the current process's uid or by root,
/// and is not group- or other-writable.
///
/// This is the check that closes #101 H2: every directory this crate ever dlopens
/// a shared library out of (registered as an `extra_lib_dir` in `registry.rs`), or
/// writes an unverified manifest/bundle into, must not be writable by any other
/// local user — otherwise another local user (or a process inheriting a shared,
/// insecure `TREE_SITTER_LANGUAGE_PACK_CACHE_DIR`) can plant a payload this process
/// will trust. Root ownership is accepted on top of self-ownership because root is
/// already trusted and a root-owned directory is unwritable by everyone else; see
/// [`is_trusted_cache_owner`]. See #202. ~keep
#[cfg(unix)]
fn verify_owner_and_perms(path: &Path) -> Result<(), Error> {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    let metadata = fs::metadata(path)
        .map_err(|e| Error::Download(format!("Failed to stat cache directory {}: {e}", path.display())))?;

    // SAFETY: getuid() takes no arguments, performs no I/O, and cannot fail.
    let current_uid = unsafe { getuid() };
    if !is_trusted_cache_owner(metadata.uid(), current_uid) {
        return Err(Error::Download(format!(
            "Refusing to use cache directory {} because it is owned by uid {}, not the current \
             process's uid {} or root. Set {CACHE_DIR_ENV} to a directory this process owns.",
            path.display(),
            metadata.uid(),
            current_uid
        )));
    }

    let mode = metadata.permissions().mode();
    if mode & GROUP_OTHER_WRITABLE_MASK != 0 {
        return Err(Error::Download(format!(
            "Refusing to use cache directory {} because it is group- or other-writable (mode {:o}). \
             Run `chmod 700 {}` or set {CACHE_DIR_ENV} to a private directory.",
            path.display(),
            mode & 0o777,
            path.display()
        )));
    }

    Ok(())
}

/// Create `path` (and any missing ancestors) as an owner-only (`0700`) directory,
/// or verify that a pre-existing `path` is still owner-only. See
/// [`verify_owner_and_perms`]. Used for every directory this crate creates under
/// the resolved cache base (`DownloadCacheLock::open`, `extract_languages`,
/// `extract_all_libs`).
#[cfg(unix)]
pub(crate) fn ensure_secure_cache_dir(path: &Path) -> Result<(), Error> {
    use std::os::unix::fs::DirBuilderExt;

    if !path.exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .mode(CACHE_DIR_MODE)
            .create(path)
            .map_err(|e| {
                Error::Download(format!(
                    "Failed to create cache directory {} with owner-only (0700) permissions: {e}",
                    path.display()
                ))
            })?;
    }
    verify_owner_and_perms(path)
}

/// `%LOCALAPPDATA%` (e.g. `C:\Users\<user>\AppData\Local`) is the environment
/// variable Windows populates with the current user's per-profile cache root,
/// and the value `dirs::cache_dir()` resolves to on this platform. Windows
/// creates this directory at profile creation with a DACL granting full
/// control only to the owning user, `SYSTEM`, and `Administrators` — no other
/// principal, including other standard users on the same machine, has access
/// by default. Referenced by [`ensure_secure_cache_dir`]'s `#[cfg(windows)]`
/// implementation. ~keep
#[cfg(windows)]
const WINDOWS_PROFILE_CACHE_ENV: &str = "LOCALAPPDATA";

/// Windows equivalent of Unix's [`ensure_secure_cache_dir`]/[`verify_owner_and_perms`]
/// pair: judgement call for #109, recorded here rather than left implicit.
///
/// Considered three options: (a) full DACL manipulation via `windows-sys`
/// (resolve the process token's owner SID, resolve `path`'s owner SID via
/// `GetNamedSecurityInfoW`, compare with `EqualSid`, walk the DACL for
/// non-owner write access via `GetEffectiveRightsFromAclW`, and refuse or
/// repair on mismatch — mirroring `verify_owner_and_perms`'s uid/mode checks
/// bit-for-bit); (b) rely on the fact that the default cache directory
/// already lives under `%LOCALAPPDATA%` (see [`WINDOWS_PROFILE_CACHE_ENV`]),
/// which Windows itself ACLs to the owning user by profile default, and skip
/// bespoke ACL code entirely; (c) detect-and-warn without attempting repair.
///
/// Went with (b), plus the detection half of (c) for the one gap (b) leaves
/// open. Reasoning:
/// - `windows-sys` is already resolved into this workspace's `Cargo.lock`
///   transitively — `fd-lock` (an existing, non-optional-once-`download`-is-on
///   dependency of this very module) depends on `windows-sys 0.59` for its
///   Windows `LockFileEx` backend — so option (a) would not add a *new*
///   third-party dependency to the graph, only new feature flags
///   (`Win32_Security`, `Win32_Security_Authorization`) and new unsafe FFI
///   surface in this crate.
/// - Despite that, (a) was rejected: this repo has no Windows host to compile
///   or run that FFI against, and both of its failure directions are worse
///   than the gap it would close — a subtly wrong SID/ACE comparison that
///   never refuses anything ships a false guarantee ("we harden Windows
///   ACLs") while protecting nothing, and a check that wrongly *does* refuse
///   hard-breaks every Windows user's cache. Either outcome is worse than an
///   honestly documented, narrower mitigation.
/// - `CreateDirectoryW` (what `fs::create_dir_all` calls) is passed no
///   explicit security descriptor here, so a newly created cache directory
///   under `%LOCALAPPDATA%` inherits that parent's restrictive DACL
///   unchanged — the real protection is Windows' own per-profile default,
///   not anything this function does.
/// - The one way a caller defeats that inherited protection is pointing
///   `TREE_SITTER_LANGUAGE_PACK_CACHE_DIR` outside the profile root (a shared
///   drive, `C:\Temp`, a redirected/roaming profile share). That's a plain
///   path-containment check against `%LOCALAPPDATA%` — no Win32 API needed —
///   so it's implemented as an advisory [`tracing::warn!`] via
///   [`warn_if_outside_profile_cache_root`], never a hard refusal (a false
///   positive here must not be able to hard-break startup, unlike Unix's
///   refusal which is backed by a real permission-bit read).
#[cfg(windows)]
pub(crate) fn ensure_secure_cache_dir(path: &Path) -> Result<(), Error> {
    fs::create_dir_all(path)
        .map_err(|e| Error::Download(format!("Failed to create cache directory {}: {e}", path.display())))?;
    warn_if_outside_profile_cache_root(path);
    Ok(())
}

/// Plain recursive creation for targets that are neither Unix nor Windows
/// (e.g. wasm32, where this crate's `download` feature has no real-filesystem
/// notion of a private cache directory to harden). Unchanged by #109.
#[cfg(not(any(unix, windows)))]
pub(crate) fn ensure_secure_cache_dir(path: &Path) -> Result<(), Error> {
    fs::create_dir_all(path)
        .map_err(|e| Error::Download(format!("Failed to create cache directory {}: {e}", path.display())))
}

/// Pure containment check backing [`warn_if_outside_profile_cache_root`].
///
/// Split out the same way [`cache_base_from`] is split from
/// [`resolve_cache_base`]: so the decision is reachable from unit tests
/// without mutating the process-global `%LOCALAPPDATA%` environment variable.
#[cfg(windows)]
fn is_outside_profile_cache_root(path: &Path, profile_root: &str) -> bool {
    !path.starts_with(profile_root)
}

/// Advisory-only companion to [`ensure_secure_cache_dir`]'s Windows path: warns
/// (never refuses) when `path` is not nested under the current user's
/// `%LOCALAPPDATA%`, the one location Windows guarantees is private to this
/// user by default. Silent when `%LOCALAPPDATA%` is unset or empty — nothing
/// to compare against, and this check must not be able to block startup.
#[cfg(windows)]
fn warn_if_outside_profile_cache_root(path: &Path) {
    let Ok(profile_root) = std::env::var(WINDOWS_PROFILE_CACHE_ENV) else {
        return;
    };
    if profile_root.trim().is_empty() {
        return;
    }
    if is_outside_profile_cache_root(path, &profile_root) {
        tracing::warn!(
            cache_dir = %path.display(),
            profile_root = %profile_root,
            "cache directory is outside the current user's {WINDOWS_PROFILE_CACHE_ENV} profile \
             directory, which Windows protects from other local users by default; verify this \
             location's permissions manually, or set {CACHE_DIR_ENV} to a directory under \
             {WINDOWS_PROFILE_CACHE_ENV}"
        );
    }
}

/// Verify a cache directory's ownership/permissions if it already exists; a
/// missing directory has nothing to verify (nothing here would be dlopened).
///
/// Used at dlopen-search-path registration time (`ensure_cache_registered` in
/// `lib.rs`), which must not have the side effect of creating a directory just
/// because a read-only query (`has_language`, `available_languages`, ...) ran.
#[cfg(unix)]
pub(crate) fn verify_cache_dir_if_present(path: &Path) -> Result<(), Error> {
    if !path.exists() {
        return Ok(());
    }
    verify_owner_and_perms(path)
}

/// Windows companion to the `#[cfg(unix)]` overload above: same advisory
/// check as [`ensure_secure_cache_dir`], run without creating anything. Never
/// returns an error — see [`warn_if_outside_profile_cache_root`].
#[cfg(windows)]
pub(crate) fn verify_cache_dir_if_present(path: &Path) -> Result<(), Error> {
    if path.exists() {
        warn_if_outside_profile_cache_root(path);
    }
    Ok(())
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn verify_cache_dir_if_present(_path: &Path) -> Result<(), Error> {
    Ok(())
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
        fs::create_dir_all(parent)
            .map_err(|e| Error::Download(format!("Failed to create directory {}: {e}", parent.display())))?;
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
        fs::create_dir_all(parent)
            .map_err(|e| Error::Download(format!("Failed to create directory {}: {e}", parent.display())))?;
    }
    let tmp = sibling_tmp_path(dest)?;
    let copy_result = (|| -> io::Result<()> {
        let f = File::create(&tmp)?;
        let mut writer = BufWriter::new(f);
        io::copy(src, &mut writer)?;
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
        // ~keep Propagate `ensure_secure_cache_dir`'s error as-is: it already names the
        // ~keep concrete reason (missing HOME/XDG_CACHE_HOME, wrong owner, group/other-writable).
        ensure_secure_cache_dir(cache_dir)?;
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
        // ~keep Unknown env values are not hard errors because PackConfig is the supported override path.
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
        // ~keep Honor proxy env vars; otherwise proxy-only networks hang until the global download timeout.
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
mod tests {
    // ~keep Test assertions legitimately use unwrap/expect; production code stays
    // ~keep covered by the crate-wide `unwrap_used`/`expect_used` deny in Cargo.toml.
    #![allow(clippy::unwrap_used, clippy::expect_used)]

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

        let error = result.expect_err(
            "a successful (no-op) extraction phase must not silently report 0 when the recount itself fails",
        );
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

        let cached = DownloadManager::load_verified_cached_bundle(&cache_path, &sha256)
            .expect("verified cache read should succeed");

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
}
