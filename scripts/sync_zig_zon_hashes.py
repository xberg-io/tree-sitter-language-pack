#!/usr/bin/env python3
"""Verify or regenerate the `.hash` fields in `test_apps/zig/build.zig.zon`.

A Zig package hash is `<name>-<version>-<digest>`, where the digest is computed over the fetched
tarball's contents. `alef sync-versions` repoints each `.url` at the release being cut, but a
content digest cannot be derived from a version string, so nothing rewrites the `.hash` and it
stays frozen at whatever release last set it by hand.

It was frozen at the 1.14.3 values from the 1.14.3 release through 1.15.7, so every `.url` in that
window named a tarball whose digest did not match the declared hash, and `zig build` in
`test_apps/zig` failed with:

    error: hash mismatch: manifest declares tree_sitter_language_pack-1.14.3-jCz0Y85sQQ...
    but the fetched package has tree_sitter_language_pack-1.15.7-jCz0Y86TSQ...

Nothing reported it because no workflow builds `test_apps/zig` -- `ci-zig` builds `packages/zig`
and `ci-e2e` runs `e2e/zig`, so the app that resolves these URLs is never exercised.

The only reproducible way to produce the value is to let Zig compute it: `zig fetch <url>` prints
the exact hash Zig will demand. Never edit a hash by hand and never rewrite the version substring
inside one -- that yields a well-formed hash that matches no artifact, turning a loud mismatch
into a silent lie.

Because the digest comes from the tarball, `--fix` only works once the release has published its
Zig assets. It runs *after* a publish, never during release prep. Verification distinguishes that
window explicitly: an asset that 404s is reported as not-yet-published and is not a failure.

`--require-published` inverts that tolerance for the one caller that knows the assets must exist:
the post-publish job in `publish.yaml`, which runs only after the upload succeeded. There a 404 is
not the release window, it is a missing artifact, and staying quiet about it would leave the
hashes stale for another release. Every other caller wants the default.

The same digests live in a second place: `[crates.e2e.registry.packages.zig.platform_hashes]` in
`alef.toml`, which is what alef substitutes when it generates a Zig manifest. Nothing populated
that table, so it carried five `STALE_HASH_REGENERATE` placeholders from the day it was written.
Alef reacts to a placeholder by omitting the `.hash` line entirely, which is the whole reason
`test_apps/zig/build.zig.zon` had to be declared `user_owned` -- the declaration exists to stop a
regeneration deleting hashes this script had just computed. Keeping both in step here is what
eventually lets that declaration come out.

Usage:
    python3 scripts/sync_zig_zon_hashes.py          # verify every hash against its tarball
    python3 scripts/sync_zig_zon_hashes.py --fix    # rewrite every hash from `zig fetch`
"""

from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ZON = ROOT / "test_apps" / "zig" / "build.zig.zon"
ALEF_TOML = ROOT / "alef.toml"

# ~keep The target triple is carried in the asset filename and nowhere else in the manifest, so
# it is the only thing that can attribute a computed digest to a `platform_hashes` key.
TRIPLE_FROM_URL = re.compile(r"-zig-v[0-9][0-9.]*-(?P<triple>[a-z0-9_]+-[a-z0-9_]+-[a-z0-9_-]+)\.tar\.gz$")

PLATFORM_HASHES_SECTION = re.compile(
    r"(?P<head>^\[crates\.e2e\.registry\.packages\.zig\.platform_hashes\]\n)(?P<body>(?:\"[^\"]+\" = \"[^\"]+\"\n)+)",
    re.MULTILINE,
)

# ~keep Captures the `.url`/`.hash` pair of one dependency. They are adjacent in every manifest
# alef generates, and pairing them positionally is what lets a hash be attributed to the tarball
# it actually guards rather than to a dependency name parsed separately.
DEPENDENCY = re.compile(r'\.url\s*=\s*"(?P<url>[^"]+)"\s*,\s*\.hash\s*=\s*"(?P<hash>[^"]+)"')

# ~keep `zig fetch` resolves a build root before it will run, even though fetching by URL needs
# nothing from it. A throwaway package satisfies that without touching the real one.
PROBE_BUILD_ZIG = 'const std = @import("std");\npub fn build(b: *std.Build) void {\n    _ = b;\n}\n'
PROBE_BUILD_ZON = """.{
    .name = .zig_hash_probe,
    .version = "0.0.0",
    .fingerprint = 0x9e6b1a2c4d8f3057,
    .minimum_zig_version = "0.16.0",
    .dependencies = .{},
    .paths = .{""},
}
"""

NOT_PUBLISHED_MARKERS = ("404", "not found", "Not Found")


class FetchError(RuntimeError):
    """`zig fetch` failed for a reason other than the asset not existing yet."""


class NotPublishedError(RuntimeError):
    """The release asset the URL names does not exist yet."""


def fetch_hash(url: str, probe: Path, cache: Path) -> str:
    """Return the package hash Zig computes for `url`.

    A dedicated global cache directory guarantees the digest is recomputed from a fresh download
    rather than replayed from a previously fetched entry -- a stale cache here would silently
    confirm whatever hash is already declared. ~keep
    """
    result = subprocess.run(
        ["zig", "fetch", "--global-cache-dir", str(cache), url],
        capture_output=True,
        text=True,
        cwd=probe,
        check=False,
    )
    if result.returncode != 0:
        stderr = result.stderr.strip()
        if any(marker in stderr for marker in NOT_PUBLISHED_MARKERS):
            raise NotPublishedError(stderr.splitlines()[-1] if stderr else "asset not found")
        raise FetchError(stderr or f"zig fetch exited {result.returncode}")
    return result.stdout.strip()


def alef_platform_hashes() -> dict[str, str]:
    """Return the `platform_hashes` table as it stands in `alef.toml`."""
    match = PLATFORM_HASHES_SECTION.search(ALEF_TOML.read_text(encoding="utf-8"))
    if match is None:
        return {}
    return dict(re.findall(r'"([^"]+)" = "([^"]+)"', match.group("body")))


def rewrite_alef_platform_hashes(computed: dict[str, str]) -> None:
    """Replace the `platform_hashes` table with `computed`, keeping the file's key order.

    Rewritten as a whole table rather than key by key: the placeholder value is identical across
    all five keys, so a value-keyed replace would rewrite every one of them with whichever digest
    happened to be substituted first. ~keep
    """
    text = ALEF_TOML.read_text(encoding="utf-8")
    match = PLATFORM_HASHES_SECTION.search(text)
    if match is None:
        return
    existing = dict(re.findall(r'"([^"]+)" = "([^"]+)"', match.group("body")))
    body = "".join(f'"{triple}" = "{computed.get(triple, value)}"\n' for triple, value in existing.items())
    ALEF_TOML.write_text(text[: match.start("body")] + body + text[match.end("body") :], encoding="utf-8")


def report_stale(stale: list[tuple[str, str, str]], alef_stale: list[tuple[str, str, str]]) -> int:
    """Print every mismatch and return the gate's exit code."""
    if stale:
        print(f"\n{ZON.relative_to(ROOT)}: {len(stale)} hash(es) do not match the tarball the URL names:\n")
        for asset, declared, actual in stale:
            print(f"  {asset}\n    declared: {declared}\n    actual:   {actual}")
    if alef_stale:
        print(f"\nalef.toml [crates.e2e.registry.packages.zig.platform_hashes]: {len(alef_stale)} stale:\n")
        for triple, declared, actual in alef_stale:
            print(f"  {triple}\n    declared: {declared}\n    actual:   {actual}")
    print(
        "\n`zig build` in test_apps/zig fails with a hash mismatch until these are regenerated.\n"
        "Run `python3 scripts/sync_zig_zon_hashes.py --fix` and commit. Never hand-edit a hash."
    )
    return 1


def write_fixes(
    text: str,
    replacements: dict[str, str],
    stale: list[tuple[str, str, str]],
    alef_stale: list[tuple[str, str, str]],
    computed_by_triple: dict[str, str],
) -> int:
    """Rewrite both files from the computed digests and report what moved."""
    for declared, actual in replacements.items():
        text = text.replace(f'"{declared}"', f'"{actual}"')
    ZON.write_text(text, encoding="utf-8")
    for asset, declared, actual in stale:
        print(f"fixed  {asset}\n         {declared}\n      -> {actual}")
    if alef_stale:
        rewrite_alef_platform_hashes(computed_by_triple)
        for triple, declared, actual in alef_stale:
            print(f"fixed  alef.toml [{triple}]\n         {declared}\n      -> {actual}")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--fix", action="store_true", help="rewrite each hash from `zig fetch` instead of reporting")
    parser.add_argument(
        "--require-published",
        action="store_true",
        help="treat a missing release asset as a failure instead of the normal pre-publish window",
    )
    args = parser.parse_args()

    if shutil.which("zig") is None:
        # ~keep Never pass silently on a missing toolchain: a hash gate that cannot compute a hash
        # has not verified anything, and reporting success here is how this drift stayed invisible.
        print("zig is not on PATH — cannot compute package hashes", file=sys.stderr)
        return 2

    text = ZON.read_text(encoding="utf-8")
    dependencies = list(DEPENDENCY.finditer(text))
    if not dependencies:
        print(f"{ZON}: no .url/.hash pairs found — the manifest format changed", file=sys.stderr)
        return 2

    stale: list[tuple[str, str, str]] = []
    pending: list[str] = []
    replacements: dict[str, str] = {}
    computed_by_triple: dict[str, str] = {}

    with tempfile.TemporaryDirectory() as scratch:
        probe = Path(scratch) / "probe"
        probe.mkdir()
        (probe / "build.zig").write_text(PROBE_BUILD_ZIG, encoding="utf-8")
        (probe / "build.zig.zon").write_text(PROBE_BUILD_ZON, encoding="utf-8")
        cache = Path(scratch) / "cache"

        for match in dependencies:
            url = match.group("url")
            declared = match.group("hash")
            asset = url.rsplit("/", 1)[-1]
            try:
                actual = fetch_hash(url, probe, cache)
            except NotPublishedError as exc:
                pending.append(f"{asset}: {exc}")
                continue
            except FetchError as exc:
                print(f"{asset}: {exc}", file=sys.stderr)
                return 2
            triple = TRIPLE_FROM_URL.search(url)
            if triple is not None:
                computed_by_triple[triple.group("triple")] = actual
            if actual != declared:
                stale.append((asset, declared, actual))
                replacements[declared] = actual

    if pending:
        # ~keep Between a version bump and the publish, the URLs name assets that do not exist.
        # That is the normal release window, not drift, so it must not fail the gate.
        print("not published yet (expected between a version bump and its release):")
        for entry in pending:
            print(f"  {entry}")
        if args.require_published:
            # ~keep The caller asserted the release assets exist, so a 404 is a missing artifact
            # rather than the release window. Writing the hashes that did resolve would produce a
            # half-refreshed manifest, so nothing is written.
            print(
                f"\n--require-published: the {len(pending)} asset(s) above should already exist.\n"
                "No hash was rewritten. Check that the release uploaded every Zig tarball.",
                file=sys.stderr,
            )
            return 2

    declared_in_alef = alef_platform_hashes()
    alef_stale = [
        (triple, declared_in_alef[triple], actual)
        for triple, actual in computed_by_triple.items()
        if triple in declared_in_alef and declared_in_alef[triple] != actual
    ]

    if not stale and not alef_stale:
        checked = len(dependencies) - len(pending)
        print(f"all {checked} zig package hashes match the tarballs their URLs name")
        return 0

    if args.fix:
        return write_fixes(text, replacements, stale, alef_stale, computed_by_triple)

    return report_stale(stale, alef_stale)


if __name__ == "__main__":
    sys.exit(main())
