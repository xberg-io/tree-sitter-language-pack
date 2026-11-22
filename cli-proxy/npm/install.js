import { execFileSync, spawnSync } from "node:child_process";
import crypto from "node:crypto";
import fs from "node:fs";
import https from "node:https";
import os from "node:os";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const REPO = "xberg-io/tree-sitter-language-pack";
const BIN_NAME = "ts-pack";
const PKG_NAME = "ts-pack-cli";
const ALLOW_UNVERIFIED_ENV = "TS_PACK_CLI_ALLOW_UNVERIFIED";
const USER_AGENT = "ts-pack-cli-npm-proxy";

function targetTriple() {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT") {
    if (arch === "x64") return "x86_64-pc-windows-msvc";
    throw new Error(`unsupported Windows arch: ${arch}`);
  }
  if (type === "Linux") {
    if (arch === "x64") return "x86_64-unknown-linux-gnu";
    if (arch === "arm64") return "aarch64-unknown-linux-gnu";
    throw new Error(`unsupported Linux arch: ${arch}`);
  }
  if (type === "Darwin") {
    if (arch === "arm64") return "aarch64-apple-darwin";
    if (arch === "x64") return "x86_64-apple-darwin";
    throw new Error(`unsupported macOS arch: ${arch}`);
  }
  throw new Error(`unsupported platform: ${type} ${arch}`);
}

function binaryName() {
  return os.type() === "Windows_NT" ? `${BIN_NAME}.exe` : BIN_NAME;
}

function httpGetBuffer(url, { headers = {} } = {}, maxRedirects = 5) {
  return new Promise((resolve, reject) => {
    if (maxRedirects < 0) {
      reject(new Error("too many redirects"));
      return;
    }
    if (!/^https:\/\//i.test(url)) {
      reject(new Error(`refusing non-https URL: ${url}`));
      return;
    }
    const req = https.get(url, { headers: { "User-Agent": USER_AGENT, ...headers } }, (res) => {
      if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
        res.resume();
        const next = res.headers.location;
        if (!/^https:\/\//i.test(next)) {
          return reject(new Error(`refusing non-https redirect to: ${next}`));
        }
        return httpGetBuffer(next, { headers }, maxRedirects - 1).then(resolve, reject);
      }
      if (res.statusCode !== 200) {
        res.resume();
        return reject(new Error(`HTTP ${res.statusCode} for ${url}`));
      }
      const chunks = [];
      res.on("data", (c) => chunks.push(c));
      res.on("end", () => resolve(Buffer.concat(chunks)));
      res.on("error", reject);
    });
    req.on("error", reject);
    req.setTimeout(60000, () => {
      req.destroy();
      reject(new Error(`timeout for ${url}`));
    });
  });
}

export class CliUnavailableError extends Error {
  constructor(message) {
    super(message);
    this.name = "CliUnavailableError";
  }
}

const SUMS_NAME = "SHA256SUMS";

export function packageVersion() {
  const manifest = JSON.parse(fs.readFileSync(new URL("./package.json", import.meta.url), "utf8"));
  return manifest.version;
}

export function archiveName(triple) {
  return `${BIN_NAME}-${triple}.${triple.includes("windows") ? "zip" : "tar.gz"}`;
}

export function assetUrl(version, name) {
  return `https://github.com/${REPO}/releases/download/v${encodeURIComponent(version)}/${name}`;
}

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const BIN_DIR = path.join(__dirname, "bin");

export function expectedDigest(text, assetName) {
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trim();
    if (!line) continue;
    const parts = line.split(/\s+/);
    if (parts.length < 2) continue;
    const name = parts[parts.length - 1].replace(/^\*/, "");
    if (name === assetName) return parts[0].toLowerCase();
  }
  return null;
}

export function verifyArchive(archiveBuf, asset, sumsText, { allowUnverified = false } = {}) {
  if (sumsText === null) {
    if (allowUnverified) {
      process.stderr.write(
        `WARNING: no ${SUMS_NAME} asset found for ${asset}; installing unverified ` +
          `because ${ALLOW_UNVERIFIED_ENV}=1 is set.\n`,
      );
      return;
    }
    throw new Error(
      `no ${SUMS_NAME} asset found in the release for ${asset} - refusing to install an ` +
        `unverified binary. Set ${ALLOW_UNVERIFIED_ENV}=1 to override.`,
    );
  }
  const expected = expectedDigest(sumsText, asset);
  if (!expected) {
    throw new Error(`no checksum entry for ${asset} in ${SUMS_NAME} - refusing to install unverified binary`);
  }
  const actual = crypto.createHash("sha256").update(archiveBuf).digest("hex").toLowerCase();
  if (actual !== expected) {
    throw new Error(`checksum mismatch for ${asset} (expected ${expected}, got ${actual})`);
  }
}

async function fetchSums(version, fetchBuffer) {
  try {
    return (await fetchBuffer(assetUrl(version, SUMS_NAME))).toString("utf8");
  } catch (err) {
    if (/HTTP 404/.test(err.message)) return null;
    throw err;
  }
}

function isUnsafeEntry(name) {
  const entry = String(name).replace(/\\/g, "/").trim();
  if (!entry) return false;
  if (entry.startsWith("/")) return true;
  if (/^[a-zA-Z]:/.test(entry)) return true;
  if (entry.startsWith("//")) return true;
  return entry.split("/").some((part) => part === "..");
}

function listTarEntries(archivePath) {
  const result = spawnSync("tar", ["-tzf", archivePath]);
  if (result.status !== 0) {
    const stderr = result.stderr ? result.stderr.toString() : "";
    throw new Error(`tar listing failed: ${stderr || result.error}`);
  }
  return result.stdout
    .toString()
    .split(/\r?\n/)
    .map((s) => s.trim())
    .filter(Boolean);
}

function extractTarGz(archivePath, destDir) {
  const result = spawnSync("tar", ["-xzf", archivePath, "-C", destDir]);
  if (result.status !== 0) {
    const stderr = result.stderr ? result.stderr.toString() : "";
    throw new Error(`tar extraction failed: ${stderr || result.error}`);
  }
}

function listZipEntries(archivePath) {
  if (os.type() === "Windows_NT") {
    const script =
      "$ErrorActionPreference='Stop';" +
      "Add-Type -AssemblyName System.IO.Compression.FileSystem;" +
      "[System.IO.Compression.ZipFile]::OpenRead($args[0]).Entries |" +
      " ForEach-Object { $_.FullName }";
    const out = execFileSync("powershell", ["-NoProfile", "-NonInteractive", "-Command", script, archivePath], {
      encoding: "utf8",
    });
    return out
      .split(/\r?\n/)
      .map((s) => s.trim())
      .filter(Boolean);
  }
  const result = spawnSync("unzip", ["-Z1", archivePath]);
  if (result.status !== 0) {
    const stderr = result.stderr ? result.stderr.toString() : "";
    throw new Error(`zip listing failed: ${stderr || result.error}`);
  }
  return result.stdout
    .toString()
    .split(/\r?\n/)
    .map((s) => s.trim())
    .filter(Boolean);
}

function extractZip(archivePath, destDir) {
  if (os.type() === "Windows_NT") {
    const result = spawnSync("powershell", [
      "-NoProfile",
      "-NonInteractive",
      "-Command",
      "Expand-Archive",
      "-LiteralPath",
      archivePath,
      "-DestinationPath",
      destDir,
      "-Force",
    ]);
    if (result.status !== 0) {
      const stderr = result.stderr ? result.stderr.toString() : "";
      throw new Error(`zip extraction failed: ${stderr || result.error}`);
    }
    return;
  }
  const result = spawnSync("unzip", ["-o", archivePath, "-d", destDir]);
  if (result.status !== 0) {
    const stderr = result.stderr ? result.stderr.toString() : "";
    throw new Error(`zip extraction failed: ${stderr || result.error}`);
  }
}

function findBinary(dir, name) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      const found = findBinary(full, name);
      if (found) return found;
    } else if (entry.name === name) {
      return full;
    }
  }
  return null;
}

function findDir(dir, name) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    if (!entry.isDirectory()) continue;
    if (entry.name === name) return path.join(dir, entry.name);
    const found = findDir(path.join(dir, entry.name), name);
    if (found) return found;
  }
  return null;
}

function safeExtract(archivePath, asset, dest) {
  const isZip = asset.toLowerCase().endsWith(".zip");
  const entries = isZip ? listZipEntries(archivePath) : listTarEntries(archivePath);
  for (const entry of entries) {
    if (isUnsafeEntry(entry)) {
      throw new Error(`refusing unsafe archive entry: ${entry}`);
    }
  }

  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), `${PKG_NAME}-x-`));
  try {
    if (isZip) {
      extractZip(archivePath, tmpDir);
    } else {
      extractTarGz(archivePath, tmpDir);
    }

    const binName = binaryName();
    const extractedBin = findBinary(tmpDir, binName);
    if (!extractedBin) {
      throw new CliUnavailableError(`archive ${asset} did not contain expected CLI binary ${binName}`);
    }
    const finalBin = path.join(dest, binName);
    fs.copyFileSync(extractedBin, finalBin);

    const libDir = findDir(tmpDir, "lib");
    if (libDir) {
      fs.cpSync(libDir, path.join(dest, "lib"), { recursive: true });
    }
    return finalBin;
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
}

export async function main({ fetchBuffer = httpGetBuffer, binDir = BIN_DIR, version = packageVersion() } = {}) {
  const binName = binaryName();
  const finalPath = path.join(binDir, binName);
  if (fs.existsSync(finalPath)) {
    try {
      const stat = fs.statSync(finalPath);
      const sizeOk = stat.size > 0;
      const execOk = os.type() === "Windows_NT" || (stat.mode & 0o111) !== 0;
      if (sizeOk && execOk) return;
    } catch {}
  }

  fs.mkdirSync(binDir, { recursive: true });

  const name = archiveName(targetTriple());
  process.stderr.write(`Downloading ${BIN_NAME} ${version} asset ${name}...\n`);

  let archiveBuf;
  try {
    archiveBuf = await fetchBuffer(assetUrl(version, name));
  } catch (err) {
    if (/HTTP 404/.test(err.message)) {
      throw new CliUnavailableError(`release v${version} of ${REPO} has no asset ${name}`);
    }
    throw err;
  }
  const sumsText = await fetchSums(version, fetchBuffer);
  verifyArchive(archiveBuf, name, sumsText, { allowUnverified: process.env[ALLOW_UNVERIFIED_ENV] === "1" });

  const stageDir = fs.mkdtempSync(path.join(os.tmpdir(), `${PKG_NAME}-dl-`));
  try {
    const archivePath = path.join(stageDir, name);
    fs.writeFileSync(archivePath, archiveBuf);
    safeExtract(archivePath, name, binDir);
  } finally {
    fs.rmSync(stageDir, { recursive: true, force: true });
  }

  if (os.type() !== "Windows_NT") {
    fs.chmodSync(finalPath, 0o755);
  }
  process.stderr.write(`${BIN_NAME} installed.\n`);
}

if (import.meta.url === pathToFileURL(process.argv[1] || "").href) {
  main().catch((err) => {
    process.stderr.write(`Error installing ${BIN_NAME}: ${err.message}\n`);
    process.exit(1);
  });
}
