---
description: "Install the tree-sitter-language-pack plugin into Claude Code, Codex, Cursor, Gemini, Factory Droid, GitHub Copilot, or opencode."
---

# AI Coding Assistants

The tree-sitter-language-pack plugin ships the tree-sitter-language-pack agent skills — parse and extract code intelligence from 300+ languages — directly inside your coding agent. It is published in the [`xberg-io/plugins`](https://github.com/xberg-io/plugins) marketplace and works with every major coding harness.

For advanced users who prefer manual MCP registration over the plugin, the CLI exposes a raw MCP server. See the [MCP Server guide](mcp-server.md) for stdio and HTTP transport setup with any compatible IDE.

## Installing

Pick your harness below.

<details open>
<summary><strong>Claude Code</strong></summary>

```text
/plugin marketplace add xberg-io/plugins
/plugin install tree-sitter-language-pack@xberg-io
```

</details>

<details>
<summary><strong>Codex CLI</strong></summary>

```text
/plugins add https://github.com/xberg-io/plugins
```

Then search for `tree-sitter-language-pack` and select **Install Plugin**.
</details>

<details>
<summary><strong>Cursor</strong></summary>

Settings → Plugins → Add from URL → `https://github.com/xberg-io/plugins`, then select **tree-sitter-language-pack**.
</details>

<details>
<summary><strong>Gemini CLI</strong></summary>

```text
gemini extensions install https://github.com/xberg-io/plugins
```

</details>

<details>
<summary><strong>Factory Droid</strong></summary>

```text
droid plugin marketplace add https://github.com/xberg-io/plugins
droid plugin install tree-sitter-language-pack@xberg-io
```

</details>

<details>
<summary><strong>GitHub Copilot CLI</strong></summary>

```text
copilot plugin marketplace add https://github.com/xberg-io/plugins
copilot plugin install tree-sitter-language-pack@xberg-io
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
