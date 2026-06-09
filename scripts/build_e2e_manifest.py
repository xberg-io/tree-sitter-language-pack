# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///
"""Emit a `parsers.json` manifest pointing at a local tar.zst bundle.

Used by CI E2E to short-circuit the runtime's network fetch of the manifest
and the platform bundle (see `crates/ts-pack-core/src/download.rs`). The
manifest schema matches what `Publish Release` ships to GitHub Releases; the
only difference is the URL scheme — bundle URLs are `file://` instead of
`https://`, which the runtime resolves via the same code path.

The `TREE_SITTER_LANGUAGE_PACK_MANIFEST_URL` env var in the runtime points
test processes at this file, so download()/manifestLanguages() resolve
against the locally-built parsers instead of a not-yet-published release.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from pathlib import Path


def emit_manifest(
    *,
    version: str,
    platform_label: str,
    bundle_path: Path,
    definitions_path: Path,
    output_path: Path,
) -> None:
    """Write `parsers.json` for a single locally-built platform bundle.

    Hashes and sizes `bundle_path`, points the manifest's only platform entry
    at a `file://` URL for that bundle, and enumerates every language from
    `definitions_path` under the synthetic `"all"` group.
    """
    if not bundle_path.is_file():
        raise SystemExit(f"bundle missing: {bundle_path}")
    if not definitions_path.is_file():
        raise SystemExit(f"language definitions missing: {definitions_path}")

    bundle_bytes = bundle_path.read_bytes()
    sha256 = hashlib.sha256(bundle_bytes).hexdigest()
    size = len(bundle_bytes)
    bundle_url = f"file://{bundle_path.resolve()}"

    definitions = json.loads(definitions_path.read_text())
    languages = sorted(definitions.keys())

    manifest = {
        "version": version,
        "platforms": {
            platform_label: {
                "url": bundle_url,
                "sha256": sha256,
                "size": size,
            },
        },
        "languages": {name: {"group": "all", "size": 0} for name in languages},
        "groups": {"all": languages},
    }

    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(json.dumps(manifest, indent=2))
    print(
        f"emitted manifest: version={version} platform={platform_label} "
        f"languages={len(languages)} bundle={bundle_path.name} sha256={sha256[:12]}…",
        file=sys.stderr,
    )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--version", required=True, help="workspace version (without leading 'v')")
    parser.add_argument(
        "--platform-label",
        required=True,
        help="canonical platform key, e.g. linux-x86_64, macos-arm64",
    )
    parser.add_argument("--bundle", required=True, type=Path, help="path to parsers-<platform>.tar.zst")
    parser.add_argument(
        "--definitions",
        default=Path("sources/language_definitions.json"),
        type=Path,
        help="path to sources/language_definitions.json",
    )
    parser.add_argument("--output", required=True, type=Path, help="output parsers.json path")
    args = parser.parse_args()

    emit_manifest(
        version=args.version,
        platform_label=args.platform_label,
        bundle_path=args.bundle,
        definitions_path=args.definitions,
        output_path=args.output,
    )


if __name__ == "__main__":
    main()
