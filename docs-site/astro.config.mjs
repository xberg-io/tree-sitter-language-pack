// @ts-check
import starlight from "@astrojs/starlight";
import { xbergStarlightConfig } from "@xberg-io/docs-theme";
import { defineConfig } from "astro/config";
import starlightLlmsTxt from "starlight-llms-txt";

const API_LANGUAGES = [
  { label: "Python", slug: "reference/api-python" },
  { label: "TypeScript / Node.js", slug: "reference/api-typescript" },
  { label: "Rust", slug: "reference/api-rust" },
  { label: "Go", slug: "reference/api-go" },
  { label: "Java", slug: "reference/api-java" },
  { label: "Kotlin (Android)", slug: "reference/api-kotlin-android" },
  { label: "C#", slug: "reference/api-csharp" },
  { label: "Ruby", slug: "reference/api-ruby" },
  { label: "PHP", slug: "reference/api-php" },
  { label: "Elixir", slug: "reference/api-elixir" },
  { label: "Dart", slug: "reference/api-dart" },
  { label: "Swift", slug: "reference/api-swift" },
  { label: "Zig", slug: "reference/api-zig" },
  { label: "WebAssembly", slug: "reference/api-wasm" },
  { label: "C / FFI", slug: "reference/api-c" },
];

export default defineConfig({
  site: "https://docs.tree-sitter-language-pack.xberg.io",
  integrations: [
    starlight(
      xbergStarlightConfig({
        title: "tree-sitter-language-pack",
        description:
          "372 tree-sitter parsers with code intelligence, chunking, and native bindings for " +
          "Python, Node.js, Rust, Go, Java, Ruby, Elixir, PHP, and WebAssembly.",
        githubUrl: "https://github.com/xberg-io/tree-sitter-language-pack",
        editBaseUrl: "https://github.com/xberg-io/tree-sitter-language-pack/edit/main/docs-site/",
        plugins: [
          starlightLlmsTxt({
            customSets: [
              {
                label: "Get Started",
                description: "Installation and first-parse walkthrough.",
                paths: ["getting-started/**"],
              },
              {
                label: "Guides",
                description:
                  "Parsing, code intelligence, chunking, extraction, the CLI, the MCP server, " +
                  "AI coding assistants, and deployment.",
                paths: ["guides/**"],
              },
              {
                label: "Concepts",
                description: "Architecture, language passthrough, the download model, and code intelligence.",
                paths: ["concepts/**"],
              },
              {
                label: "Reference",
                description:
                  "Per-language API reference, the supported-language list, CLI, MCP, types, " +
                  "errors, and configuration.",
                paths: ["reference/**", "languages"],
              },
              {
                label: "More",
                description: "Contributing, CI/CD, changelog, and the Xberg ecosystem.",
                paths: ["contributing", "contributing/**", "changelog", "ecosystem"],
              },
            ],
            optionalLinks: [
              {
                label: "GitHub",
                url: "https://github.com/xberg-io/tree-sitter-language-pack",
                description: "Source code and issues",
              },
            ],
          }),
        ],
        sidebar: [
          { label: "Home", link: "/" },
          {
            label: "Get Started",
            items: [
              { label: "Installation", slug: "getting-started/installation" },
              { label: "Quick Start", slug: "getting-started/quickstart" },
            ],
          },
          {
            label: "Guides",
            items: [
              {
                label: "Core",
                items: [
                  { label: "Parsing Code", slug: "guides/parsing" },
                  { label: "Code Intelligence", slug: "guides/intelligence" },
                  { label: "Chunking for LLMs", slug: "guides/chunking" },
                  { label: "Configuration", slug: "guides/configuration" },
                  { label: "Extraction Queries", slug: "guides/extraction" },
                ],
              },
              { label: "CLI", slug: "guides/cli" },
              { label: "MCP Server", slug: "guides/mcp-server" },
              {
                label: "AI Coding Assistants",
                slug: "guides/ai-coding-assistants",
              },
              {
                label: "Deployment",
                items: [
                  { label: "Docker", slug: "guides/docker" },
                  { label: "Building from Source", slug: "guides/building" },
                  {
                    label: "Performance & Benchmarks",
                    slug: "guides/performance",
                  },
                ],
              },
            ],
          },
          {
            label: "Concepts",
            items: [
              { label: "Architecture", slug: "concepts/architecture" },
              {
                label: "Language Passthrough",
                slug: "concepts/language-passthrough",
              },
              { label: "Download Model", slug: "concepts/download-model" },
              {
                label: "Code Intelligence",
                slug: "concepts/code-intelligence",
              },
            ],
          },
          {
            label: "Reference",
            items: [
              { label: "API", items: API_LANGUAGES },
              { label: "Languages", slug: "languages" },
              { label: "CLI", slug: "reference/cli" },
              { label: "MCP", slug: "reference/mcp" },
              { label: "Types", slug: "reference/types" },
              { label: "Errors", slug: "reference/errors" },
              { label: "Configuration", slug: "reference/configuration" },
            ],
          },
          {
            label: "More",
            items: [
              { label: "Contributing", slug: "contributing" },
              { label: "CI / CD", slug: "contributing/ci" },
              { label: "Changelog", slug: "changelog" },
              { label: "Ecosystem", slug: "ecosystem" },
            ],
          },
        ],
      }),
    ),
  ],
});
