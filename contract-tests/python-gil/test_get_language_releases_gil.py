# ruff: noqa: S101
"""Guards against #203/#206: get_language()/get_parser() must not hold the GIL during a download.

Not alef-generated. alef.toml's `release_gil = true` skips functions that return a capsule, so a
regeneration can silently drop the `py.detach` around the core call. This test points the
manifest at a local server that stalls, then checks that another Python thread keeps running
while the call is blocked on it. ~keep

The probed name is in no wheel's static grammar table: CI builds the wheel with every grammar
compiled in, so a real name such as "rust" resolves statically, never downloads, and the test
would measure nothing. ~keep
"""

import json
import os
import subprocess
import sys
import textwrap
from collections.abc import Iterator
from pathlib import Path

import pytest

STALL_SECONDS = 2.0
UNBUNDLED_LANGUAGE = "tslp-gil-probe-unbundled"
MAX_TICK_GAP_SECONDS = 0.5

SERVER = textwrap.dedent(
    f"""
    import http.server, sys, time

    class Stall(http.server.BaseHTTPRequestHandler):
        def do_GET(self):
            time.sleep({STALL_SECONDS})
            self.send_error(503)
        def log_message(self, *args):
            pass

    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Stall)
    print(server.server_address[1], flush=True)
    server.serve_forever()
    """
)

PROBE = textwrap.dedent(
    """
    import json, sys, threading, time
    import tree_sitter_language_pack as tslp

    call, name = sys.argv[1], sys.argv[2]
    stop = threading.Event()
    gaps = []

    def tick():
        last = time.monotonic()
        while not stop.is_set():
            time.sleep(0.002)
            now = time.monotonic()
            gaps.append(now - last)
            last = now

    ticker = threading.Thread(target=tick)
    ticker.start()
    time.sleep(0.05)
    started = time.monotonic()
    error = None
    try:
        getattr(tslp, call)(name)
    except Exception as exc:
        error = repr(exc)
    elapsed = time.monotonic() - started
    stop.set()
    ticker.join()
    print(json.dumps({"elapsed": elapsed, "max_gap": max(gaps), "error": error}))
    """
)


@pytest.fixture
def stalled_manifest_url() -> Iterator[str]:
    """Serve a manifest endpoint that stalls every request for STALL_SECONDS."""
    server = subprocess.Popen([sys.executable, "-c", SERVER], stdout=subprocess.PIPE, text=True)
    try:
        port = int(server.stdout.readline())
        yield f"http://127.0.0.1:{port}/parsers.json"
    finally:
        server.kill()
        server.wait()


@pytest.mark.parametrize("call", ["get_language", "get_parser"])
def test_download_does_not_hold_the_gil(call: str, stalled_manifest_url: str, tmp_path: Path) -> None:
    """Another thread keeps ticking while the call waits on the download."""
    env = {
        **os.environ,
        "TREE_SITTER_LANGUAGE_PACK_MANIFEST_URL": stalled_manifest_url,
        "TREE_SITTER_LANGUAGE_PACK_CACHE_DIR": str(tmp_path),
    }
    out = subprocess.run(
        [sys.executable, "-c", PROBE, call, UNBUNDLED_LANGUAGE],
        env=env,
        capture_output=True,
        text=True,
        timeout=60,
        check=True,
    )
    result = json.loads(out.stdout.splitlines()[-1])
    # The call must really have waited on the stalled server, or the gap check proves nothing.
    assert result["elapsed"] >= STALL_SECONDS * 0.9, result
    assert result["max_gap"] < MAX_TICK_GAP_SECONDS, result
