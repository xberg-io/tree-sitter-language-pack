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
    /// Named groups to download. The manifest currently defines exactly one group,
    /// `"all"`; call `manifest_groups` to enumerate the names that actually exist.
    pub groups: Option<Vec<String>>,
    /// Clean the cache before downloading for a fresh fetch. Default: false.
    pub fresh: Option<bool>,
}

/// Structured result of the `detect_language` tool.
#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct DetectResult {
    /// Detected language name, or `null` when detection failed.
    pub language: Option<String>,
    /// The path echoed back from the request, when one was supplied.
    pub path: Option<String>,
}

/// Structured result of the `list_languages` tool.
#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct ListLanguagesResult {
    /// The queried set: `"available"`, `"downloaded"`, or `"manifest"`.
    pub source: String,
    /// The substring filter applied, when one was supplied.
    pub filter: Option<String>,
    /// Number of languages returned after filtering.
    pub count: usize,
    /// The matching language names.
    pub languages: Vec<String>,
}

/// Structured result of the `info` tool.
#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct InfoResult {
    /// The language name inspected.
    pub language: String,
    /// Whether the language is known to this build of the pack.
    pub known: bool,
    /// Whether the language's parser library is cached locally.
    pub downloaded: bool,
    /// The effective parser cache directory.
    pub cache_dir: String,
}

/// Structured result of the `download` tool.
#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct DownloadResult {
    /// Number of languages available after the download completed.
    pub languages_available: usize,
}

/// Structured result of the `cache_dir` tool.
#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct CacheDirResult {
    /// The effective parser cache directory.
    pub cache_dir: String,
}

/// Structured result of the `clean_cache` tool.
#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct CleanCacheResult {
    /// The cache directory that was cleared.
    pub cache_dir: String,
    /// Outcome status, always `"cleared"` on success.
    pub status: String,
}

/// Maximum accepted length of a language or group name.
const MAX_NAME_LEN: usize = 64;

/// Allowlist pattern for language and group names.
const NAME_PATTERN: &str = "^[a-z0-9_]+$";

/// Reject any name that is not on the `^[a-z0-9_]+$` allowlist.
///
/// Mirrors `validate_definition_keys` in `ts-pack-core/build.rs`: these names become
/// filesystem paths and shared-library names, so an MCP client must not be able to
/// smuggle separators or traversal segments through them. ~keep
fn validate_name(kind: &str, name: &str) -> Result<(), rmcp::ErrorData> {
    let valid = !name.is_empty()
        && name.len() <= MAX_NAME_LEN
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
    if valid {
        return Ok(());
    }

    let shown: String = name.chars().take(MAX_NAME_LEN).collect();
    Err(rmcp::ErrorData::invalid_params(
        format!("Invalid {kind} name '{shown}': must match {NAME_PATTERN} and be at most {MAX_NAME_LEN} characters"),
        None,
    ))
}

use rmcp::{
    RoleServer, ServerHandler,
    handler::server::{
        router::tool::ToolRouter,
        wrapper::{Json, Parameters},
    },
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router,
};

/// MCP server exposing the ts-pack API surface.
#[derive(Clone)]
pub struct TsPackMcp {
    // ~keep `#[tool_router]` reads this through generated `ServerHandler` delegation.
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
    async fn parse(&self, Parameters(params): Parameters<ParseParams>) -> Result<CallToolResult, rmcp::ErrorData> {
        validate_name("language", &params.language)?;

        // ~keep Grammar loading and parsing are CPU-bound and touch the filesystem;
        // ~keep run them on the blocking pool so the async runtime stays responsive.
        tokio::task::spawn_blocking(move || {
            use tree_sitter_language_pack::get_parser;

            let mut parser = get_parser(&params.language)
                .map_err(|e| rmcp::ErrorData::invalid_params(format!("Language error: {e}"), None))?;

            let tree = parser
                .parse_bytes(params.source.as_bytes())
                .ok_or_else(|| rmcp::ErrorData::internal_error("Parser returned no tree", None))?;

            let sexp = tree.root_node().to_sexp();
            let has_errors = tree.root_node().has_error();
            let value = serde_json::json!({
                "language": &params.language,
                "sexp": &sexp,
                "has_errors": has_errors,
            });

            // ~keep `format` controls the human-readable text block for legacy clients;
            // ~keep `structured_content` always carries the typed result for modern ones.
            let text = match params.format.as_deref().unwrap_or("sexp") {
                "json" => serde_json::to_string_pretty(&value).unwrap_or_default(),
                _ => sexp,
            };

            let mut result = CallToolResult::structured(value);
            result.content = vec![ContentBlock::text(text)];
            Ok(result)
        })
        .await
        .map_err(|e| rmcp::ErrorData::internal_error(format!("parse worker task failed: {e}"), None))?
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
    async fn process(&self, Parameters(params): Parameters<ProcessParams>) -> Result<CallToolResult, rmcp::ErrorData> {
        validate_name("language", &params.language)?;

        // ~keep The code-intelligence pipeline is CPU-bound; keep it off the async runtime.
        tokio::task::spawn_blocking(move || {
            use tree_sitter_language_pack::{ProcessConfig, process};

            let mut config = ProcessConfig::new(params.language);

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

            let value = serde_json::to_value(&result)
                .map_err(|e| rmcp::ErrorData::internal_error(format!("serialize failed: {e}"), None))?;
            let pretty = serde_json::to_string_pretty(&value).unwrap_or_default();

            let mut call = CallToolResult::structured(value);
            call.content = vec![ContentBlock::text(pretty)];
            Ok(call)
        })
        .await
        .map_err(|e| rmcp::ErrorData::internal_error(format!("process worker task failed: {e}"), None))?
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
    async fn detect_language(
        &self,
        Parameters(params): Parameters<DetectLanguageParams>,
    ) -> Result<Json<DetectResult>, rmcp::ErrorData> {
        let result = tokio::task::spawn_blocking(move || {
            use tree_sitter_language_pack::{detect_language_from_content, detect_language_from_path};

            let detected = params
                .path
                .as_deref()
                .and_then(detect_language_from_path)
                .or_else(|| params.content.as_deref().and_then(detect_language_from_content));

            DetectResult {
                language: detected.map(str::to_string),
                path: params.path,
            }
        })
        .await
        .map_err(|e| rmcp::ErrorData::internal_error(format!("detect worker task failed: {e}"), None))?;

        Ok(Json(result))
    }

    /// List available, downloaded, or manifest languages.
    ///
    /// Pass `source` = `"available"` (default), `"downloaded"`, or `"manifest"`.
    #[tool(
        description = "List languages. source: 'available' (default), 'downloaded', or 'manifest'. Optional substring filter.",
        // ~keep `open_world_hint = true`: the `manifest` source fetches the remote download manifest.
        annotations(
            title = "List Languages",
            read_only_hint = true,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn list_languages(
        &self,
        Parameters(params): Parameters<ListLanguagesParams>,
    ) -> Result<Json<ListLanguagesResult>, rmcp::ErrorData> {
        // ~keep The `manifest` source fetches the remote download manifest; enumerating
        // ~keep downloaded languages hits the filesystem. Offload to the blocking pool.
        tokio::task::spawn_blocking(move || {
            use tree_sitter_language_pack::{available_languages, downloaded_languages, manifest_languages};

            let source = params.source.as_deref().unwrap_or("available").to_string();
            let langs: Vec<String> = match source.as_str() {
                "downloaded" => downloaded_languages(),
                "manifest" => manifest_languages().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?,
                _ => available_languages(),
            };

            let languages: Vec<String> = match params.filter {
                Some(ref f) => langs.into_iter().filter(|l| l.contains(f.as_str())).collect(),
                None => langs,
            };

            Ok(Json(ListLanguagesResult {
                count: languages.len(),
                source,
                filter: params.filter,
                languages,
            }))
        })
        .await
        .map_err(|e| rmcp::ErrorData::internal_error(format!("list worker task failed: {e}"), None))?
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
    async fn info(&self, Parameters(params): Parameters<InfoParams>) -> Result<Json<InfoResult>, rmcp::ErrorData> {
        validate_name("language", &params.language)?;

        tokio::task::spawn_blocking(move || {
            use tree_sitter_language_pack::{cache_dir, downloaded_languages, has_language};

            let known = has_language(&params.language);
            let is_downloaded = downloaded_languages().contains(&params.language);
            let cache = cache_dir().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

            Ok(Json(InfoResult {
                language: params.language,
                known,
                downloaded: is_downloaded,
                cache_dir: cache,
            }))
        })
        .await
        .map_err(|e| rmcp::ErrorData::internal_error(format!("info worker task failed: {e}"), None))?
    }

    /// Download parser libraries for specific languages, a group, or all.
    ///
    /// Set `all: true` to download everything, `group` to download a named group,
    /// or pass `languages` with a list of names.
    #[tool(
        description = "Download parser libraries from the remote registry. Pass languages list, groups, or all=true. \
                       Set fresh=true to clean the cache first.",
        // ~keep Network fetch is additive/idempotent; only `fresh` performs explicit cache cleanup.
        annotations(
            title = "Download",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn download(
        &self,
        Parameters(params): Parameters<DownloadParams>,
    ) -> Result<Json<DownloadResult>, rmcp::ErrorData> {
        for language in params.languages.iter().flatten() {
            validate_name("language", language)?;
        }
        for group in params.groups.iter().flatten() {
            validate_name("group", group)?;
        }

        // ~keep Downloads perform network I/O and cache writes; keep them off the runtime.
        tokio::task::spawn_blocking(move || {
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

            Ok(Json(DownloadResult {
                languages_available: count,
            }))
        })
        .await
        .map_err(|e| rmcp::ErrorData::internal_error(format!("download worker task failed: {e}"), None))?
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
    async fn cache_dir(&self) -> Result<Json<CacheDirResult>, rmcp::ErrorData> {
        use tree_sitter_language_pack::cache_dir;

        let dir = cache_dir().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
        Ok(Json(CacheDirResult { cache_dir: dir }))
    }

    /// Delete all cached parser libraries.
    #[tool(
        description = "Delete all cached parser libraries from the cache directory.",
        // ~keep Cache deletion is destructive but idempotent and has no network access.
        annotations(
            title = "Clean Cache",
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn clean_cache(&self) -> Result<Json<CleanCacheResult>, rmcp::ErrorData> {
        // ~keep Cache deletion is blocking filesystem I/O; keep it off the async runtime.
        tokio::task::spawn_blocking(move || {
            use tree_sitter_language_pack::{cache_dir, clean_cache};

            clean_cache().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
            let dir = cache_dir().map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;
            Ok(Json(CleanCacheResult {
                cache_dir: dir,
                status: "cleared".to_string(),
            }))
        })
        .await
        .map_err(|e| rmcp::ErrorData::internal_error(format!("clean_cache worker task failed: {e}"), None))?
    }
}

/// Context-free implementations of the resource/prompt/completion capabilities.
///
/// The `ServerHandler` trait methods are thin delegators to these so the logic
/// is unit-testable without constructing a live `RequestContext`.
impl TsPackMcp {
    const LANGUAGE_URI_PREFIX: &'static str = "ts-pack://language/";

    fn list_resources_inner(&self) -> ListResourcesResult {
        let available = Resource::new("ts-pack://languages", "available-languages")
            .with_title("Available languages")
            .with_description("Every language available to this build of the pack.")
            .with_mime_type("application/json");

        let downloaded = Resource::new("ts-pack://languages/downloaded", "downloaded-languages")
            .with_title("Downloaded languages")
            .with_description("Languages whose parser libraries are already cached locally.")
            .with_mime_type("application/json");

        ListResourcesResult::with_all_items(vec![available, downloaded])
    }

    fn list_resource_templates_inner(&self) -> ListResourceTemplatesResult {
        let template = ResourceTemplate::new("ts-pack://language/{name}", "language-info")
            .with_title("Language info")
            .with_description("Per-language status: known, downloaded, and cache directory.")
            .with_mime_type("application/json");
        ListResourceTemplatesResult::with_all_items(vec![template])
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
                validate_name("language", name)?;
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

        let message = PromptMessage::new_text(Role::User, text);
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
            _ => false,
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
    fn get_info(&self) -> ServerConfig {
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
    ) -> Result<ReadResourceResponse, rmcp::ErrorData> {
        self.read_resource_inner(&request.uri).map(Into::into)
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
    ) -> Result<GetPromptResponse, rmcp::ErrorData> {
        self.get_prompt_inner(&request.name, request.arguments).map(Into::into)
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

/// Environment variable carrying the bearer token the HTTP transport requires.
const AUTH_TOKEN_ENV: &str = "TS_PACK_MCP_AUTH_TOKEN";

/// Environment variable carrying extra comma-separated allowed `Origin` values.
const ALLOWED_ORIGINS_ENV: &str = "TS_PACK_MCP_ALLOWED_ORIGINS";

/// Request guard for the HTTP transport: `Origin` allowlist plus optional bearer auth.
#[derive(Clone)]
struct HttpGuard {
    allowed_origins: std::sync::Arc<Vec<String>>,
    auth_token: Option<std::sync::Arc<String>>,
}

impl HttpGuard {
    /// Build a guard whose allowlist covers the loopback origins for `port`, plus any
    /// origin listed in `extra_origins` (comma-separated).
    fn new(port: u16, extra_origins: Option<&str>, auth_token: Option<String>) -> Self {
        let mut allowed_origins = vec![
            format!("http://127.0.0.1:{port}"),
            format!("http://localhost:{port}"),
            format!("http://[::1]:{port}"),
        ];
        if let Some(extra) = extra_origins {
            allowed_origins.extend(
                extra
                    .split(',')
                    .map(str::trim)
                    .filter(|origin| !origin.is_empty())
                    .map(str::to_string),
            );
        }

        Self {
            allowed_origins: std::sync::Arc::new(allowed_origins),
            auth_token: auth_token
                .map(|token| token.trim().to_string())
                .filter(|token| !token.is_empty())
                .map(std::sync::Arc::new),
        }
    }

    fn from_env(port: u16) -> Self {
        Self::new(
            port,
            std::env::var(ALLOWED_ORIGINS_ENV).ok().as_deref(),
            std::env::var(AUTH_TOKEN_ENV).ok(),
        )
    }

    fn is_origin_allowed(&self, origin: &str) -> bool {
        self.allowed_origins.iter().any(|allowed| allowed == origin)
    }

    /// Accept the request when no token is configured, otherwise require an exact match.
    fn is_token_valid(&self, presented: Option<&str>) -> bool {
        let Some(expected) = self.auth_token.as_deref() else {
            return true;
        };
        let Some(presented) = presented else {
            return false;
        };
        constant_time_eq(expected.as_bytes(), presented.as_bytes())
    }
}

/// Compare two byte strings without an early exit on the first differing byte. ~keep
fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut difference = 0u8;
    for (a, b) in left.iter().zip(right.iter()) {
        difference |= a ^ b;
    }
    difference == 0
}

/// Reject cross-origin and unauthenticated requests before they reach the MCP service.
///
/// A browser page on another origin can otherwise reach a loopback-bound server via DNS
/// rebinding; the `Origin` allowlist is the defence the MCP transport spec prescribes.
/// Non-browser clients send no `Origin` at all, so an absent header is not a rejection. ~keep
async fn guard_http_request(
    axum::extract::State(guard): axum::extract::State<HttpGuard>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::response::IntoResponse;

    let origin = request
        .headers()
        .get(axum::http::header::ORIGIN)
        .map(|value| value.to_str().unwrap_or_default().to_string());
    if let Some(ref origin) = origin
        && !guard.is_origin_allowed(origin)
    {
        tracing::warn!(%origin, "rejected MCP HTTP request: Origin is not in the allowlist");
        return (axum::http::StatusCode::FORBIDDEN, "Origin not allowed").into_response();
    }

    let token = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::to_string);
    if !guard.is_token_valid(token.as_deref()) {
        tracing::warn!("rejected MCP HTTP request: missing or invalid bearer token");
        return (axum::http::StatusCode::UNAUTHORIZED, "Missing or invalid bearer token").into_response();
    }

    next.run(request).await
}

/// Run the MCP server with the given transport.
pub async fn run(args: McpArgs) -> Result<(), String> {
    use rmcp::ServiceExt;

    // ~keep Applying config before serving sets cache dir and pre-warms configured language groups.
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

            let guard = HttpGuard::from_env(args.port);
            if !addr.ip().is_loopback() {
                tracing::warn!(
                    address = %addr,
                    auth_env = AUTH_TOKEN_ENV,
                    origins_env = ALLOWED_ORIGINS_ENV,
                    "MCP HTTP transport is bound to a non-loopback address; configure a bearer token \
                     and the Origin allowlist before exposing it"
                );
            }

            let router = axum::Router::new()
                .nest_service("/mcp", http_service)
                .layer(axum::middleware::from_fn_with_state(guard.clone(), guard_http_request));

            tracing::info!(
                auth_required = guard.auth_token.is_some(),
                allowed_origins = guard.allowed_origins.len(),
                "starting ts-pack MCP server (HTTP transport) on {addr}"
            );
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_accept_names_on_the_allowlist() {
        for name in ["python", "c_sharp", "html", "tsx", "php2"] {
            assert!(validate_name("language", name).is_ok(), "'{name}' should be accepted");
        }
    }

    #[test]
    fn should_reject_names_outside_the_allowlist() {
        for name in [
            "",
            "Python",
            "../../etc/passwd",
            "py thon",
            "py/thon",
            "py-thon",
            "py.thon",
            "py\0thon",
            "py;rm -rf /",
        ] {
            assert!(validate_name("language", name).is_err(), "'{name}' should be rejected");
        }
        let too_long = "a".repeat(MAX_NAME_LEN + 1);
        assert!(validate_name("language", &too_long).is_err(), "over-long name rejected");
    }

    #[tokio::test]
    async fn should_reject_parse_with_a_traversal_language_name() {
        let server = TsPackMcp::new();
        let result = server
            .parse(Parameters(ParseParams {
                source: "x = 1".to_string(),
                language: "../../etc/passwd".to_string(),
                format: None,
            }))
            .await;
        assert!(result.is_err(), "traversal language name is rejected");
    }

    #[test]
    fn should_reject_resource_uri_with_an_invalid_language_name() {
        let server = TsPackMcp::new();
        assert!(server.read_resource_inner("ts-pack://language/../secrets").is_err());
    }

    #[test]
    fn should_allow_only_loopback_origins_by_default() {
        let guard = HttpGuard::new(8011, None, None);
        assert!(guard.is_origin_allowed("http://127.0.0.1:8011"));
        assert!(guard.is_origin_allowed("http://localhost:8011"));
        assert!(!guard.is_origin_allowed("http://evil.example.com"));
        assert!(!guard.is_origin_allowed("http://127.0.0.1:9999"));
    }

    #[test]
    fn should_allow_extra_configured_origins() {
        let guard = HttpGuard::new(8011, Some("https://ide.example.com, https://other.example.com"), None);
        assert!(guard.is_origin_allowed("https://ide.example.com"));
        assert!(guard.is_origin_allowed("https://other.example.com"));
        assert!(!guard.is_origin_allowed("https://nope.example.com"));
    }

    #[test]
    fn should_accept_any_token_when_none_is_configured() {
        let guard = HttpGuard::new(8011, None, None);
        assert!(guard.is_token_valid(None));
        assert!(guard.is_token_valid(Some("anything")));
    }

    #[test]
    fn should_require_the_exact_token_when_one_is_configured() {
        let guard = HttpGuard::new(8011, None, Some("s3cret".to_string()));
        assert!(guard.is_token_valid(Some("s3cret")));
        assert!(!guard.is_token_valid(Some("s3cre")));
        assert!(!guard.is_token_valid(Some("wrong")));
        assert!(!guard.is_token_valid(None));
    }

    #[test]
    fn should_treat_a_blank_configured_token_as_unset() {
        let guard = HttpGuard::new(8011, None, Some("   ".to_string()));
        assert!(guard.auth_token.is_none());
        assert!(guard.is_token_valid(None));
    }

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
        let ContentBlock::Text(text_content) = &result.messages[0].content else {
            panic!("expected text message");
        };
        let text = &text_content.text;
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
        let reference = Reference::for_resource("ts-pack://language/{name}");
        let argument = ArgumentInfo::new("name", "py");
        let result = server
            .complete_inner(&reference, &argument)
            .expect("complete should succeed");
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
        let argument = ArgumentInfo::new("language", "py");
        let result = server
            .complete_inner(&reference, &argument)
            .expect("complete should succeed");
        assert!(
            result.completion.values.is_empty(),
            "no completions for unrelated prompt"
        );
    }

    #[tokio::test]
    async fn test_list_languages_available() {
        let server = TsPackMcp::new();
        let result = server
            .list_languages(Parameters(ListLanguagesParams {
                source: None,
                filter: None,
            }))
            .await
            .expect("list should succeed");
        assert_eq!(result.0.source, "available");
        assert_eq!(
            result.0.count,
            result.0.languages.len(),
            "count matches the list length"
        );
    }

    #[tokio::test]
    async fn test_list_languages_with_filter() {
        let server = TsPackMcp::new();
        let result = server
            .list_languages(Parameters(ListLanguagesParams {
                source: Some("available".to_string()),
                filter: Some("python".to_string()),
            }))
            .await
            .expect("list should succeed");
        assert_eq!(result.0.filter.as_deref(), Some("python"));
        assert_eq!(result.0.source, "available");
        assert!(
            result.0.languages.iter().all(|l| l.contains("python")),
            "every returned language matches the filter"
        );
    }

    #[tokio::test]
    async fn test_cache_dir_returns_path() {
        let server = TsPackMcp::new();
        let result = server.cache_dir().await.expect("cache_dir should succeed");
        assert!(!result.0.cache_dir.is_empty(), "cache directory path is non-empty");
    }

    #[tokio::test]
    async fn test_detect_language_from_path() {
        let server = TsPackMcp::new();
        let result = server
            .detect_language(Parameters(DetectLanguageParams {
                path: Some("main.py".to_string()),
                content: None,
            }))
            .await
            .expect("detect should succeed");
        assert_eq!(result.0.language.as_deref(), Some("python"));
    }

    #[tokio::test]
    async fn test_download_requires_params() {
        let server = TsPackMcp::new();
        let result = server
            .download(Parameters(DownloadParams {
                languages: None,
                all: None,
                groups: None,
                fresh: None,
            }))
            .await;
        assert!(result.is_err());
    }
}
