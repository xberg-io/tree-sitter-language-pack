use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use fd_lock::RwLock as FdRwLock;

use super::{CACHE_DIR_ENV, LOCK_FILE_NAME};
use crate::error::Error;

/// Monotonically increasing counter that makes sibling-tmp paths unique within
/// the current process even when `SystemTime::now()` returns the same value for
/// two threads (possible on systems where the wall clock has coarse resolution).
static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

/// Pick the base directory that the parser cache is rooted in, in priority order.
///
/// Split from [`resolve_cache_base`] so every branch — including the one where
/// the platform reports no cache directory — is reachable from unit tests
/// without mutating process-global environment state. ~keep
pub(super) fn cache_base_from(env_override: Option<&str>, system_cache_dir: Option<PathBuf>) -> Result<PathBuf, Error> {
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
pub(super) fn resolve_cache_base() -> Result<PathBuf, Error> {
    cache_base_from(std::env::var(CACHE_DIR_ENV).ok().as_deref(), dirs::cache_dir())
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
    pub(super) fn getuid() -> u32;
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
pub(super) fn is_trusted_cache_owner(owner_uid: u32, current_uid: u32) -> bool {
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
pub(super) fn is_outside_profile_cache_root(path: &Path, profile_root: &str) -> bool {
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
            target: "tree_sitter_language_pack::download",
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
pub(super) fn atomic_write(dest: &Path, data: &[u8]) -> Result<(), Error> {
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
pub(super) fn atomic_copy_from_reader<R: Read>(dest: &Path, src: &mut R) -> Result<(), Error> {
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
