"""A known-stale committed parser must never win over an explicit regeneration request."""

from __future__ import annotations

import asyncio
import sys
from pathlib import Path
from typing import Any

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import clone_vendors as vendor


@pytest.fixture
def oversized_parser(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> Path:
    monkeypatch.setattr(vendor, "vendor_directory", tmp_path)
    monkeypatch.setattr(vendor, "ABI_EXEMPT_PARSER_BYTES", 1)
    source = tmp_path / "abl/src"
    source.mkdir(parents=True)
    parser = source / "parser.c"
    parser.write_text("#define LANGUAGE_VERSION 15\n")
    return parser


def test_force_bypasses_only_the_large_parser_exemption(oversized_parser: Path) -> None:
    assert vendor._should_regenerate("abl", None) is False
    assert vendor._should_regenerate("abl", None, force=True) is True
    oversized_parser.write_text("#define LANGUAGE_VERSION 16\n")
    with pytest.raises(RuntimeError, match="outside the range"):
        vendor._should_regenerate("abl", None)


def test_force_flag_invalidates_the_cached_parser_identity() -> None:
    definition = {"repo": "https://github.com/example/grammar", "rev": "a" * 40, "generate": True}
    ordinary = vendor._language_cache_key(definition)
    assert ordinary == vendor._language_cache_key({**definition, "force_generate": False})
    forced = vendor._language_cache_key({**definition, "force_generate": True})
    assert forced != ordinary


def test_process_passes_force_to_selection_and_generation(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setattr(vendor, "vendor_directory", tmp_path)
    observed: dict[str, Any] = {}

    def select(*_args: Any, **kwargs: Any) -> bool:
        observed["selection"] = kwargs.get("force")
        return True

    async def generate(**kwargs: Any) -> None:
        observed["generation"] = kwargs.get("force")

    async def no_op(**_kwargs: Any) -> None:
        return None

    monkeypatch.setattr(vendor, "clone_repository", no_op)
    monkeypatch.setattr(vendor, "move_src_folder", no_op)
    monkeypatch.setattr(vendor, "_should_regenerate", select)
    monkeypatch.setattr(vendor, "handle_generate", generate)
    definition = {"repo": "https://github.com/example/grammar", "generate": True, "force_generate": True}
    asyncio.run(vendor.process_repo("abl", definition, asyncio.Semaphore(1)))
    assert observed == {"selection": True, "generation": True}


@pytest.mark.usefixtures("oversized_parser")
def test_forced_generation_timeout_cannot_ship_the_stale_parser(monkeypatch: pytest.MonkeyPatch) -> None:
    async def timeout(*_args: Any) -> None:
        raise TimeoutError

    async def no_op(*_args: Any, **_kwargs: Any) -> None:
        return None

    monkeypatch.setattr(vendor, "_run_generate_with_timeout", timeout)
    monkeypatch.setattr(vendor, "run_process", no_op)
    with pytest.raises(RuntimeError, match="forced regeneration"):
        asyncio.run(vendor.handle_generate("abl", None, 15, asyncio.Semaphore(1), force=True))


@pytest.mark.usefixtures("oversized_parser")
def test_ordinary_generation_retains_the_compatible_timeout_fallback(monkeypatch: pytest.MonkeyPatch) -> None:
    async def timeout(*_args: Any) -> None:
        raise TimeoutError

    async def no_op(*_args: Any, **_kwargs: Any) -> None:
        return None

    monkeypatch.setattr(vendor, "_run_generate_with_timeout", timeout)
    monkeypatch.setattr(vendor, "run_process", no_op)
    asyncio.run(vendor.handle_generate("abl", None, 15, asyncio.Semaphore(1)))
