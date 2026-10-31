"""Resolve the generator before entering an upstream repository's tool configuration."""

import asyncio
import io
import os
import sys
from pathlib import Path
from subprocess import CompletedProcess
from typing import Any

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import clone_vendors as vendor


class Output(io.StringIO):
    def reconfigure(self, **_kwargs: Any) -> None:
        pass


def test_all_generation_uses_one_project_root_resolution(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    root = tmp_path.resolve()
    binary = root / "canonical-tree-sitter"
    trace = root / "trace"
    binary.write_text(f'#!/bin/sh\nprintf "%s|%s\\n" "$PWD" "$*" >> "{trace}"\n')
    binary.chmod(0o755)
    shim = root / "tree-sitter"
    shim.write_text('#!/bin/sh\necho "vendor CLI 0.26.13 selected" >&2\nexit 99\n')
    shim.chmod(0o755)
    monkeypatch.setenv("PATH", f"{root}:{os.environ['PATH']}")
    definitions = {name: {"generate": True, "repo": "https://github.com/example/grammar"} for name in ("a", "b")}
    for name in definitions:
        directory = root / "vendor" / name
        directory.mkdir(parents=True)
        (directory / "mise.toml").write_text('[tools]\ntree-sitter="0.26.13"\n')
    resolved = []

    async def process(command: list[str], **kwargs: Any) -> CompletedProcess:
        assert command == ["mise", "which", "tree-sitter"]
        resolved.append(kwargs["cwd"])
        return CompletedProcess(command, 0, f"{binary}\n".encode(), b"")

    async def no_op(**_kwargs: Any) -> None:
        pass

    monkeypatch.setattr(vendor, "_project_root", root)
    monkeypatch.setattr(vendor, "vendor_directory", root / "vendor")
    monkeypatch.setattr(vendor, "parsers_directory", root / "parsers")
    monkeypatch.setattr(vendor, "run_process", process)
    monkeypatch.setattr(vendor, "which", lambda _name: None)
    monkeypatch.setattr(vendor, "get_language_definitions", lambda: (definitions, list(definitions)))
    monkeypatch.setattr(vendor, "_load_cache_manifest", dict)
    monkeypatch.setattr(vendor, "_is_language_cached", lambda *_args: False)
    monkeypatch.setattr(vendor, "_should_regenerate", lambda *_args, **_kwargs: True)
    monkeypatch.setattr(vendor, "_save_cache_manifest", lambda _manifest: None)
    monkeypatch.setattr(vendor, "clone_repository", no_op)
    monkeypatch.setattr(vendor, "move_src_folder", no_op)
    monkeypatch.setattr(sys, "stdout", Output())
    asyncio.run(vendor.main())
    assert resolved == [str(root)]
    assert sorted(trace.read_text().splitlines()) == [f"{root}/vendor/{name}|generate --abi 14" for name in ("a", "b")]


@pytest.mark.parametrize("output", [b"relative-tree-sitter\n", b"/missing-tree-sitter\n", b"", b"/one\n/two\n"])
def test_invalid_generator_resolution_fails_closed(monkeypatch: pytest.MonkeyPatch, output: bytes) -> None:
    async def process(command: list[str], **_kwargs: Any) -> CompletedProcess:
        return CompletedProcess(command, 0, output, b"")

    monkeypatch.setattr(vendor, "run_process", process)
    with pytest.raises(RuntimeError, match="canonical tree-sitter"):
        asyncio.run(vendor._resolve_tree_sitter_cli())


def test_failed_mise_resolution_never_uses_an_available_path_shim(monkeypatch: pytest.MonkeyPatch) -> None:
    async def process(command: list[str], **_kwargs: Any) -> CompletedProcess:
        return CompletedProcess(command, 1, b"", b"fixture missing tool")

    monkeypatch.setattr(vendor, "run_process", process)
    monkeypatch.setattr(vendor, "which", lambda _name: "/fake/shims/tree-sitter")
    with pytest.raises(RuntimeError, match="canonical tree-sitter"):
        asyncio.run(vendor._resolve_tree_sitter_cli())


def test_generation_failure_reports_a_bounded_stderr_tail(tmp_path: Path) -> None:
    command = [
        sys.executable,
        "-c",
        'import sys;sys.stderr.write("OMIT_PREFIX"+"x"*20000+"exact fixture failure");sys.exit(23)',
    ]
    with pytest.raises(RuntimeError, match="23") as caught:
        asyncio.run(vendor._run_generate_with_timeout(command, str(tmp_path)))
    message = str(caught.value)
    assert "exact fixture failure" in message
    assert "OMIT_PREFIX" not in message
    assert len(message) < 8500


def test_nonexecutable_generator_resolution_is_refused(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    binary = tmp_path / "tree-sitter"
    binary.write_text("not executable\n")
    binary.chmod(0o644)

    async def process(command: list[str], **_kwargs: Any) -> CompletedProcess:
        return CompletedProcess(command, 0, f"{binary}\n".encode(), b"")

    monkeypatch.setattr(vendor, "run_process", process)
    with pytest.raises(RuntimeError, match="absolute executable"):
        asyncio.run(vendor._resolve_tree_sitter_cli())


def test_windows_failure_preserves_only_the_bounded_stderr_tail(monkeypatch: pytest.MonkeyPatch) -> None:
    async def process(command: list[str], **_kwargs: Any) -> CompletedProcess:
        return CompletedProcess(command, 23, b"", b"OMIT_PREFIX" + b"x" * 20000 + b"exact fixture failure")

    monkeypatch.setattr(vendor.platform, "system", lambda: "Windows")
    monkeypatch.setattr(vendor, "run_process", process)
    with pytest.raises(RuntimeError, match="exact fixture failure") as caught:
        asyncio.run(vendor._run_generate_with_timeout(["fixture"], "/fixture"))
    assert "OMIT_PREFIX" not in str(caught.value)
    assert len(str(caught.value)) < 8500


def test_generation_cancellation_reaps_its_child(tmp_path: Path) -> None:
    pid_file = tmp_path / "pid"
    command = [
        sys.executable,
        "-c",
        "import os,sys,time;open(sys.argv[1],'w').write(str(os.getpid()));time.sleep(30)",
        str(pid_file),
    ]

    async def cancel() -> None:
        task = asyncio.create_task(vendor._run_generate_with_timeout(command, str(tmp_path)))
        for _ in range(200):
            if pid_file.exists():
                break
            await asyncio.sleep(0.01)
        assert pid_file.exists(), "fixture child did not start"
        task.cancel()
        with pytest.raises(asyncio.CancelledError):
            await task

    asyncio.run(cancel())
    with pytest.raises(ProcessLookupError):
        os.kill(int(pid_file.read_text()), 0)
