---
description: "Set up the tree-sitter-language-pack MCP server for AI agents — stdio and HTTP transport, IDE integration with Claude Code, Cursor, VS Code."
---

# MCP Server

The tree-sitter-language-pack CLI includes an MCP server that exposes parsing, code intelligence extraction, language detection, and cache management as standard tools for AI agents. Use it to add code analysis to Claude, Cursor, VS Code, or any MCP-compatible application.

## What is MCP?

The [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) is an open standard for connecting AI applications to tools and data. The tree-sitter-language-pack MCP server provides tools for parsing source code, analyzing structure and symbols, and managing language packs — all through a unified interface.

## Starting the Server

### Stdio Transport (Default)

For local AI tools — Claude Desktop, Cursor, VS Code — use stdio transport:

```bash
ts-pack mcp
```

The server runs as a subprocess and communicates over stdin/stdout with JSON-RPC messages. No network configuration needed.

### HTTP Transport

For remote agents or team environments where stdio doesn't work:

```bash
ts-pack mcp --transport http --host 127.0.0.1 --port 8011
```

The server listens on `http://127.0.0.1:8011` (default). Change `--host` to `0.0.0.0` for network-wide access (use with caution).

### Custom Configuration

Point the server at a `language-pack.toml` config file:

```bash
ts-pack mcp --config /path/to/language-pack.toml
```

This sets default languages and download preferences for all tool calls.

## Registering with AI Tools

### Claude Desktop / Claude Code

Add to `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) or `%APPDATA%\Claude\claude_desktop_config.json` (Windows):

```json
{
  "mcpServers": {
    "ts-pack": {
      "command": "ts-pack",
      "args": ["mcp"]
    }
  }
}
```

Or use the CLI to register automatically:

```bash
claude mcp add ts-pack -- ts-pack mcp
```

Restart Claude. The tree-sitter-language-pack tools appear in the Tools panel.

### Cursor

Edit `.cursor/mcp.json` in your project root (or global Cursor settings):

```json
{
  "mcpServers": {
    "ts-pack": {
      "command": "ts-pack",
      "args": ["mcp"]
    }
  }
}
```

Reload Cursor. Tools are now available in the AI chat.

### VS Code / GitHub Copilot

Edit `.vscode/settings.json` or your VS Code global settings:

```json
{
  "mcpServers": [
    {
      "name": "ts-pack",
      "command": "ts-pack",
      "args": ["mcp"]
    }
  ]
}
```

Then reference tools in GitHub Copilot chat or use the Tools panel.

### Generic MCP Client

For a client that spawns the server over stdio, point it at the `ts-pack` binary:

```json
{
  "mcpServers": [
    {
      "name": "ts-pack",
      "command": "ts-pack",
      "args": ["mcp"]
    }
  ]
}
```

For an HTTP client, start the server yourself (`ts-pack mcp --transport http --port 8011`)
and connect to its URL:

```json
{
  "mcpServers": [
    {
      "name": "ts-pack",
      "url": "http://127.0.0.1:8011"
    }
  ]
}
```

## Tools

The MCP server exposes 8 tools for parsing, analysis, and management:

=== "Parsing & Analysis"

    **`parse`**

    Render the syntax tree as S-expression or JSON.

    | Parameter | Type | Description |
    |-----------|------|-------------|
    | `source` | string | Source code to parse |
    | `language` | string | Language name (e.g., `python`, `rust`) |
    | `format` | string | Output format: `sexp` or `json` (default: `sexp`) |

    **`process`**

    Extract code intelligence: structure, imports, exports, symbols, docstrings, comments, diagnostics, and optionally chunk for LLMs.

    | Parameter | Type | Description |
    |-----------|------|-------------|
    | `source` | string | Source code to analyze |
    | `language` | string | Language name |
    | `all` | boolean | Extract all intelligence fields (default: false) |
    | `chunk_size` | integer | Split output into chunks for LLMs (optional) |
    | `chunk_overlap` | integer | Overlap between chunks in tokens (optional) |

    **`detect_language`**

    Identify language from file path or source code.

    | Parameter | Type | Description |
    |-----------|------|-------------|
    | `path` | string | File path (e.g., `main.rs`) — optional |
    | `source` | string | Source code — optional, used if path unclear |

=== "Languages & Discovery"

    **`list_languages`**

    Enumerate available, downloaded, or manifest languages.

    | Parameter | Type | Description |
    |-----------|------|-------------|
    | `filter` | string | One of: `available` (default), `downloaded`, `manifest` |
    | `query` | string | Filter by name or pattern (optional) |

    **`info`**

    Get status of a specific language (available, downloaded, extensions, aliases).

    | Parameter | Type | Description |
    |-----------|------|-------------|
    | `name` | string | Language name (e.g., `python`) |

    **`download`**

    Fetch language parsers for offline use.

    | Parameter | Type | Description |
    |-----------|------|-------------|
    | `languages` | array | List of language names — optional |
    | `groups` | string | Download a group: `web`, `systems`, `data`, `all` — optional |
    | `fresh` | boolean | Force fresh download (ignore cache) |

=== "Cache & Configuration"

    **`cache_dir`**

    Retrieve the local cache directory where parsers are stored.

    Returns: path to the cache (e.g., `~/.ts-pack-cache`)

    **`clean_cache`**

    Delete cached parsers.

    | Parameter | Type | Description |
    |-----------|------|-------------|
    | `all` | boolean | Delete all cached parsers (default: false) |
    | `languages` | array | Delete parsers for specific languages — optional |

## Resources

The MCP server provides read-only resources for browsing the language catalog:

- **`ts-pack://languages`** — list of all 306 available languages with extensions and aliases
- **`ts-pack://languages/downloaded`** — list of user-downloaded languages
- **`ts-pack://language/{name}`** — status of a specific language (template resource)

## Prompt

The MCP server includes a built-in prompt template:

**`analyze-code`**

Analyzes source code by detecting its language, extracting code intelligence, and formatting results for readability. Use this when you want the agent to understand code structure without manual language specification.

| Parameter | Type | Description |
|-----------|------|-------------|
| `source` | string | Source code to analyze |
| `focus` | string | What to extract: `all`, `structure`, `imports`, `exports`, `symbols` (default: `all`) |

## Using the Plugin Instead

For most users, installing the tree-sitter-language-pack plugin from [`kreuzberg-dev/plugins`](https://github.com/kreuzberg-dev/plugins) is simpler than manual MCP registration. The plugin auto-registers the server and is maintained alongside the core library.

See [AI Coding Assistants](ai-coding-assistants.md) for installation steps.

## Next Steps

- [CLI Guide](cli.md) — full CLI command reference
- [Parsing Code](parsing.md) — understanding syntax trees
- [Code Intelligence](intelligence.md) — extract structure and symbols
- [Installation](../getting-started/installation.md) — install ts-pack-cli
