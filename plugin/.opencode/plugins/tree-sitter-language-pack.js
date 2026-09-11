// AI-RULEZ :: GENERATED FILE — DO NOT EDIT
// Content-Hash: blake3:22d84b89b01d054cd550760e43b4e8fbcfed01b4118be59c11d24da2c2e2d7d5
// Source-Hash: blake3:9e073d427a178d533cfbe4d3d43f29d564c0bda35ef83d901c3958931fb60dc0
// Schema-Version: v1

import { tool } from "@opencode-ai/plugin";
import { spawn } from "node:child_process";

const schema = tool.schema;

const parseFormat = schema.enum(["sexp", "json"]).default("sexp").describe("Parse output format.");

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
          output:
            "Install the ts-pack CLI with `brew install xberg-io/tap/ts-pack`, or run it via `npx @xberg-io/ts-pack-cli` (the CLI proxy's bin is `ts-pack`).",
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

export const TreeSitterLanguagePackPlugin = async () => ({
  tool: {
    tspack_parse: tool({
      description: "Parse a source file into a tree-sitter syntax tree with the ts-pack CLI.",
      args: {
        file: schema.string().min(1).describe("Path to the source file."),
        language: schema
          .string()
          .min(1)
          .optional()
          .describe("Language override (auto-detected from extension if omitted)."),
        format: parseFormat,
      },
      async execute(args, context) {
        const cliArgs = ["parse", args.file, "--format", args.format];
        pushOption(cliArgs, "--language", args.language);
        return runCli(cliArgs, context);
      },
    }),
    tspack_process: tool({
      description:
        "Extract code intelligence (structure, imports, exports, symbols, docstrings, comments, diagnostics) from a source file with the ts-pack CLI. Output is JSON.",
      args: {
        file: schema.string().min(1).describe("Path to the source file."),
        language: schema
          .string()
          .min(1)
          .optional()
          .describe("Language override (auto-detected from extension if omitted)."),
        all: schema.boolean().optional().describe("Enable all analysis features."),
        structure: schema.boolean().optional().describe("Extract structure (functions, classes)."),
        imports: schema.boolean().optional().describe("Extract imports."),
        exports: schema.boolean().optional().describe("Extract exports."),
        comments: schema.boolean().optional().describe("Extract comments."),
        symbols: schema.boolean().optional().describe("Extract symbols."),
        docstrings: schema.boolean().optional().describe("Extract docstrings."),
        diagnostics: schema.boolean().optional().describe("Include syntax diagnostics."),
        chunk_size: schema
          .number()
          .int()
          .positive()
          .optional()
          .describe("Maximum chunk size in bytes (enables syntax-aware chunking)."),
      },
      async execute(args, context) {
        const cliArgs = ["process", args.file];
        pushOption(cliArgs, "--language", args.language);
        pushFlag(cliArgs, "--all", args.all);
        pushFlag(cliArgs, "--structure", args.structure);
        pushFlag(cliArgs, "--imports", args.imports);
        pushFlag(cliArgs, "--exports", args.exports);
        pushFlag(cliArgs, "--comments", args.comments);
        pushFlag(cliArgs, "--symbols", args.symbols);
        pushFlag(cliArgs, "--docstrings", args.docstrings);
        pushFlag(cliArgs, "--diagnostics", args.diagnostics);
        pushOption(cliArgs, "--chunk-size", args.chunk_size);
        return runCli(cliArgs, context);
      },
    }),
    tspack_info: tool({
      description:
        "Show details about a language (whether it is known and cached) with the ts-pack CLI. Use to confirm a language is supported before parsing.",
      args: {
        language: schema.string().min(1).describe("Language name (e.g. python, rust, typescript)."),
      },
      async execute(args, context) {
        return runCli(["info", args.language], context);
      },
    }),
  },
});

export default TreeSitterLanguagePackPlugin;
