import assert from "node:assert/strict";
import crypto from "node:crypto";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { test } from "node:test";

import { archiveName, assetUrl, expectedDigest, main, packageVersion, verifyArchive } from "./install.js";

const sha = (buf) => crypto.createHash("sha256").update(buf).digest("hex");

test("archiveName matches the release asset naming", () => {
  assert.equal(archiveName("aarch64-apple-darwin"), "ts-pack-aarch64-apple-darwin.tar.gz");
  assert.equal(archiveName("x86_64-unknown-linux-gnu"), "ts-pack-x86_64-unknown-linux-gnu.tar.gz");
  assert.equal(archiveName("x86_64-pc-windows-msvc"), "ts-pack-x86_64-pc-windows-msvc.zip");
});

test("assetUrl addresses the exact tag, never latest", () => {
  const url = assetUrl("1.28.0-rc.1", "ts-pack-aarch64-apple-darwin.tar.gz");
  assert.equal(
    url,
    "https://github.com/xberg-io/tree-sitter-language-pack/releases/download/v1.28.0-rc.1/ts-pack-aarch64-apple-darwin.tar.gz",
  );
  assert.ok(!url.includes("latest"));
});

test("expectedDigest reads sha256sum text and binary-mode markers", () => {
  const text = `${"a".repeat(64)}  one.tar.gz\n${"B".repeat(64)} *two.zip\n`;
  assert.equal(expectedDigest(text, "one.tar.gz"), "a".repeat(64));
  assert.equal(expectedDigest(text, "two.zip"), "b".repeat(64));
  assert.equal(expectedDigest(text, "three.zip"), null);
});

test("verifyArchive accepts a matching digest", () => {
  const buf = Buffer.from("payload");
  verifyArchive(buf, "a.tar.gz", `${sha(buf)}  a.tar.gz\n`);
});

test("verifyArchive rejects a digest mismatch", () => {
  const buf = Buffer.from("payload");
  assert.throws(() => verifyArchive(buf, "a.tar.gz", `${sha(Buffer.from("other"))}  a.tar.gz\n`), /checksum mismatch/);
});

test("verifyArchive rejects an archive missing from the sums file", () => {
  const buf = Buffer.from("payload");
  assert.throws(() => verifyArchive(buf, "a.tar.gz", `${sha(buf)}  b.tar.gz\n`), /no checksum entry/);
});

test("verifyArchive fails closed without a sums file unless explicitly allowed", () => {
  const buf = Buffer.from("payload");
  assert.throws(() => verifyArchive(buf, "a.tar.gz", null), /refusing to install an unverified binary/);
  verifyArchive(buf, "a.tar.gz", null, { allowUnverified: true });
});

function makeFakeRelease(version) {
  const work = fs.mkdtempSync(path.join(os.tmpdir(), "ts-pack-test-"));
  const isWin = os.type() === "Windows_NT";
  const name = isWin ? "ts-pack.exe" : "ts-pack";
  fs.writeFileSync(path.join(work, name), "#!/bin/sh\necho fake\n");
  const triples = { Darwin: "apple-darwin", Linux: "unknown-linux-gnu", Windows_NT: "pc-windows-msvc" };
  const arch = os.arch() === "arm64" ? "aarch64" : "x86_64";
  const archive = path.join(work, `ts-pack-${arch}-${triples[os.type()]}.${isWin ? "zip" : "tar.gz"}`);
  if (isWin) return null;
  spawnSync("tar", ["-czf", archive, "-C", work, name], { stdio: "inherit" });
  return { work, archive, asset: path.basename(archive), version };
}

test("main downloads exactly the package version and installs after a good checksum", async (t) => {
  const release = makeFakeRelease(packageVersion());
  if (!release) return t.skip("tar fixture is not built on Windows");
  const binDir = fs.mkdtempSync(path.join(os.tmpdir(), "ts-pack-bin-"));
  const archiveBytes = fs.readFileSync(release.archive);
  const requested = [];
  const fetchBuffer = (url) => {
    requested.push(url);
    if (url.endsWith("/SHA256SUMS")) return Promise.resolve(Buffer.from(`${sha(archiveBytes)}  ${release.asset}\n`));
    if (url.endsWith(release.asset)) return Promise.resolve(archiveBytes);
    return Promise.reject(new Error(`HTTP 404 for ${url}`));
  };
  await main({ fetchBuffer, binDir, version: "9.8.7" });
  assert.deepEqual(requested, [assetUrl("9.8.7", release.asset), assetUrl("9.8.7", "SHA256SUMS")]);
  assert.ok(fs.existsSync(path.join(binDir, "ts-pack")));
});

test("main refuses to install when the checksum does not match", async (t) => {
  const release = makeFakeRelease("9.8.7");
  if (!release) return t.skip("tar fixture is not built on Windows");
  const binDir = fs.mkdtempSync(path.join(os.tmpdir(), "ts-pack-bin-"));
  const archiveBytes = fs.readFileSync(release.archive);
  const fetchBuffer = (url) => {
    if (url.endsWith("/SHA256SUMS")) return Promise.resolve(Buffer.from(`${"0".repeat(64)}  ${release.asset}\n`));
    return Promise.resolve(archiveBytes);
  };
  await assert.rejects(main({ fetchBuffer, binDir, version: "9.8.7" }), /checksum mismatch/);
  assert.ok(!fs.existsSync(path.join(binDir, "ts-pack")));
});

test("main refuses to install when the release has no SHA256SUMS", async (t) => {
  const release = makeFakeRelease("9.8.7");
  if (!release) return t.skip("tar fixture is not built on Windows");
  const binDir = fs.mkdtempSync(path.join(os.tmpdir(), "ts-pack-bin-"));
  const archiveBytes = fs.readFileSync(release.archive);
  const fetchBuffer = (url) => {
    if (url.endsWith("/SHA256SUMS")) return Promise.reject(new Error(`HTTP 404 for ${url}`));
    return Promise.resolve(archiveBytes);
  };
  await assert.rejects(main({ fetchBuffer, binDir, version: "9.8.7" }), /refusing to install an unverified binary/);
});
