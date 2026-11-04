---
title: "AI Coding Assistants"
description: "Install the tree-sitter-language-pack plugin into Claude Code, Codex, Cursor, Gemini, Factory Droid, GitHub Copilot, or opencode."
---

Give your coding agent structural understanding of any codebase — parse and extract code intelligence from 371 languages without leaving the chat.

## What this plugin does

The plugin drops the tree-sitter-language-pack agent skills straight into your coding assistant. Once installed, the agent can:

- Parse a file in any of 371 languages and reason over its syntax tree.
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

From a checkout of this repository:

```sh
gemini extensions install ./plugin
```

The manifest is inside `plugin/`. See the [Gemini extension reference](https://geminicli.com/docs/extensions/reference/)
for local installation and updates.

</details>

<details>
<summary><strong>Kimi installation and compatibility</strong></summary>

From a checkout of this repository, install with the Python Kimi CLI version used for native
plugin validation:

```sh
uvx --from kimi-cli==1.51.0 kimi plugin install ./plugin
```

Native plugin installation was validated with Python Kimi CLI 1.51.0. The replacement
`@moonshot-ai/kimi-code` 2.1.1 does not provide a `plugin` CLI subcommand; its interactive
plugin loading has not been verified. These results do not establish compatibility with
newer Kimi releases.

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

After a release publishes the updated Hermes package, install it into Hermes' Python environment and enable it in Hermes configuration:

```bash
pip install tree-sitter-language-pack-hermes-plugin
```

</details>

## Runtime resources and prerequisites

The generated bundle targets Claude Code, Codex, Cursor, Gemini CLI, Kimi, OpenCode,
Factory Droid, Hermes, Pi, Agent Plugins and GitHub Copilot CLI. Use
[the plugin installation guide](https://github.com/xberg-io/tree-sitter-language-pack/blob/main/plugin/README.md)
for the runtime's installation method and configuration.

| Runtime | Integration |
| --- | --- |
| Claude, Codex, Cursor, Factory | Native plugin manifests, skills and MCP configuration |
| Gemini CLI | Extension manifest, command templates and MCP configuration |
| Kimi | Plugin manifest and session instructions |
| OpenCode | npm adapter, instructions and MCP configuration |
| Hermes | Python plugin, explicitly selected skills and a local stdio MCP bridge |
| Pi | Skills and prompt templates; execute operations through the CLI |
| Agent Plugins, Copilot | Agent Plugins 1.0 manifest, skills and MCP configuration |

Install the product CLI before using Hermes or Pi. Hermes starts the installed CLI directly;
its wheel does not include the auto-installing launcher used by other MCP clients.
Select `list_tools` through the product's Hermes bridge to inspect the supported tools and
request schemas. Select skills explicitly when needed; they are not appended automatically
to every conversation.

From a checkout, install the generated Hermes package into Hermes' Python environment:

```sh
pip install ./plugin/.hermes/package
```

Enable the plugin in Hermes configuration. Loading project plugins also requires
`HERMES_ENABLE_PROJECT_PLUGINS=true`. After a release publishes the updated package, the
registry install is `pip install tree-sitter-language-pack-hermes-plugin`.

Pi resources can be installed from the checkout with `pi install ./plugin`.
After a plugin release includes them, install `npm:@xberg-io/opencode-tree-sitter-language-pack` through Pi.
Pi does not register an MCP server, hooks or executable extensions from this package.

## Standalone public skills

The [public skills directory](https://github.com/xberg-io/tree-sitter-language-pack/tree/main/skills) contains
instructions you can install independently of the runtime plugin. Use the
[skills CLI](https://github.com/vercel-labs/skills) to list and select them:

```sh
npx skills add xberg-io/tree-sitter-language-pack --list
npx skills add xberg-io/tree-sitter-language-pack
```

Choose the target agents interactively. For a local checkout, use `npx skills add ./skills`.
This installs instructions; it does not install the product CLI, enable MCP, download models
or set provider credentials.
