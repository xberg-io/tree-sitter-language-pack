//! `ts-pack mcp` — embedded MCP (Model Context Protocol) server.
//!
//! Exposes the full ts-pack API surface as MCP tools over stdio or HTTP
//! transport, enabling LLM clients and IDE plugins to query languages,
//! parse files, run the code-intelligence pipeline, and manage the cache.

use std::path::PathBuf;

use clap::Args;

/// Arguments for the `mcp` subcommand.
#[derive(Args)]
pub struct McpArgs {
    /// Path to a language-pack.toml config file (accepted but optional).
    #[arg(long, short)]
    pub config: Option<PathBuf>,
    /// Transport mode: `stdio` or `http`.
    #[arg(long, default_value = "stdio")]
    pub transport: String,
    /// Host to bind for HTTP transport.
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
    /// Port to bind for HTTP transport.
    #[arg(long, default_value_t = 8011)]
    pub port: u16,
}

// ── Tool parameter types ──────────────────────────────────────────────────────

/// Parameters for the `parse` tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ParseParams {
    /// Source code to parse.
    pub source: String,
    /// Language name (e.g. `"python"`). Required.
    pub language: String,
    /// Output format: `"sexp"` (default) or `"json"`.
    pub format: Option<String>,
}

/// Parameters for the `process` tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ProcessParams {
    /// Source code to analyse.
    pub source: String,
    /// Language name (e.g. `"rust"`). Required.
    pub language: String,
    /// Extract structural items (functions, classes). Default: true.
    pub structure: Option<bool>,
    /// Extract import statements. Default: true.
    pub imports: Option<bool>,
    /// Extract export statements. Default: true.
    pub exports: Option<bool>,
    /// Extract comments. Default: false.
    pub comments: Option<bool>,
    /// Extract symbol definitions. Default: false.
    pub symbols: Option<bool>,
    /// Extract docstrings. Default: false.
    pub docstrings: Option<bool>,
    /// Include parse diagnostics. Default: false.
    pub diagnostics: Option<bool>,
    /// Enable hierarchical data extraction for data-format files. Default: false.
    pub data_extraction: Option<bool>,
    /// Maximum chunk size in bytes (`None` disables chunking).
    pub chunk_max_size: Option<usize>,
}

/// Parameters for the `detect_language` tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct DetectLanguageParams {
    /// File path or name used for extension-based detection.
    pub path: Option<String>,
    /// Source content used for content-based detection.
    pub content: Option<String>,
}

/// Parameters for the `list_languages` tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ListLanguagesParams {
    /// Which set to query: `"available"` (default), `"downloaded"`, or `"manifest"`.
    pub source: Option<String>,
    /// Optional substring filter applied to the result.
    pub filter: Option<String>,
}

/// Parameters for the `info` tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct InfoParams {
    /// Language name to inspect.
    pub language: String,
}

/// Parameters for the `download` tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct DownloadParams {
    /// Specific language names to download.
    pub languages: Option<Vec<String>>,
    /// Download all available languages.
    pub all: Option<bool>,
    /// Download a named group (e.g. `"web"`, `"systems"`).
    pub group: Option<String>,
}

/// Parameters for the `cache` tool.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CacheParams {
    /// Action to perform: `"dir"` (default) or `"clean"`.
    pub action: Option<String>,
}

// ── Server struct ─────────────────────────────────────────────────────────────

use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_handler, tool_router,
};

/// MCP server exposing the ts-pack API surface.
#[derive(Clone)]
pub struct TsPackMcp {
    // The `#[tool_router]` macro reads this field through the generated
    // `ServerHandler` delegation; the field looks unused to the dead-code
    // lint because the access happens inside macro-generated code.
    #[allow(dead_code)]
    tool_router: ToolRouter<TsPackMcp>,
}

#[tool_router]
impl TsPackMcp {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Parse source code with a tree-sitter grammar.
    ///
    /// Returns the syntax tree as an S-expression or JSON.
    #[tool(
        description = "Parse source code with a tree-sitter grammar. Returns the syntax tree as sexp or JSON.",
        annotations(title = "Parse", read_only_hint = true, idempotent_hint = true)
    )]
    fn parse(&self, Parameters(params): Parameters<ParseParams>) -> Result<CallToolResult, rmcp::ErrorData> {
        use tree_sitter_language_pack::get_parser;

        let mut parser = get_parser(&params.language)
            .map_err(|e| rmcp::ErrorData::invalid_params(format!("Language error: {e}"), None))?;

        let tree = parser
            .parse_bytes(params.source.as_bytes())
            .ok_or_else(|| rmcp::ErrorData::internal_error("Parser returned no tree", None))?;

        let format = params.format.as_deref().unwrap_or("sexp");

        let response = match format {
            "json" => {
                let sexp = tree.root_node().to_sexp();
                serde_json::to_string_pretty(&serde_json::json!({
                    "language": params.language,
                    "sexp": sexp,
                    "has_errors": tree.root_node().has_error(),
                }))
                .unwrap_or_default()
            }
            _ => tree.root_node().to_sexp(),
        };

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Run the code-intelligence pipeline on source code.
    ///
    /// Extracts structure, imports, exports, comments, symbols, docstrings,
    /// diagnostics, and/or chunks. Returns JSON.
    #[tool(
        description = "Run the code-intelligence pipeline on source code. Extracts structure, imports, exports, symbols, and more.",
        annotations(title = "Process", read_only_hint = true, idempotent_hint = true)
    )]
    fn process(&self, Parameters(params): Parameters<ProcessParams>) -> Result<CallToolResult, rmcp::ErrorData> {
        use tree_sitter_language_pack::{ProcessConfig, process};

        let mut config = ProcessConfig::new(params.language);

        if let Some(v) = params.structure {
            config.structure = v;
        }
        if let Some(v) = params.imports {
            config.imports = v;
        }
        if let Some(v) = params.exports {
            config.exports = v;
        }
        if let Some(v) = params.comments {
            config.comments = v;
        }
        if let Some(v) = params.symbols {
            config.symbols = v;
        }
        if let Some(v) = params.docstrings {
            config.docstrings = v;
        }
        if let Some(v) = params.diagnostics {
            config.diagnostics = v;
        }
        if let Some(v) = params.data_extraction {
            config.data_extraction = v;
        }
        if let Some(sz) = params.chunk_max_size {
            config.chunk_max_size = Some(sz);
        }

        let result =
            process(&params.source, &config).map_err(|e| rmcp::ErrorData::invalid_params(e.to_string(), None))?;

        let response = serde_json::to_string_pretty(&result).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Detect the language for a file path or source content.
    ///
    /// Returns the detected language name or `null` when detection fails.
    #[tool(
        description = "Detect the language for a file path or source content. Returns the detected language name.",
        annotations(title = "Detect Language", read_only_hint = true, idempotent_hint = true)
    )]
    fn detect_language(
        &self,
        Parameters(params): Parameters<DetectLanguageParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        use tree_sitter_language_pack::{detect_language_from_content, detect_language_from_path};

        let detected = params
            .path
            .as_deref()
            .and_then(detect_language_from_path)
            .or_else(|| params.content.as_deref().and_then(detect_language_from_content));

        let response = serde_json::to_string_pretty(&serde_json::json!({
            "language": detected,
            "path": params.path,
        }))
        .unwrap_or_default();

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// List available, downloaded, or manifest languages.
    ///
    /// Pass `source` = `"available"` (default), `"downloaded"`, or `"manifest"`.
    #[tool(
        description = "List languages. source: 'available' (default), 'downloaded', or 'manifest'. Optional substring filter.",
        annotations(title = "List Languages", read_only_hint = true, idempotent_hint = true)
    )]
    fn list_languages(
        &self,
        Parameters(params): Parameters<ListLanguagesParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        use tree_sitter_language_pack::{available_languages, downloaded_languages, manifest_languages};

        let source = params.source.as_deref().unwrap_or("available");
        let langs: Vec<String> = match source {
            "downloaded" => downloaded_languages(),
            "manifest" => manifest_languages().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?,
            _ => available_languages(),
        };

        let filtered: Vec<&String> = if let Some(ref f) = params.filter {
            langs.iter().filter(|l| l.contains(f.as_str())).collect()
        } else {
            langs.iter().collect()
        };

        let response = serde_json::to_string_pretty(&serde_json::json!({
            "source": source,
            "filter": params.filter,
            "count": filtered.len(),
            "languages": filtered,
        }))
        .unwrap_or_default();

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Show information about a specific language.
    ///
    /// Returns whether the language is known, downloaded, and its cache path.
    #[tool(
        description = "Show whether a language is known, downloaded, and its cache path.",
        annotations(title = "Language Info", read_only_hint = true, idempotent_hint = true)
    )]
    fn info(&self, Parameters(params): Parameters<InfoParams>) -> Result<CallToolResult, rmcp::ErrorData> {
        use tree_sitter_language_pack::{cache_dir, downloaded_languages, has_language};

        let known = has_language(&params.language);
        let is_downloaded = downloaded_languages().contains(&params.language);
        let cache = cache_dir().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

        let response = serde_json::to_string_pretty(&serde_json::json!({
            "language": params.language,
            "known": known,
            "downloaded": is_downloaded,
            "cache_dir": cache,
        }))
        .unwrap_or_default();

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Download parser libraries for specific languages, a group, or all.
    ///
    /// Set `all: true` to download everything, `group` to download a named group,
    /// or pass `languages` with a list of names.
    #[tool(
        description = "Download parser libraries. Pass languages list, group name, or all=true.",
        annotations(title = "Download", destructive_hint = false)
    )]
    fn download(&self, Parameters(params): Parameters<DownloadParams>) -> Result<CallToolResult, rmcp::ErrorData> {
        use tree_sitter_language_pack::{download, download_all, download_group};

        let count = if params.all.unwrap_or(false) {
            download_all().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?
        } else if let Some(ref group) = params.group {
            download_group(group).map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?
        } else if let Some(ref languages) = params.languages {
            if languages.is_empty() {
                return Err(rmcp::ErrorData::invalid_params(
                    "Provide at least one language name, a group, or set all=true",
                    None,
                ));
            }
            let refs: Vec<&str> = languages.iter().map(String::as_str).collect();
            download(&refs).map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?
        } else {
            return Err(rmcp::ErrorData::invalid_params(
                "Provide languages, group, or all=true",
                None,
            ));
        };

        let response = serde_json::to_string_pretty(&serde_json::json!({
            "languages_available": count,
        }))
        .unwrap_or_default();

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Query or manage the parser cache.
    ///
    /// Use `action = "dir"` (default) to return the cache directory path,
    /// or `action = "clean"` to delete all cached parser libraries.
    #[tool(
        description = "Query or manage the parser cache. action: 'dir' (default) returns the path; 'clean' deletes cached parsers.",
        annotations(title = "Cache")
    )]
    fn cache(&self, Parameters(params): Parameters<CacheParams>) -> Result<CallToolResult, rmcp::ErrorData> {
        use tree_sitter_language_pack::{cache_dir, clean_cache};

        let action = params.action.as_deref().unwrap_or("dir");

        let response = match action {
            "clean" => {
                clean_cache().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
                let dir = cache_dir().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
                serde_json::to_string_pretty(&serde_json::json!({
                    "action": "clean",
                    "cache_dir": dir,
                    "status": "cleared",
                }))
                .unwrap_or_default()
            }
            _ => {
                let dir = cache_dir().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
                serde_json::to_string_pretty(&serde_json::json!({
                    "action": "dir",
                    "cache_dir": dir,
                }))
                .unwrap_or_default()
            }
        };

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }
}

#[tool_handler]
impl ServerHandler for TsPackMcp {
    fn get_info(&self) -> ServerInfo {
        let mut capabilities = ServerCapabilities::default();
        capabilities.tools = Some(ToolsCapability::default());

        let server_info = Implementation::new("ts-pack-mcp", env!("CARGO_PKG_VERSION"))
            .with_title("Tree-Sitter Language Pack MCP Server")
            .with_description(
                "MCP server for the tree-sitter language pack. \
                 Parse source code, run code-intelligence analysis, detect languages, \
                 list and download parsers, and manage the cache.",
            )
            .with_website_url("https://github.com/kreuzberg-dev/tree-sitter-language-pack");

        InitializeResult::new(capabilities)
            .with_server_info(server_info)
            .with_instructions(
                "Use 'parse' to get a syntax tree (sexp or JSON). \
                 Use 'process' to extract structure, imports, exports, symbols, comments, and diagnostics. \
                 Use 'detect_language' to identify a language from a file path or source snippet. \
                 Use 'list_languages' to see available, downloaded, or manifest languages. \
                 Use 'info' to check whether a specific language is downloaded. \
                 Use 'download' to fetch parser libraries (by name, group, or all). \
                 Use 'cache' to query the cache directory or clean all cached parsers.",
            )
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

/// Run the MCP server with the given transport.
pub async fn run(args: McpArgs) -> Result<(), String> {
    use rmcp::ServiceExt;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .init();

    // If a config path is provided, apply it so the language pack uses the
    // specified cache directory and pre-warms configured language groups.
    if let Some(ref path) = args.config {
        let config = tree_sitter_language_pack::PackConfig::from_toml_file(path)
            .map_err(|e| format!("Failed to load config '{}': {e}", path.display()))?;
        tree_sitter_language_pack::init(&config).map_err(|e| format!("Init failed: {e}"))?;
    }

    match args.transport.as_str() {
        "stdio" => {
            tracing::info!("starting ts-pack MCP server (stdio transport)");
            let service = TsPackMcp::new()
                .serve(rmcp::transport::stdio())
                .await
                .map_err(|e| format!("MCP stdio serve failed: {e}"))?;
            service.waiting().await.map_err(|e| format!("MCP server error: {e}"))?;
        }
        "http" => {
            use rmcp::transport::streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager};

            let addr: std::net::SocketAddr = format!("{}:{}", args.host, args.port)
                .parse()
                .map_err(|e| format!("invalid MCP listen address: {e}"))?;

            let http_service = StreamableHttpService::new(
                || Ok(TsPackMcp::new()),
                LocalSessionManager::default().into(),
                Default::default(),
            );

            let router = axum::Router::new().nest_service("/mcp", http_service);

            tracing::info!("starting ts-pack MCP server (HTTP transport) on {addr}");
            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .map_err(|e| format!("failed to bind MCP HTTP {addr}: {e}"))?;
            axum::serve(listener, router)
                .await
                .map_err(|e| format!("MCP HTTP server error: {e}"))?;
        }
        other => return Err(format!("unknown transport '{other}', use 'stdio' or 'http'")),
    }

    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_router_has_all_tools() {
        let router = TsPackMcp::tool_router();
        for name in [
            "parse",
            "process",
            "detect_language",
            "list_languages",
            "info",
            "download",
            "cache",
        ] {
            assert!(router.has_route(name), "Expected tool '{name}' to be registered");
        }
    }

    #[test]
    fn test_server_info_fields() {
        let server = TsPackMcp::new();
        let info = server.get_info();

        assert_eq!(info.server_info.name, "ts-pack-mcp");
        assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));
        assert!(info.capabilities.tools.is_some());
        assert!(info.instructions.is_some());
    }

    #[test]
    fn test_list_languages_available() {
        let server = TsPackMcp::new();
        let result = server.list_languages(Parameters(ListLanguagesParams {
            source: None,
            filter: None,
        }));
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_languages_with_filter() {
        let server = TsPackMcp::new();
        let result = server.list_languages(Parameters(ListLanguagesParams {
            source: Some("available".to_string()),
            filter: Some("python".to_string()),
        }));
        assert!(result.is_ok());
        let call = result.unwrap();
        if let Some(content) = call.content.first()
            && let rmcp::model::RawContent::Text(text) = &content.raw
        {
            let parsed: serde_json::Value = serde_json::from_str(&text.text).expect("Should be valid JSON");
            assert_eq!(parsed["filter"], "python");
            assert_eq!(parsed["source"], "available");
            assert!(parsed["count"].is_number());
        }
    }

    #[test]
    fn test_cache_dir_action() {
        let server = TsPackMcp::new();
        let result = server.cache(Parameters(CacheParams {
            action: Some("dir".to_string()),
        }));
        assert!(result.is_ok());
        let call = result.unwrap();
        if let Some(content) = call.content.first()
            && let rmcp::model::RawContent::Text(text) = &content.raw
        {
            let parsed: serde_json::Value = serde_json::from_str(&text.text).expect("Should be valid JSON");
            assert_eq!(parsed["action"], "dir");
            assert!(parsed["cache_dir"].is_string());
        }
    }

    #[test]
    fn test_detect_language_from_path() {
        let server = TsPackMcp::new();
        let result = server.detect_language(Parameters(DetectLanguageParams {
            path: Some("main.py".to_string()),
            content: None,
        }));
        assert!(result.is_ok());
        let call = result.unwrap();
        if let Some(content) = call.content.first()
            && let rmcp::model::RawContent::Text(text) = &content.raw
        {
            let parsed: serde_json::Value = serde_json::from_str(&text.text).expect("Should be valid JSON");
            // Python should be detected from .py extension
            assert_eq!(parsed["language"], "python");
        }
    }

    #[test]
    fn test_download_requires_params() {
        let server = TsPackMcp::new();
        let result = server.download(Parameters(DownloadParams {
            languages: None,
            all: None,
            group: None,
        }));
        assert!(result.is_err());
    }
}
