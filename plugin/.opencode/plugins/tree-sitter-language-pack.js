// AI-RULEZ :: GENERATED FILE — DO NOT EDIT
// Content-Hash: blake3:1cd1331fdeedd10799bbfbdd9e57436901a2e446e41eeb73af738e8e081948c3
// Source-Hash: blake3:52fd7169d61a07bda345632f78c71b9457bac54f9293e52ef78b2091e1d477cf
// Schema-Version: v1

import { spawn } from "node:child_process";

import { registerBundledContent } from "../ai-rulez-content.js";

function hasValue(value) {
  return value !== undefined && value !== null && value !== "";
}

function pushOption(args, name, value) {
  if (hasValue(value)) {
    args.push(name, String(value));
  }
}

function pushFlag(args, name, enabled) {
  if (enabled) {
    args.push(name);
  }
}

function runCli(args, context) {
  const directory = context?.directory ?? context?.worktree ?? process.cwd();

  return new Promise((resolve, reject) => {
    const child = spawn("ts-pack", args, {
      cwd: directory,
      env: process.env,
      signal: context?.abort,
      stdio: ["ignore", "pipe", "pipe"],
    });

    const stdout = [];
    const stderr = [];

    child.stdout.on("data", (chunk) => stdout.push(chunk));
    child.stderr.on("data", (chunk) => stderr.push(chunk));
    child.on("error", (error) => {
      if (error.code === "ENOENT") {
        resolve({
          title: "ts-pack CLI not found",
          output: "Install the ts-pack CLI with `brew install xberg-io/tap/ts-pack`, or `cargo install ts-pack-cli`.",
          metadata: { exitCode: 127, command: "ts-pack", subcommand: args[0] },
        });
        return;
      }
      reject(error);
    });
    child.on("close", (exitCode, signal) => {
      const stdoutText = Buffer.concat(stdout).toString("utf8").trim();
      const stderrText = Buffer.concat(stderr).toString("utf8").trim();
      const output = [stdoutText, stderrText && `stderr:\n${stderrText}`].filter(Boolean).join("\n\n");

      resolve({
        title: exitCode === 0 ? `ts-pack ${args[0]}` : `ts-pack ${args[0]} failed`,
        output: output || "(no output)",
        metadata: {
          exitCode,
          signal,
          command: "ts-pack",
          subcommand: args[0],
        },
      });
    });
  });
}

const file = {
  type: "string",
  minLength: 1,
  description: "Path to the source file.",
};
const language = {
  type: "string",
  minLength: 1,
  description: "Language override; auto-detected if omitted.",
};
const analysis = ["all", "structure", "imports", "exports", "comments", "symbols", "docstrings", "diagnostics"];

function input(properties, required) {
  return { type: "object", properties, required, additionalProperties: false };
}

export const TreeSitterLanguagePackPlugin = {
  id: "tree-sitter-language-pack",
  async setup(ctx) {
    await registerBundledContent(ctx);
    await ctx.tool.transform((editor) => {
      editor.add({
        name: "tspack_parse",
        description: "Parse a source file into a tree-sitter syntax tree with ts-pack.",
        input: input(
          {
            file,
            language,
            format: { type: "string", enum: ["sexp", "json"], default: "sexp" },
          },
          ["file"],
        ),
        execute(args, context) {
          const cliArgs = ["parse", args.file, "--format", args.format ?? "sexp"];
          pushOption(cliArgs, "--language", args.language);
          return runCli(cliArgs, { ...context, directory: ctx.location.directory });
        },
      });
      const properties = { file, language, chunk_size: { type: "integer", minimum: 1 } };
      for (const name of analysis) properties[name] = { type: "boolean" };
      editor.add({
        name: "tspack_process",
        description:
          "Extract code structure, imports, exports, symbols, docstrings, comments and diagnostics with ts-pack.",
        input: input(properties, ["file"]),
        execute(args, context) {
          const cliArgs = ["process", args.file];
          pushOption(cliArgs, "--language", args.language);
          for (const name of analysis) pushFlag(cliArgs, "--" + name, args[name]);
          pushOption(cliArgs, "--chunk-size", args.chunk_size);
          return runCli(cliArgs, { ...context, directory: ctx.location.directory });
        },
      });
      editor.add({
        name: "tspack_info",
        description: "Show whether a language is supported and cached before parsing.",
        input: input({ language }, ["language"]),
        execute(args, context) {
          return runCli(["info", args.language], { ...context, directory: ctx.location.directory });
        },
      });
    });
  },
};

export default TreeSitterLanguagePackPlugin;
