//! `ts-pack` — command-line interface for the tree-sitter language pack.
//!
//! Download parsers, list and inspect supported languages, parse source files,
//! run the code-intelligence pipeline, manage the cache, generate shell completions,
//! and scaffold project configuration.

// This binary's command results and prompts ARE its stdout/stderr output contract; diagnostics
// route through `tracing` (see commands/mcp.rs). Result output opts back in here crate-wide. ~keep
#![allow(clippy::print_stdout, clippy::print_stderr)]

mod commands;

use clap::{CommandFactory, Parser, Subcommand};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process;
use tree_sitter_language_pack::{PackConfig, ProcessConfig, get_parser, process};

#[derive(Parser)]
#[command(name = "ts-pack", about = "Tree-sitter language pack CLI")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download parser libraries
    Download {
        /// Languages to download (omit for all or use config)
        languages: Vec<String>,
        /// Download all available languages
        #[arg(long)]
        all: bool,
        /// Download language groups (comma-separated). The manifest currently defines
        /// exactly one group, `all`; enumerate the real names with `manifest_groups()`.
        #[arg(long, value_delimiter = ',')]
        groups: Vec<String>,
        /// Clean cache before downloading (fresh download)
        #[arg(long)]
        fresh: bool,
    },
    /// Remove all cached parser libraries
    Clean {
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },
    /// List available languages
    List {
        /// Show only downloaded/cached languages
        #[arg(long)]
        downloaded: bool,
        /// Show all languages from remote manifest
        #[arg(long)]
        manifest: bool,
        /// Filter languages by substring
        #[arg(long)]
        filter: Option<String>,
    },
    /// Show details about a language
    Info {
        /// Language name
        language: String,
    },
    /// Parse a file and output the syntax tree
    Parse {
        /// File to parse (use "-" for stdin)
        file: String,
        /// Language (auto-detected from extension if omitted)
        #[arg(long, short)]
        language: Option<String>,
        /// Output format
        #[arg(long, short, default_value = "sexp")]
        format: ParseFormat,
    },
    /// Run code intelligence pipeline
    Process {
        /// File to process (use "-" for stdin)
        file: String,
        /// Language (auto-detected from extension if omitted)
        #[arg(long, short)]
        language: Option<String>,
        /// Enable all analysis features
        #[arg(long)]
        all: bool,
        /// Extract structure (functions, classes)
        #[arg(long)]
        structure: bool,
        /// Extract imports
        #[arg(long)]
        imports: bool,
        /// Extract exports
        #[arg(long)]
        exports: bool,
        /// Extract comments
        #[arg(long)]
        comments: bool,
        /// Extract symbols
        #[arg(long)]
        symbols: bool,
        /// Extract docstrings
        #[arg(long)]
        docstrings: bool,
        /// Include diagnostics
        #[arg(long)]
        diagnostics: bool,
        /// Maximum chunk size in bytes
        #[arg(long)]
        chunk_size: Option<usize>,
    },
    /// Print the effective cache directory
    CacheDir,
    /// Create a language-pack.toml config file
    Init {
        /// Base directory for the parser cache (a versioned subdirectory is created under it)
        #[arg(long)]
        cache_dir: Option<String>,
        /// Languages to include (comma-separated)
        #[arg(long, value_delimiter = ',')]
        languages: Vec<String>,
    },
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: clap_complete::Shell,
    },
    /// Start the MCP (Model Context Protocol) server
    #[cfg(feature = "mcp")]
    Mcp(commands::mcp::McpArgs),
}

#[derive(Clone, clap::ValueEnum)]
enum ParseFormat {
    Sexp,
    Json,
}

fn detect_language(path: &str) -> Option<&'static str> {
    tree_sitter_language_pack::detect_language_from_path(path)
}

fn read_source(file: &str) -> Result<Vec<u8>, String> {
    if file == "-" {
        let mut buf = Vec::new();
        io::stdin()
            .read_to_end(&mut buf)
            .map_err(|e| format!("Failed to read stdin: {e}"))?;
        Ok(buf)
    } else {
        std::fs::read(file).map_err(|e| format!("Failed to read '{}': {e}", file))
    }
}

struct ProcessFlags {
    all: bool,
    structure: bool,
    imports: bool,
    exports: bool,
    comments: bool,
    symbols: bool,
    docstrings: bool,
    diagnostics: bool,
    chunk_size: Option<usize>,
}

impl ProcessFlags {
    fn any_explicit(&self) -> bool {
        self.structure
            || self.imports
            || self.exports
            || self.comments
            || self.symbols
            || self.docstrings
            || self.diagnostics
    }

    fn into_config(self, lang: String) -> ProcessConfig {
        let mut config = ProcessConfig::new(lang);

        if self.all {
            config = config.all();
        } else if self.any_explicit() {
            config.structure = self.structure;
            config.imports = self.imports;
            config.exports = self.exports;
            config.comments = self.comments;
            config.symbols = self.symbols;
            config.docstrings = self.docstrings;
            config.diagnostics = self.diagnostics;
        }

        if let Some(sz) = self.chunk_size {
            config = config.with_chunking(sz);
        }
        config
    }
}

fn run_download(languages: &[String], all: bool, groups: &[String], fresh: bool) -> Result<(), String> {
    if fresh {
        tree_sitter_language_pack::clean_cache().map_err(|e| e.to_string())?;
        println!("Cache cleared.");
    }

    if all {
        let count = tree_sitter_language_pack::download_all().map_err(|e| e.to_string())?;
        println!("Ensured {count} languages.");
    } else if !groups.is_empty() {
        let config = PackConfig {
            cache_dir: None,
            languages: None,
            groups: Some(groups.to_vec()),
        };
        tree_sitter_language_pack::init(&config).map_err(|e| e.to_string())?;
        println!("Downloaded groups: {}", groups.join(", "));
    } else if !languages.is_empty() {
        let refs: Vec<&str> = languages.iter().map(String::as_str).collect();
        let count = tree_sitter_language_pack::download(&refs).map_err(|e| e.to_string())?;
        println!("Ensured {count} languages.");
    } else {
        download_discovered()?;
    }
    Ok(())
}

fn download_discovered() -> Result<(), String> {
    match PackConfig::try_discover() {
        Ok(Some(config)) => {
            tree_sitter_language_pack::init(&config).map_err(|e| e.to_string())?;
            println!("Initialized from discovered config.");
            Ok(())
        }
        Ok(None) => Err("No languages specified and no language-pack.toml found. \
             Use --all, --groups, or specify language names."
            .to_string()),
        Err(error) => Err(format!("Failed to load the discovered config: {error}")),
    }
}

fn run_clean(force: bool) -> Result<(), String> {
    if !force {
        print!("This will delete all cached parser libraries. Continue? [y/N] ");
        io::stdout().flush().ok();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).map_err(|e| e.to_string())?;
        let trimmed = answer.trim().to_lowercase();
        if trimmed != "y" && trimmed != "yes" {
            println!("Aborted.");
            return Ok(());
        }
    }
    tree_sitter_language_pack::clean_cache().map_err(|e| e.to_string())?;
    println!("Cache cleared.");
    Ok(())
}

fn run_list(downloaded: bool, manifest: bool, filter: Option<&str>) -> Result<(), String> {
    let langs: Vec<String> = if downloaded {
        tree_sitter_language_pack::downloaded_languages()
    } else if manifest {
        tree_sitter_language_pack::manifest_languages().map_err(|e| e.to_string())?
    } else {
        tree_sitter_language_pack::available_languages()
    };

    let filtered: Vec<&String> = if let Some(f) = filter {
        langs.iter().filter(|l| l.contains(f)).collect()
    } else {
        langs.iter().collect()
    };

    for lang in &filtered {
        println!("{lang}");
    }
    println!("\n{} language(s)", filtered.len());
    Ok(())
}

fn run_info(language: &str) -> Result<(), String> {
    let known = tree_sitter_language_pack::has_language(language);
    let downloaded = tree_sitter_language_pack::downloaded_languages();
    let is_downloaded = downloaded.iter().any(|l| l == language);
    let cache = PathBuf::from(tree_sitter_language_pack::cache_dir().map_err(|e| e.to_string())?);

    println!("Language:    {language}");
    println!("Known:       {known}");
    println!("Downloaded:  {is_downloaded}");
    if is_downloaded {
        let lib_name = tree_sitter_language_pack::registry::library_file_name(language);
        println!("Cache path:  {}", cache.join(lib_name).display());
    } else {
        println!("Cache dir:   {}", cache.display());
    }
    Ok(())
}

fn run_parse(file: &str, language: Option<String>, format: &ParseFormat) -> Result<(), String> {
    let source = read_source(file)?;
    let lang = match language {
        Some(l) => l,
        None => detect_language(file)
            .ok_or_else(|| format!("Cannot detect language for '{}'. Use --language.", file))?
            .to_string(),
    };

    let mut parser = get_parser(&lang).map_err(|e| e.to_string())?;
    let tree = parser.parse_bytes(&source).ok_or("Failed to parse source")?;

    match format {
        ParseFormat::Sexp => {
            println!("{}", tree.root_node().to_sexp());
        }
        ParseFormat::Json => {
            let sexp = tree.root_node().to_sexp();
            let json = serde_json::json!({
                "language": lang,
                "sexp": sexp,
                "has_errors": tree.root_node().has_error(),
            });
            println!("{}", serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?);
        }
    }
    Ok(())
}

fn run_process(file: &str, language: Option<String>, flags: ProcessFlags) -> Result<(), String> {
    let source_bytes = read_source(file)?;
    let source = String::from_utf8(source_bytes).map_err(|e| format!("File is not valid UTF-8: {e}"))?;

    let lang = match language {
        Some(l) => l,
        None => {
            if file == "-" {
                return Err("Cannot detect language from stdin. Use --language to specify.".to_string());
            }
            detect_language(file)
                .ok_or_else(|| format!("Cannot detect language for '{}'. Use --language to specify.", file))?
                .to_string()
        }
    };

    let config = flags.into_config(lang);
    let result = process(&source, &config).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?;
    println!("{json}");
    Ok(())
}

fn run_cache_dir() -> Result<(), String> {
    let dir = tree_sitter_language_pack::cache_dir().map_err(|e| e.to_string())?;
    println!("{dir}");
    Ok(())
}

fn init_toml_content(config: &PackConfig) -> String {
    let mut lines = Vec::new();
    if let Some(ref dir) = config.cache_dir {
        lines.push(format!("cache_dir = {:?}", dir.display().to_string()));
    }
    if let Some(ref langs) = config.languages {
        let quoted: Vec<String> = langs.iter().map(|l| format!("{l:?}")).collect();
        lines.push(format!("languages = [{}]", quoted.join(", ")));
    }
    if lines.is_empty() {
        "# language-pack.toml\n# languages = [\"python\", \"rust\"]\n".to_string()
    } else {
        lines.join("\n") + "\n"
    }
}

fn run_init(cache_dir: Option<String>, languages: Vec<String>) -> Result<(), String> {
    let config = PackConfig {
        cache_dir: cache_dir.as_deref().map(PathBuf::from),
        languages: if languages.is_empty() { None } else { Some(languages) },
        groups: None,
    };

    let toml_content = init_toml_content(&config);
    let path = std::path::Path::new("language-pack.toml");
    std::fs::write(path, &toml_content).map_err(|e| format!("Failed to write language-pack.toml: {e}"))?;
    println!("Created language-pack.toml");

    if config.languages.is_some() || config.groups.is_some() {
        tree_sitter_language_pack::init(&config).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn run_completions(shell: clap_complete::Shell) -> Result<(), String> {
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();
    clap_complete::generate(shell, &mut cmd, bin_name, &mut io::stdout());
    Ok(())
}

#[cfg(feature = "mcp")]
fn run_mcp(args: commands::mcp::McpArgs) -> Result<(), String> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("Failed to build Tokio runtime: {e}"))?
        .block_on(commands::mcp::run(args))
}

fn run() -> Result<(), String> {
    match Cli::parse().command {
        Commands::Download {
            languages,
            all,
            groups,
            fresh,
        } => run_download(&languages, all, &groups, fresh),
        Commands::Clean { force } => run_clean(force),
        Commands::List {
            downloaded,
            manifest,
            filter,
        } => run_list(downloaded, manifest, filter.as_deref()),
        Commands::Info { language } => run_info(&language),
        Commands::Parse { file, language, format } => run_parse(&file, language, &format),
        Commands::Process {
            file,
            language,
            all,
            structure,
            imports,
            exports,
            comments,
            symbols,
            docstrings,
            diagnostics,
            chunk_size,
        } => run_process(
            &file,
            language,
            ProcessFlags {
                all,
                structure,
                imports,
                exports,
                comments,
                symbols,
                docstrings,
                diagnostics,
                chunk_size,
            },
        ),
        Commands::CacheDir => run_cache_dir(),
        Commands::Init { cache_dir, languages } => run_init(cache_dir, languages),
        Commands::Completions { shell } => run_completions(shell),
        #[cfg(feature = "mcp")]
        Commands::Mcp(args) => run_mcp(args),
    }
}

/// Install the process-wide tracing subscriber for every command.
///
/// Diagnostics from the library and CLI surface on stderr so machine-readable
/// result output on stdout stays clean. `RUST_LOG` overrides the default filter.
fn init_tracing() {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();
}

fn main() {
    #[cfg(unix)]
    reset_sigpipe();
    init_tracing();
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

#[cfg(unix)]
#[allow(unsafe_code, reason = "restore SIGPIPE default disposition for Unix CLI semantics")]
fn reset_sigpipe() {
    // ~keep Restore default SIGPIPE so closed stdout terminates Unix filter-style CLI processes.
    // ~keep SAFETY: single-threaded at startup; libc::signal is async-signal-safe.
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}
