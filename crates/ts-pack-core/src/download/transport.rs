use std::fs;

use super::{GITHUB_RELEASE_BASE, HTTP_TIMEOUT, MANIFEST_URL_ENV, TLS_ROOTS_ENV};
use crate::error::Error;
use crate::pack_config::TlsRootsMode;

/// Resolve the URL used to fetch the parser manifest.
///
/// Layered override: `TREE_SITTER_LANGUAGE_PACK_MANIFEST_URL` env var (if set
/// and non-empty) wins over the compile-time GitHub release URL. Allows tests,
/// air-gapped deployments, and private mirrors to redirect manifest fetches
/// without recompiling. Supports both `http(s)://` (over the ureq agent) and
/// `file://` (read straight from disk) schemes.
pub(super) fn resolve_manifest_url(version: &str) -> String {
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
pub(super) fn read_file_url(url: &str) -> Result<String, Error> {
    let path = url
        .strip_prefix("file://")
        .ok_or_else(|| Error::Download(format!("not a file:// URL: {url}")))?;
    fs::read_to_string(path).map_err(|e| Error::Download(format!("Failed to read manifest from {url}: {e}")))
}

/// Resolve which CA root set the downloader's TLS client should trust.
///
/// Layered override: `tls_roots` struct field (if `Some`) > `TREE_SITTER_LANGUAGE_PACK_TLS_ROOTS`
/// environment variable (`platform` or `webpki`, case-insensitive) > compile-time default.
pub(super) fn resolve_tls_roots(override_mode: Option<TlsRootsMode>) -> TlsRootsMode {
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
pub(super) fn build_agent(mode: TlsRootsMode) -> ureq::Agent {
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
