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

const GITHUB_RELEASE_BASE: &str = "https://github.com/kreuzberg-dev/tree-sitter-language-pack/releases/download";
const CACHE_REMOVE_RETRIES: usize = 5;
const CACHE_REMOVE_RETRY_DELAY: Duration = Duration::from_millis(10);
const HTTP_TIMEOUT: Duration = Duration::from_secs(60);
const TLS_ROOTS_ENV: &str = "TREE_SITTER_LANGUAGE_PACK_TLS_ROOTS";

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
        .build()
        .new_agent()
}

/// Manifest describing available parser downloads for a specific version.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserManifest {
    pub version: String,
    pub platforms: HashMap<String, PlatformBundle>,
    pub languages: HashMap<String, LanguageInfo>,
    pub groups: HashMap<String, Vec<String>>,
}

#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformBundle {
    pub url: String,
    pub sha256: String,
    pub size: u64,
}

#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageInfo {
    pub group: String,
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
    #[cfg_attr(alef, alef(skip))]
    pub fn ensure_languages(&self, names: &[&str]) -> Result<(), Error> {
        let missing: Vec<&str> = names.iter().filter(|name| !self.is_cached(name)).copied().collect();

        if missing.is_empty() {
            return Ok(());
        }

        // Fetch manifest if not already loaded
        {
            let mut guard = self.manifest.lock().unwrap();
            if guard.is_none() {
                *guard = Some(self.fetch_manifest_inner()?);
            }
        }

        let guard = self.manifest.lock().unwrap();
        let manifest = guard.as_ref().expect("manifest loaded above");

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
    #[cfg_attr(alef, alef(skip))]
    pub fn ensure_group(&self, group: &str) -> Result<(), Error> {
        {
            let mut guard = self.manifest.lock().unwrap();
            if guard.is_none() {
                *guard = Some(self.fetch_manifest_inner()?);
            }
        }

        let guard = self.manifest.lock().unwrap();
        let manifest = guard.as_ref().expect("manifest loaded above");
        let langs = manifest.groups.get(group).ok_or_else(|| {
            Error::Download(format!(
                "Group '{}' not found. Available: {:?}",
                group,
                manifest.groups.keys().collect::<Vec<_>>()
            ))
        })?;

        let lang_names: Vec<String> = langs.clone();
        drop(guard);
        let names: Vec<&str> = lang_names.iter().map(String::as_str).collect();
        self.ensure_languages(&names)
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
        self.fetch_manifest_inner()
    }

    /// Internal manifest fetcher (does not require &mut self).
    fn fetch_manifest_inner(&self) -> Result<ParserManifest, Error> {
        // Check for cached manifest first
        let manifest_path = self.cache_dir.parent().map(|p| p.join("manifest.json"));
        if let Some(ref path) = manifest_path
            && path.exists()
        {
            let data = fs::read_to_string(path)?;
            let manifest: ParserManifest = serde_json::from_str(&data)?;
            if manifest.version == self.version {
                return Ok(manifest);
            }
        }

        let url = format!("{}/v{}/parsers.json", GITHUB_RELEASE_BASE, self.version);

        let body = self
            .agent
            .get(&url)
            .call()
            .map_err(|e| Error::Download(format!("Failed to fetch manifest from {}: {}", url, e)))?
            .into_body()
            .read_to_string()
            .map_err(|e| Error::Download(format!("Failed to read manifest body: {}", e)))?;

        let manifest: ParserManifest = serde_json::from_str(&body)?;

        // Cache the manifest
        if let Some(ref path) = manifest_path {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, &body)?;
        }

        Ok(manifest)
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
    fn load_or_download_bundle(&self, platform_key: &str, bundle: &PlatformBundle) -> Result<Vec<u8>, Error> {
        let cache_path = self.bundle_cache_path(platform_key, &bundle.sha256)?;

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

        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(cache_path, &data)?;
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

    /// Extract specific languages from a zstd-compressed tar archive.
    fn extract_languages(&self, archive_data: &[u8], names: &[&str]) -> Result<(), Error> {
        fs::create_dir_all(&self.cache_dir)?;

        let decoder = zstd::Decoder::new(archive_data)
            .map_err(|e| Error::Download(format!("Failed to decompress archive: {}", e)))?;
        let mut archive = tar::Archive::new(decoder);

        // Build a set of expected filenames for the requested languages
        let expected_files: HashMap<String, &str> = names
            .iter()
            .map(|name| {
                let filename = self
                    .lib_path(name)
                    .file_name()
                    .expect("lib_path always has a filename")
                    .to_string_lossy()
                    .to_string();
                (filename, *name)
            })
            .collect();
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
                entry
                    .unpack(&dest)
                    .map_err(|e| Error::Download(format!("Failed to extract {}: {}", filename, e)))?;
                extracted_files.insert(filename);
            }
        }

        let mut missing_languages: Vec<&str> = expected_files
            .iter()
            .filter_map(|(filename, name)| {
                (!extracted_files.contains(filename) && !self.cache_dir.join(filename).exists()).then_some(*name)
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

    /// Download the platform bundle and extract every library file it contains.
    ///
    /// Unlike [`ensure_languages`], this does not check the manifest language list
    /// against archive contents — it simply extracts all `.so`/`.dylib`/`.dll` files
    /// from the bundle. Languages in the manifest that are missing from the archive
    /// are silently ignored rather than returning an error.
    ///
    /// Returns the number of library files extracted (including those already cached).
    pub fn download_all_best_effort(&self) -> Result<usize, Error> {
        {
            let mut guard = self.manifest.lock().unwrap();
            if guard.is_none() {
                *guard = Some(self.fetch_manifest_inner()?);
            }
        }

        let guard = self.manifest.lock().unwrap();
        let manifest = guard.as_ref().expect("manifest loaded above");
        let platform_key = Self::platform_key();
        let bundle = manifest.platforms.get(&platform_key).ok_or_else(|| {
            Error::Download(format!(
                "No pre-built parsers available for platform '{}'. Available: {:?}",
                platform_key,
                manifest.platforms.keys().collect::<Vec<_>>()
            ))
        })?;
        let bundle = bundle.clone();
        drop(guard);

        let archive_data = self.load_or_download_bundle(&platform_key, &bundle)?;
        self.extract_all_libs(&archive_data)
    }

    /// Extract every library file from a zstd-compressed tar archive into the cache directory.
    ///
    /// Files are matched by extension (`.so`, `.dylib`, `.dll`) — no per-language
    /// verification is performed. Returns the count of files now present in the cache dir.
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
                if !dest.exists() {
                    entry
                        .unpack(&dest)
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
    pub fn clean_cache(&self) -> Result<(), Error> {
        Self::remove_dir_if_exists(&self.cache_dir)?;
        let version_cache_dir = self.version_cache_dir()?;
        let bundle_dir = version_cache_dir.join("bundles");
        Self::remove_dir_if_exists(&bundle_dir)?;
        let manifest_path = version_cache_dir.join("manifest.json");
        Self::remove_file_if_exists(&manifest_path)?;
        Ok(())
    }

    fn remove_dir_if_exists(path: &Path) -> Result<(), Error> {
        for attempt in 0..=CACHE_REMOVE_RETRIES {
            match fs::remove_dir_all(path) {
                Ok(()) => return Ok(()),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
                Err(error)
                    if error.kind() == std::io::ErrorKind::DirectoryNotEmpty && attempt < CACHE_REMOVE_RETRIES =>
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
        let mut hex = String::with_capacity(hash.len() * 2);
        for byte in hash {
            use std::fmt::Write;
            write!(hex, "{byte:02x}").unwrap();
        }
        hex
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
}
