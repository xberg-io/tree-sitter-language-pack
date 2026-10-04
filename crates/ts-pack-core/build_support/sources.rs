use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Workspace-relative path of the helper that clones upstream grammar sources.
const CLONE_VENDORS_SCRIPT: &str = "scripts/clone_vendors.py";

/// Env var naming an explicit interpreter for [`CLONE_VENDORS_SCRIPT`].
const CLONE_VENDORS_INTERPRETER_ENV: &str = "TSLP_PYTHON";

/// Interpreter probe list used when [`CLONE_VENDORS_INTERPRETER_ENV`] is unset.
const DEFAULT_CLONE_VENDORS_RUNNERS: &[&[&str]] =
    &[&["uv", "run", "--no-sync"], &["uv", "run"], &["python3"], &["python"]];

/// Probe whether the parsers tree at `root` looks populated for the requested
/// selection. Uses the same heuristic in both the workspace and OUT_DIR cache
/// locations.
fn parsers_root_populated(root: &Path, selected: &[String]) -> bool {
    match selected.first() {
        Some(first) => root.join(first).join("src/parser.c").exists(),
        None => root.join("python").join("src/parser.c").exists(),
    }
}

/// Try to populate the workspace `parsers/` tree by invoking
/// `scripts/clone_vendors.py` from `project_root`. Returns true if the script
/// ran successfully AND the tree is populated afterwards. Returns false if no
/// script is present, no runner is available, or the script failed.
///
/// This is the local-development fallback: a fresh workspace clone has
/// `parsers/` gitignored, and a rc whose release hasn't been published yet has
/// no GH tarball — so neither the workspace nor the remote path works. Cloning
/// upstream grammars via the script bridges the gap.
fn try_clone_vendors_locally(project_root: &Path, parsers_dir: &Path, selected: &[String]) -> bool {
    let clone_script = project_root.join(CLONE_VENDORS_SCRIPT);
    if !clone_script.exists() {
        return false;
    }

    println!("cargo:rerun-if-env-changed={CLONE_VENDORS_INTERPRETER_ENV}");
    println!("cargo:warning=parsers/ tree is empty; running {CLONE_VENDORS_SCRIPT} to populate from upstream grammars");

    let runners = clone_vendors_runners();
    let mut spawned_any = false;

    for cmd_args in &runners {
        let mut cmd = std::process::Command::new(&cmd_args[0]);
        cmd.args(&cmd_args[1..]);
        cmd.current_dir(project_root);
        match cmd.status() {
            Ok(status) if status.success() => {
                if parsers_root_populated(parsers_dir, selected) {
                    return true;
                }
                println!(
                    "cargo:warning={} succeeded but parsers/ is still not populated",
                    cmd_args.join(" ")
                );
                return false;
            }
            Ok(status) => {
                spawned_any = true;
                println!("cargo:warning={} exited with {:?}", cmd_args.join(" "), status.code());
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                println!("cargo:warning=interpreter '{}' not found on PATH", cmd_args[0]);
            }
            Err(e) => {
                spawned_any = true;
                println!("cargo:warning=could not run {}: {e}", cmd_args.join(" "));
            }
        }
    }

    if !spawned_any {
        report_no_interpreter(&runners);
    }
    false
}

/// Build the ordered list of argv vectors used to run [`CLONE_VENDORS_SCRIPT`].
///
/// A non-empty [`CLONE_VENDORS_INTERPRETER_ENV`] replaces the probe list entirely
/// so hermetic builds (nix, containers, vendored toolchains) do not depend on
/// `uv` or `python3` happening to be on `PATH`.
fn clone_vendors_runners() -> Vec<Vec<String>> {
    let script = CLONE_VENDORS_SCRIPT.to_string();

    if let Ok(interpreter) = env::var(CLONE_VENDORS_INTERPRETER_ENV)
        && !interpreter.trim().is_empty()
    {
        return vec![vec![interpreter.trim().to_string(), script]];
    }

    DEFAULT_CLONE_VENDORS_RUNNERS
        .iter()
        .map(|base| base.iter().map(|a| (*a).to_string()).chain([script.clone()]).collect())
        .collect()
}

/// Emit an actionable diagnostic when no candidate interpreter could be spawned.
fn report_no_interpreter(runners: &[Vec<String>]) {
    let mut tried: Vec<&str> = Vec::new();
    for runner in runners {
        let name = runner[0].as_str();
        if !tried.contains(&name) {
            tried.push(name);
        }
    }
    println!(
        "cargo:warning=No usable Python interpreter was found to run {CLONE_VENDORS_SCRIPT} (tried: {}). \
         Install `uv` or `python3`, or set {CLONE_VENDORS_INTERPRETER_ENV} to an interpreter path. \
         Falling back to the released parser-source bundle; set TSLP_OFFLINE=1 to skip parser sources entirely.",
        tried.join(", ")
    );
}

/// Resolve grammar sources for the build. Order of preference:
/// 1. Workspace `parsers/` tree is already populated → use it as-is.
/// 2. `TSLP_OFFLINE=1` → return the empty workspace dir; downstream code falls
///    into the "missing parser, skipping" warning path.
/// 3. OUT_DIR cache already populated from a prior run → reuse it.
/// 4. Workspace tree exists (`scripts/clone_vendors.py` is present) → invoke
///    it to clone upstream grammars locally. Bridges the gap for fresh
///    workspace clones and unpublished rc builds where the remote release
///    tarball doesn't exist yet.
/// 5. Otherwise (sdist install or step 4 failed) → download the
///    `parser-sources-{version}.tar.zst` release asset and unpack it under
///    OUT_DIR. `TSLP_SOURCE_BUNDLE_URL` overrides the URL (also accepts
///    `file://` for local-bundle smoke testing).
///
/// Returns the directory to use as the parsers root.
pub(crate) fn ensure_parser_sources(parsers_dir: &Path, selected: &[String], out_dir: &Path) -> PathBuf {
    if parsers_root_populated(parsers_dir, selected) {
        return parsers_dir.to_path_buf();
    }
    if env::var("TSLP_OFFLINE").is_ok_and(|v| !v.is_empty() && v != "0") {
        println!("cargo:warning=TSLP_OFFLINE set; refusing to download parser sources");
        return parsers_dir.to_path_buf();
    }

    let cache_dir = out_dir.join("_parsers");
    // ~keep Same two-probe strategy for the OUT_DIR parser-source cache.
    let cache_populated = match selected.first() {
        Some(first) => cache_dir.join(first).join("src/parser.c").exists(),
        None => cache_dir.join("parsers").join("python").join("src/parser.c").exists(),
    };
    if cache_populated {
        let inner = cache_dir.join("parsers");
        return if inner.is_dir() { inner } else { cache_dir };
    }

    // ~keep Dev checkouts clone upstream grammars before falling back to release tarballs.
    // ~keep This covers fresh workspaces and unpublished rc builds.
    let project_root = parsers_dir.parent().unwrap_or(parsers_dir).to_path_buf();
    println!("cargo:rerun-if-env-changed=TSLP_OFFLINE");
    println!("cargo:rerun-if-env-changed=TSLP_SOURCE_BUNDLE_URL");
    if try_clone_vendors_locally(&project_root, parsers_dir, selected) {
        return parsers_dir.to_path_buf();
    }

    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());
    let default_url = format!(
        "https://github.com/xberg-io/tree-sitter-language-pack/releases/download/v{version}/parser-sources-{version}.tar.zst"
    );
    let url = env::var("TSLP_SOURCE_BUNDLE_URL").unwrap_or(default_url);
    let sha_url = format!("{url}.sha256");

    println!("cargo:warning=Downloading parser sources from {url}");

    let body = fetch_bytes(&url).unwrap_or_else(|e| {
        panic!(
            "Failed to download parser sources from {url}: {e}. Set TSLP_OFFLINE=1 to skip, or ensure network access and that v{version} is published with a parser-sources-{version}.tar.zst asset."
        )
    });
    // ~keep The bytes fetched above become C that is compiled into this binary, so a
    // missing or malformed checksum sidecar is a hard error rather than a skipped
    // check. TSLP_OFFLINE=1 is the existing escape hatch for "do not download at all".
    let sha_text = fetch_text(&sha_url).unwrap_or_else(|e| {
        panic!(
            "Failed to download the SHA-256 sidecar {sha_url}: {e}. The parser-source bundle is compiled into this crate, so it is refused without an integrity check. Set TSLP_OFFLINE=1 to skip downloading parser sources entirely, or point TSLP_SOURCE_BUNDLE_URL at a bundle that has a .sha256 sidecar next to it."
        )
    });
    let expected = sha_text.split_whitespace().next().unwrap_or_default().to_lowercase();
    assert!(
        expected.len() == 64 && expected.bytes().all(|b| b.is_ascii_hexdigit()),
        "Malformed SHA-256 sidecar at {sha_url}: expected a 64-character hex digest, got {expected:?}. Refusing to compile unverified parser sources; set TSLP_OFFLINE=1 to skip the download entirely."
    );
    let digest = Sha256::digest(&body);
    let actual: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(
        expected, actual,
        "SHA-256 mismatch for {url}: expected {expected}, got {actual}"
    );

    fs::create_dir_all(&cache_dir).expect("create OUT_DIR/_parsers");
    let cursor = std::io::Cursor::new(body);
    let decoder = zstd::stream::read::Decoder::with_buffer(cursor).expect("zstd decoder");
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(&cache_dir).expect("extract parser-sources tarball");

    // ~keep Release tarballs contain top-level `parsers/` and `sources/`, nested under OUT_DIR extraction.
    let inner = cache_dir.join("parsers");
    if inner.is_dir() { inner } else { cache_dir }
}

fn fetch_bytes(url: &str) -> Result<Vec<u8>, String> {
    if let Some(path) = url.strip_prefix("file://") {
        return fs::read(path).map_err(|e| format!("read {path}: {e}"));
    }

    // ~keep Retry transport errors, including intermittent GitHub release CDN 504s during publish verification.
    let max_attempts = 6u32;
    let mut last_err = String::new();
    for attempt in 1..=max_attempts {
        let call_result = ureq::get(url)
            .config()
            .timeout_global(Some(Duration::from_secs(300)))
            .build()
            .call();

        match call_result {
            Ok(response) => {
                let mut reader = response.into_body().into_reader();
                let mut buf = Vec::new();
                match std::io::Read::read_to_end(&mut reader, &mut buf) {
                    Ok(_) => return Ok(buf),
                    Err(e) => last_err = format!("read body for {url}: {e}"),
                }
            }
            Err(e) => last_err = format!("GET {url}: {e}"),
        }

        if attempt < max_attempts {
            let backoff = Duration::from_secs(2u64.saturating_pow(attempt));
            println!(
                "cargo:warning={last_err}; retrying in {}s (attempt {}/{})",
                backoff.as_secs(),
                attempt + 1,
                max_attempts
            );
            std::thread::sleep(backoff);
        }
    }
    Err(last_err)
}

fn fetch_text(url: &str) -> Result<String, String> {
    let bytes = fetch_bytes(url)?;
    String::from_utf8(bytes).map_err(|e| format!("decode utf8: {e}"))
}
