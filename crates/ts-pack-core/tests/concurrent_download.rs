//! Cross-process concurrency test for the download cache.
//!
//! # Worker-process pattern
//!
//! This file is both the orchestrating test and the worker implementation.
//! At the very top of every test entry point, we check for
//! `TSLP_CONCURRENT_DOWNLOAD_WORKER`. When set, we run the worker function and
//! return immediately; the child process exits with code 0 on success or 1 on
//! failure. This means spawning `std::env::current_exe()` with the worker env
//! var routes back into this same binary and executes the worker.
//!
//! The orchestrator test (`concurrent_processes_share_cache`) builds a fake
//! archive in a `TempDir`, then launches 8 child processes that all call
//! `DownloadManager::_testing_extract_languages` against the same shared cache
//! directory. After all 8 exit 0, it verifies the extracted file content
//! matches the expected bytes.
//!
//! # Start-gate
//!
//! The orchestrator creates a `.start_gate` sentinel file after spawning all
//! child processes. Each worker busy-waits (with a 5 ms sleep between polls)
//! until that file exists before extracting. This widens the race window to
//! approximate real-world contention.

// Environment variables used to communicate between orchestrator and worker.
const WORKER_ENV_VAR: &str = "TSLP_CONCURRENT_DOWNLOAD_WORKER";
const WORKER_CACHE_DIR_VAR: &str = "TSLP_WORKER_CACHE_DIR";
const WORKER_ARCHIVE_PATH_VAR: &str = "TSLP_WORKER_ARCHIVE_PATH";
const WORKER_GATE_PATH_VAR: &str = "TSLP_WORKER_GATE_PATH";

/// Worker entry point.  Called when `TSLP_CONCURRENT_DOWNLOAD_WORKER` is set.
/// Reads an archive from `TSLP_WORKER_ARCHIVE_PATH`, calls
/// `_testing_extract_languages` against the shared `TSLP_WORKER_CACHE_DIR`,
/// then exits.
fn run_worker() {
    let cache_dir = std::env::var(WORKER_CACHE_DIR_VAR).expect("TSLP_WORKER_CACHE_DIR must be set");
    let archive_path = std::env::var(WORKER_ARCHIVE_PATH_VAR).expect("TSLP_WORKER_ARCHIVE_PATH must be set");
    let gate_path = std::env::var(WORKER_GATE_PATH_VAR).expect("TSLP_WORKER_GATE_PATH must be set");

    // Busy-wait on the start-gate so all 8 workers begin extracting at once,
    // maximizing the cross-process race window.
    while !std::path::Path::new(&gate_path).exists() {
        std::thread::sleep(std::time::Duration::from_millis(5));
    }

    let archive_data = std::fs::read(&archive_path).expect("worker should read archive");

    // Use a fixed version string; the cache dir is fully controlled by the orchestrator.
    let dm = tree_sitter_language_pack::download::DownloadManager::with_cache_dir(
        "concurrent-test",
        std::path::PathBuf::from(cache_dir),
    );

    match dm._testing_extract_languages(&archive_data, &["python"]) {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("worker _testing_extract_languages failed: {e}");
            std::process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers shared with the test
// ---------------------------------------------------------------------------

fn compressed_tar_for_manager(
    manager: &tree_sitter_language_pack::download::DownloadManager,
    lang: &str,
    content: &[u8],
) -> Vec<u8> {
    let filename = manager
        .lib_path(lang)
        .file_name()
        .expect("lib_path has filename")
        .to_string_lossy()
        .into_owned();

    let encoder = zstd::Encoder::new(Vec::new(), 0).expect("zstd encoder should initialize");
    let mut builder = tar::Builder::new(encoder);

    let mut header = tar::Header::new_gnu();
    header.set_size(content.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    builder
        .append_data(&mut header, &filename, content)
        .expect("tar entry should append");

    let encoder = builder.into_inner().expect("tar builder should finish");
    encoder.finish().expect("zstd encoder should finish")
}

// ---------------------------------------------------------------------------
// Integration test: cross-process concurrency
// ---------------------------------------------------------------------------

#[test]
fn concurrent_processes_share_cache() {
    // Worker-process gate: if this env var is set, this invocation is a worker.
    if std::env::var(WORKER_ENV_VAR).is_ok() {
        run_worker();
        return;
    }

    use std::path::PathBuf;

    let temp_dir = tempfile::Builder::new()
        .prefix("tslp-concurrent-")
        .tempdir()
        .expect("temp dir should be created");

    let cache_dir: PathBuf = temp_dir.path().join("libs");
    let manager =
        tree_sitter_language_pack::download::DownloadManager::with_cache_dir("concurrent-test", cache_dir.clone());

    let expected: &[u8] = b"cross-process-safe-library-bytes";
    let archive = compressed_tar_for_manager(&manager, "python", expected);

    // Write the archive to a file so workers can read it.
    let archive_path = temp_dir.path().join("test-archive.tar.zst");
    std::fs::write(&archive_path, &archive).expect("archive write should succeed");

    // Start-gate sentinel: created AFTER all workers are spawned so they all
    // begin extracting at approximately the same time.
    let gate_path = temp_dir.path().join(".start_gate");

    // Current test binary path — workers re-invoke the same binary with the
    // worker env var so the worker code in run_worker() is executed.
    let current_exe = std::env::current_exe().expect("current_exe should be resolvable");

    // Spawn 8 worker processes.  Each calls _testing_extract_languages against
    // the shared cache_dir.  The cross-process file lock (DownloadCacheLock)
    // must serialise the mutations so no worker sees a torn write.
    //
    // Workers invoke the same test binary with TSLP_CONCURRENT_DOWNLOAD_WORKER
    // set; run_worker() detects the var, waits on the gate, performs the
    // extraction, and calls std::process::exit(0) before the test harness
    // dispatches any #[test]. We suppress worker stdout/stdin to keep test
    // output clean; stderr is inherited so worker errors are visible in CI
    // failure output.
    let children: Vec<_> = (0..8)
        .map(|_| {
            std::process::Command::new(&current_exe)
                .env(WORKER_ENV_VAR, "1")
                .env(WORKER_CACHE_DIR_VAR, &cache_dir)
                .env(WORKER_ARCHIVE_PATH_VAR, &archive_path)
                .env(WORKER_GATE_PATH_VAR, &gate_path)
                .stdout(std::process::Stdio::null())
                .stdin(std::process::Stdio::null())
                .spawn()
                .expect("worker process should spawn")
        })
        .collect();

    // Open the start-gate: all workers begin extracting from this point.
    std::fs::write(&gate_path, b"go").expect("start gate should be written");

    let mut all_ok = true;
    for (i, mut child) in children.into_iter().enumerate() {
        let status = child.wait().expect("worker should exit");
        if !status.success() {
            eprintln!("worker {i} exited with status {status}");
            all_ok = false;
        }
    }
    assert!(all_ok, "all 8 worker processes must exit 0");

    // Verify the extracted file matches the expected bytes — no torn writes.
    let extracted = std::fs::read(manager.lib_path("python")).expect("extracted library should exist");
    assert_eq!(
        extracted,
        expected,
        "extracted file must contain the complete expected bytes; got {} bytes",
        extracted.len()
    );

    // The lock file must still exist — clean_cache must not have deleted it.
    // (Here clean_cache was never called, so this is a sanity check that the
    // lock infrastructure left the cache dir intact.)
    assert!(cache_dir.exists(), "cache dir must exist after extraction");
}
