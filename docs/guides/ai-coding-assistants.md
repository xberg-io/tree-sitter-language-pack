---
description: "Install the tree-sitter-language-pack plugin into Claude Code, Codex, Cursor, Gemini, Factory Droid, GitHub Copilot, or opencode."
---

# AI Coding Assistants

The tree-sitter-language-pack plugin ships the tree-sitter-language-pack agent skills — parse and extract code intelligence from 300+ languages — directly inside your coding agent. It is published in the [`kreuzberg-dev/plugins`](https://github.com/kreuzberg-dev/plugins) marketplace and works with every major coding harness.

## Installing

Pick your harness below.

<details open>
<summary><strong>Claude Code</strong></summary>

```text
/plugin marketplace add kreuzberg-dev/plugins
/plugin install tree-sitter-language-pack@kreuzberg
```

</details>

<details>
<summary><strong>Codex CLI</strong></summary>

```text
/plugins add https://github.com/kreuzberg-dev/plugins
```

Then search for `tree-sitter-language-pack` and select **Install Plugin**.
</details>

<details>
<summary><strong>Cursor</strong></summary>

Settings → Plugins → Add from URL → `https://github.com/kreuzberg-dev/plugins`, then select **tree-sitter-language-pack**.
</details>

<details>
<summary><strong>Gemini CLI</strong></summary>

```text
gemini extensions install https://github.com/kreuzberg-dev/plugins
```

</details>

<details>
<summary><strong>Factory Droid</strong></summary>

```text
droid plugin marketplace add https://github.com/kreuzberg-dev/plugins
droid plugin install tree-sitter-language-pack@kreuzberg
```

</details>

<details>
<summary><strong>GitHub Copilot CLI</strong></summary>

```text
copilot plugin marketplace add https://github.com/kreuzberg-dev/plugins
copilot plugin install tree-sitter-language-pack@kreuzberg
```

</details>

<details>
<summary><strong>opencode</strong></summary>

Add the package to `opencode.json`:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "plugin": ["@kreuzberg/opencode-tree-sitter-language-pack"]
}
```

</details>
