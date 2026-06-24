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
    /// Enable every analysis feature. Overrides the individual flags below when true.
    pub all: Option<bool>,
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
    /// Named groups to download (e.g. `"web"`, `"systems"`, `"scripting"`, `"data"`, `"jvm"`, `"functional"`).
    pub groups: Option<Vec<String>>,
    /// Clean the cache before downloading for a fresh fetch. Default: false.
    pub fresh: Option<bool>,
}

// ── Server struct ─────────────────────────────────────────────────────────────

use rmcp::{
    RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    service::RequestContext,
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
        annotations(
            title = "Parse",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        )
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
        annotations(
            title = "Process",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    fn process(&self, Parameters(params): Parameters<ProcessParams>) -> Result<CallToolResult, rmcp::ErrorData> {
        use tree_sitter_language_pack::{ProcessConfig, process};

        let mut config = ProcessConfig::new(params.language);

        // Mirror the CLI `--all` flag: enable every analysis feature via the builder.
        if params.all.unwrap_or(false) {
            config = config.all();
        }
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
        annotations(
            title = "Detect Language",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        )
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
        // `open_world_hint = true`: the `manifest` source fetches the remote download manifest.
        annotations(
            title = "List Languages",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = true
        )
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
        annotations(
            title = "Language Info",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        )
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
        description = "Download parser libraries from the remote registry. Pass languages list, groups, or all=true. \
                       Set fresh=true to clean the cache first.",
        // Network fetch: `open_world_hint = true`. Additive to the cache (re-downloading is a no-op),
        // so `destructive_hint = false` and `idempotent_hint = true`. The `fresh` option performs an
        // explicit, opt-in cache clean before downloading.
        annotations(
            title = "Download",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    fn download(&self, Parameters(params): Parameters<DownloadParams>) -> Result<CallToolResult, rmcp::ErrorData> {
        use tree_sitter_language_pack::{clean_cache, download, download_all, download_group};

        if params.fresh.unwrap_or(false) {
            clean_cache().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        }

        let count = if params.all.unwrap_or(false) {
            download_all().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?
        } else if let Some(ref groups) = params.groups
            && !groups.is_empty()
        {
            let mut last = 0;
            for group in groups {
                last = download_group(group).map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
            }
            last
        } else if let Some(ref languages) = params.languages {
            if languages.is_empty() {
                return Err(rmcp::ErrorData::invalid_params(
                    "Provide at least one language name, one or more groups, or set all=true",
                    None,
                ));
            }
            let refs: Vec<&str> = languages.iter().map(String::as_str).collect();
            download(&refs).map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?
        } else {
            return Err(rmcp::ErrorData::invalid_params(
                "Provide languages, groups, or all=true",
                None,
            ));
        };

        let response = serde_json::to_string_pretty(&serde_json::json!({
            "languages_available": count,
        }))
        .unwrap_or_default();

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Return the effective parser cache directory.
    #[tool(
        description = "Return the effective parser cache directory path.",
        annotations(
            title = "Cache Directory",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    fn cache_dir(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        use tree_sitter_language_pack::cache_dir;

        let dir = cache_dir().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        let response = serde_json::to_string_pretty(&serde_json::json!({ "cache_dir": dir })).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Delete all cached parser libraries.
    #[tool(
        description = "Delete all cached parser libraries from the cache directory.",
        // Local filesystem deletion: destructive but idempotent (clearing twice yields the same state)
        // and no network access.
        annotations(
            title = "Clean Cache",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    fn clean_cache(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        use tree_sitter_language_pack::{cache_dir, clean_cache};

        clean_cache().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        let dir = cache_dir().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        let response = serde_json::to_string_pretty(&serde_json::json!({
            "cache_dir": dir,
            "status": "cleared",
        }))
        .unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(response)]))
    }
}

/// Context-free implementations of the resource/prompt/completion capabilities.
///
/// The `ServerHandler` trait methods are thin delegators to these so the logic
/// is unit-testable without constructing a live `RequestContext`.
impl TsPackMcp {
    const LANGUAGE_URI_PREFIX: &'static str = "ts-pack://language/";

    fn list_resources_inner(&self) -> ListResourcesResult {
        let mut available = RawResource::new("ts-pack://languages", "available-languages");
        available.title = Some("Available languages".to_string());
        available.description = Some("Every language available to this build of the pack.".to_string());
        available.mime_type = Some("application/json".to_string());

        let mut downloaded = RawResource::new("ts-pack://languages/downloaded", "downloaded-languages");
        downloaded.title = Some("Downloaded languages".to_string());
        downloaded.description = Some("Languages whose parser libraries are already cached locally.".to_string());
        downloaded.mime_type = Some("application/json".to_string());

        ListResourcesResult::with_all_items(vec![available.no_annotation(), downloaded.no_annotation()])
    }

    fn list_resource_templates_inner(&self) -> ListResourceTemplatesResult {
        let template = RawResourceTemplate {
            uri_template: "ts-pack://language/{name}".to_string(),
            name: "language-info".to_string(),
            title: Some("Language info".to_string()),
            description: Some("Per-language status: known, downloaded, and cache directory.".to_string()),
            mime_type: Some("application/json".to_string()),
            icons: None,
        };
        ListResourceTemplatesResult::with_all_items(vec![template.no_annotation()])
    }

    fn read_resource_inner(&self, uri: &str) -> Result<ReadResourceResult, rmcp::ErrorData> {
        use tree_sitter_language_pack::{available_languages, cache_dir, downloaded_languages, has_language};

        let json = match uri {
            "ts-pack://languages" => {
                let langs = available_languages();
                serde_json::json!({ "count": langs.len(), "languages": langs })
            }
            "ts-pack://languages/downloaded" => {
                let langs = downloaded_languages();
                serde_json::json!({ "count": langs.len(), "languages": langs })
            }
            other if other.starts_with(Self::LANGUAGE_URI_PREFIX) => {
                let name = &other[Self::LANGUAGE_URI_PREFIX.len()..];
                let cache = cache_dir().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
                serde_json::json!({
                    "language": name,
                    "known": has_language(name),
                    "downloaded": downloaded_languages().iter().any(|l| l == name),
                    "cache_dir": cache,
                })
            }
            _ => {
                return Err(rmcp::ErrorData::invalid_params(
                    format!("Unknown resource URI: {uri}"),
                    None,
                ));
            }
        };

        let text = serde_json::to_string_pretty(&json).unwrap_or_default();
        Ok(ReadResourceResult::new(vec![
            ResourceContents::text(text, uri).with_mime_type("application/json"),
        ]))
    }

    fn list_prompts_inner(&self) -> ListPromptsResult {
        let prompt = Prompt::new(
            "analyze-code",
            Some("Analyze a source file's structure, imports, exports, and symbols using the pack's tools."),
            Some(vec![
                PromptArgument::new("language")
                    .with_description("Language name (supports completion).")
                    .with_required(true),
                PromptArgument::new("focus")
                    .with_description("Optional area to emphasize, e.g. 'security' or 'public API'.")
                    .with_required(false),
            ]),
        );
        ListPromptsResult::with_all_items(vec![prompt])
    }

    fn get_prompt_inner(&self, name: &str, arguments: Option<JsonObject>) -> Result<GetPromptResult, rmcp::ErrorData> {
        if name != "analyze-code" {
            return Err(rmcp::ErrorData::invalid_params(format!("Unknown prompt: {name}"), None));
        }

        let args = arguments.unwrap_or_default();
        let language = args
            .get("language")
            .and_then(|v| v.as_str())
            .unwrap_or("the file's language");
        let focus = args.get("focus").and_then(|v| v.as_str()).unwrap_or("");

        let mut text = format!(
            "Analyze the following {language} source file. Call the 'process' tool with all=true to extract \
             its structure, imports, exports, symbols, and diagnostics, then summarize the design, key \
             entry points, and any issues."
        );
        if !focus.is_empty() {
            text.push_str(&format!(" Pay particular attention to: {focus}."));
        }

        let message = PromptMessage::new_text(PromptMessageRole::User, text);
        Ok(GetPromptResult::new(vec![message]).with_description("Code analysis workflow"))
    }

    fn complete_inner(
        &self,
        reference: &Reference,
        argument: &ArgumentInfo,
    ) -> Result<CompleteResult, rmcp::ErrorData> {
        use tree_sitter_language_pack::available_languages;

        let completes_language = match reference {
            Reference::Prompt(prompt) => prompt.name == "analyze-code" && argument.name == "language",
            Reference::Resource(resource) => {
                resource.uri.starts_with(Self::LANGUAGE_URI_PREFIX) && argument.name == "name"
            }
        };
        if !completes_language {
            return Ok(CompleteResult::default());
        }

        let prefix = argument.value.as_str();
        let mut values: Vec<String> = available_languages()
            .into_iter()
            .filter(|lang| lang.starts_with(prefix))
            .collect();
        let total = values.len() as u32;
        values.truncate(CompletionInfo::MAX_VALUES);
        let has_more = (values.len() as u32) < total;

        let completion = CompletionInfo::with_pagination(values, Some(total), has_more)
            .map_err(|e| rmcp::ErrorData::internal_error(e, None))?;
        Ok(CompleteResult::new(completion))
    }
}

#[tool_handler]
impl ServerHandler for TsPackMcp {
    fn get_info(&self) -> ServerInfo {
        // Advertise every capability this server implements: tools, readable
        // resources (the language catalog + per-language templates), prompts, and
        // argument completions for language names.
        let capabilities = ServerCapabilities::builder()
            .enable_tools()
            .enable_resources()
            .enable_prompts()
            .enable_completions()
            .build();

        let server_info = Implementation::new("ts-pack-mcp", env!("CARGO_PKG_VERSION"))
            .with_title("Tree-Sitter Language Pack MCP Server")
            .with_description(
                "MCP server for the tree-sitter language pack. \
                 Parse source code, run code-intelligence analysis, detect languages, \
                 list and download parsers, and manage the cache.",
            )
            .with_website_url("https://github.com/xberg-io/tree-sitter-language-pack");

        InitializeResult::new(capabilities)
            .with_server_info(server_info)
            .with_instructions(
                "Use 'parse' to get a syntax tree (sexp or JSON). \
                 Use 'process' to extract structure, imports, exports, symbols, comments, and diagnostics. \
                 Use 'detect_language' to identify a language from a file path or source snippet. \
                 Use 'list_languages' to see available, downloaded, or manifest languages. \
                 Use 'info' to check whether a specific language is downloaded. \
                 Use 'download' to fetch parser libraries (by name, groups, or all; fresh=true to re-fetch). \
                 Use 'cache_dir' to query the cache directory and 'clean_cache' to delete all cached parsers. \
                 Read 'ts-pack://languages' or the 'ts-pack://language/{name}' template for the catalog. \
                 Use the 'analyze-code' prompt for a ready-made analysis workflow.",
            )
    }

    /// List the readable resources: the available and downloaded language catalogs.
    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, rmcp::ErrorData> {
        Ok(self.list_resources_inner())
    }

    /// Expose the per-language resource template `ts-pack://language/{name}`.
    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, rmcp::ErrorData> {
        Ok(self.list_resource_templates_inner())
    }

    /// Read a resource: the language catalogs or a single `ts-pack://language/{name}` entry.
    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, rmcp::ErrorData> {
        self.read_resource_inner(&request.uri)
    }

    /// List the available prompts.
    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, rmcp::ErrorData> {
        Ok(self.list_prompts_inner())
    }

    /// Render the `analyze-code` prompt with the supplied arguments.
    async fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, rmcp::ErrorData> {
        self.get_prompt_inner(&request.name, request.arguments)
    }

    /// Complete language-name arguments for the `analyze-code` prompt and the
    /// `ts-pack://language/{name}` resource template.
    async fn complete(
        &self,
        request: CompleteRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CompleteResult, rmcp::ErrorData> {
        self.complete_inner(&request.r#ref, &request.argument)
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
            "cache_dir",
            "clean_cache",
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
        assert!(info.capabilities.tools.is_some(), "tools capability advertised");
        assert!(info.capabilities.resources.is_some(), "resources capability advertised");
        assert!(info.capabilities.prompts.is_some(), "prompts capability advertised");
        assert!(
            info.capabilities.completions.is_some(),
            "completions capability advertised"
        );
        assert!(info.instructions.is_some());
    }

    #[test]
    fn test_list_resources_exposes_catalogs() {
        let server = TsPackMcp::new();
        let result = server.list_resources_inner();
        let uris: Vec<&str> = result.resources.iter().map(|r| r.uri.as_str()).collect();
        assert!(
            uris.contains(&"ts-pack://languages"),
            "available catalog resource present"
        );
        assert!(
            uris.contains(&"ts-pack://languages/downloaded"),
            "downloaded catalog resource present"
        );
    }

    #[test]
    fn test_list_resource_templates_exposes_language_template() {
        let server = TsPackMcp::new();
        let result = server.list_resource_templates_inner();
        assert!(
            result
                .resource_templates
                .iter()
                .any(|t| t.uri_template == "ts-pack://language/{name}"),
            "per-language template present"
        );
    }

    #[test]
    fn test_read_resource_available_languages() {
        let server = TsPackMcp::new();
        let result = server
            .read_resource_inner("ts-pack://languages")
            .expect("read should succeed");
        let ResourceContents::TextResourceContents { text, .. } = &result.contents[0] else {
            panic!("expected text contents");
        };
        let parsed: serde_json::Value = serde_json::from_str(text).expect("valid JSON");
        // The CLI's dependency has no statically-compiled grammars, so the catalog may be empty
        // in unit tests; assert the shape and the count/array consistency instead.
        let count = parsed["count"].as_u64().expect("count is a number");
        let languages = parsed["languages"].as_array().expect("languages is an array");
        assert_eq!(count as usize, languages.len(), "count matches the array length");
    }

    #[test]
    fn test_read_resource_language_template() {
        let server = TsPackMcp::new();
        let result = server
            .read_resource_inner("ts-pack://language/python")
            .expect("read should succeed");
        let ResourceContents::TextResourceContents { text, .. } = &result.contents[0] else {
            panic!("expected text contents");
        };
        let parsed: serde_json::Value = serde_json::from_str(text).expect("valid JSON");
        assert_eq!(parsed["language"], "python");
        assert!(parsed["known"].is_boolean());
    }

    #[test]
    fn test_read_resource_unknown_uri_errors() {
        let server = TsPackMcp::new();
        assert!(server.read_resource_inner("ts-pack://nope").is_err());
    }

    #[test]
    fn test_list_prompts_exposes_analyze_code() {
        let server = TsPackMcp::new();
        let result = server.list_prompts_inner();
        assert!(result.prompts.iter().any(|p| p.name == "analyze-code"));
    }

    #[test]
    fn test_get_prompt_renders_language_and_focus() {
        let server = TsPackMcp::new();
        let mut args = serde_json::Map::new();
        args.insert("language".to_string(), serde_json::json!("rust"));
        args.insert("focus".to_string(), serde_json::json!("security"));
        let result = server
            .get_prompt_inner("analyze-code", Some(args))
            .expect("render should succeed");
        let PromptMessageContent::Text { text } = &result.messages[0].content else {
            panic!("expected text message");
        };
        assert!(text.contains("rust"), "language interpolated");
        assert!(text.contains("security"), "focus interpolated");
    }

    #[test]
    fn test_get_prompt_unknown_errors() {
        let server = TsPackMcp::new();
        assert!(server.get_prompt_inner("nope", None).is_err());
    }

    #[test]
    fn test_complete_language_prefix() {
        let server = TsPackMcp::new();
        let reference = Reference::Resource(ResourceReference {
            uri: "ts-pack://language/{name}".to_string(),
        });
        let argument = ArgumentInfo {
            name: "name".to_string(),
            value: "py".to_string(),
        };
        let result = server
            .complete_inner(&reference, &argument)
            .expect("complete should succeed");
        // The catalog may be empty in unit tests (no statically-compiled grammars), so assert the
        // filter invariant and that pagination metadata is populated rather than specific languages.
        assert!(
            result.completion.values.iter().all(|v| v.starts_with("py")),
            "all completions match the prefix"
        );
        assert!(result.completion.total.is_some(), "completion reports a total count");
    }

    #[test]
    fn test_complete_ignores_unrelated_reference() {
        let server = TsPackMcp::new();
        let reference = Reference::for_prompt("other");
        let argument = ArgumentInfo {
            name: "language".to_string(),
            value: "py".to_string(),
        };
        let result = server
            .complete_inner(&reference, &argument)
            .expect("complete should succeed");
        assert!(
            result.completion.values.is_empty(),
            "no completions for unrelated prompt"
        );
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
    fn test_cache_dir_returns_path() {
        let server = TsPackMcp::new();
        let result = server.cache_dir();
        assert!(result.is_ok());
        let call = result.unwrap();
        if let Some(content) = call.content.first()
            && let rmcp::model::RawContent::Text(text) = &content.raw
        {
            let parsed: serde_json::Value = serde_json::from_str(&text.text).expect("Should be valid JSON");
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
            groups: None,
            fresh: None,
        }));
        assert!(result.is_err());
    }
}
