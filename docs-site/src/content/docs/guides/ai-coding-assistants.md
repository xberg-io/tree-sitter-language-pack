---
title: "AI Coding Assistants"
description: "Install the tree-sitter-language-pack plugin into Claude Code, Codex, Cursor, Gemini, Factory Droid, GitHub Copilot, or opencode."
---

Give your coding agent structural understanding of any codebase — parse and extract code intelligence from 372 languages without leaving the chat.

## What this plugin does

The plugin drops the tree-sitter-language-pack agent skills straight into your coding assistant. Once installed, the agent can:

- Parse a file in any of 372 languages and reason over its syntax tree.
- Pull out functions, classes, imports, exports, and symbols when asked.
- Detect a file's language, list supported languages, and manage the local parser cache.

Under the hood it registers the `tree-sitter-language-pack` MCP server for you, so there is nothing to configure by hand. The plugin ships from this repository's own marketplace, [`xberg-io/tree-sitter-language-pack`](https://github.com/xberg-io/tree-sitter-language-pack), where you can also see its version history and source.

If you prefer manual MCP registration over the plugin, the CLI exposes the same server directly. See the [MCP Server guide](/guides/mcp-server/) for stdio and HTTP transport setup with any compatible IDE.

## Installing

Pick your harness below.

<details open>
<summary><strong>Claude Code</strong></summary>

```text
/plugin marketplace add xberg-io/tree-sitter-language-pack
/plugin install tree-sitter-language-pack@tree-sitter-language-pack
```

</details>

<details>
<summary><strong>Codex CLI</strong></summary>

```text
/plugins add https://github.com/xberg-io/tree-sitter-language-pack
```

Then search for `tree-sitter-language-pack` and select **Install Plugin**.
</details>

<details>
<summary><strong>Cursor</strong></summary>

Settings → Plugins → Add from URL → `https://github.com/xberg-io/tree-sitter-language-pack`, then select **tree-sitter-language-pack**.
</details>

<details>
<summary><strong>Gemini CLI</strong></summary>

```text
gemini extensions install https://github.com/xberg-io/tree-sitter-language-pack
```

</details>

<details>
<summary><strong>Factory Droid</strong></summary>

```text
droid plugin marketplace add https://github.com/xberg-io/tree-sitter-language-pack
droid plugin install tree-sitter-language-pack@tree-sitter-language-pack
```

</details>

<details>
<summary><strong>GitHub Copilot CLI</strong></summary>

```text
copilot plugin marketplace add https://github.com/xberg-io/tree-sitter-language-pack
copilot plugin install tree-sitter-language-pack@tree-sitter-language-pack
```

</details>

<details>
<summary><strong>opencode</strong></summary>

Add the package to `opencode.json`:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "plugin": ["@xberg-io/opencode-tree-sitter-language-pack"]
}
```

</details>

<details>
<summary><strong>Hermes</strong></summary>

Install the Hermes plugin from PyPI — Hermes auto-discovers it via entry points, so no extra configuration is needed:

```bash
pip install tree-sitter-language-pack-hermes-plugin
```

</details>
