//! `ts-pack` — command-line interface for the tree-sitter language pack.
//!
//! Download parsers, list and inspect supported languages, parse source files,
//! run the code-intelligence pipeline, manage the cache, generate shell completions,
//! and scaffold project configuration.

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
        /// Download language groups (comma-separated: web,systems,scripting,data,jvm,functional)
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
        /// Cache directory
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

fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Download {
            languages,
            all,
            groups,
            fresh,
        } => {
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
                    groups: Some(groups.clone()),
                };
                tree_sitter_language_pack::init(&config).map_err(|e| e.to_string())?;
                println!("Downloaded groups: {}", groups.join(", "));
            } else if !languages.is_empty() {
                let refs: Vec<&str> = languages.iter().map(String::as_str).collect();
                let count = tree_sitter_language_pack::download(&refs).map_err(|e| e.to_string())?;
                println!("Ensured {count} languages.");
            } else {
                // No flags: try config discovery
                match PackConfig::discover() {
                    Some(config) => {
                        tree_sitter_language_pack::init(&config).map_err(|e| e.to_string())?;
                        println!("Initialized from discovered config.");
                    }
                    None => {
                        return Err("No languages specified and no language-pack.toml found. \
                             Use --all, --groups, or specify language names."
                            .to_string());
                    }
                }
            }
        }

        Commands::Clean { force } => {
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
        }

        Commands::List {
            downloaded,
            manifest,
            filter,
        } => {
            let langs: Vec<String> = if downloaded {
                tree_sitter_language_pack::downloaded_languages()
            } else if manifest {
                tree_sitter_language_pack::manifest_languages().map_err(|e| e.to_string())?
            } else {
                tree_sitter_language_pack::available_languages()
            };

            let filtered: Vec<&String> = if let Some(ref f) = filter {
                langs.iter().filter(|l| l.contains(f.as_str())).collect()
            } else {
                langs.iter().collect()
            };

            for lang in &filtered {
                println!("{lang}");
            }
            println!("\n{} language(s)", filtered.len());
        }

        Commands::Info { language } => {
            let known = tree_sitter_language_pack::has_language(&language);
            let downloaded = tree_sitter_language_pack::downloaded_languages();
            let is_downloaded = downloaded.contains(&language);
            let cache = PathBuf::from(tree_sitter_language_pack::cache_dir().map_err(|e| e.to_string())?);

            println!("Language:    {language}");
            println!("Known:       {known}");
            println!("Downloaded:  {is_downloaded}");
            if is_downloaded {
                let lib_name = format!("tree_sitter_{language}");
                let (prefix, ext) = if cfg!(target_os = "macos") {
                    ("lib", "dylib")
                } else if cfg!(target_os = "windows") {
                    ("", "dll")
                } else {
                    ("lib", "so")
                };
                println!(
                    "Cache path:  {}",
                    cache.join(format!("{prefix}{lib_name}.{ext}")).display()
                );
            } else {
                println!("Cache dir:   {}", cache.display());
            }
        }

        Commands::Parse { file, language, format } => {
            let source = read_source(&file)?;
            let lang = match language {
                Some(ref l) => l.as_str().to_string(),
                None => detect_language(&file)
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
                    // Emit a simple JSON representation of the sexp
                    let sexp = tree.root_node().to_sexp();
                    let json = serde_json::json!({
                        "language": lang,
                        "sexp": sexp,
                        "has_errors": tree.root_node().has_error(),
                    });
                    println!("{}", serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?);
                }
            }
        }

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
        } => {
            let source_bytes = read_source(&file)?;
            let source = String::from_utf8(source_bytes).map_err(|e| format!("File is not valid UTF-8: {e}"))?;

            let lang = match language {
                Some(ref l) => l.as_str().to_string(),
                None => {
                    if file == "-" {
                        return Err("Cannot detect language from stdin. Use --language to specify.".to_string());
                    }
                    detect_language(&file)
                        .ok_or_else(|| format!("Cannot detect language for '{}'. Use --language to specify.", file))?
                        .to_string()
                }
            };

            let mut config = ProcessConfig::new(lang);

            if all {
                config = config.all();
            } else {
                // Apply explicit flags; when none are given, defaults kick in (structure+imports+exports=true)
                let any_explicit = structure || imports || exports || comments || symbols || docstrings || diagnostics;
                if any_explicit {
                    config.structure = structure;
                    config.imports = imports;
                    config.exports = exports;
                    config.comments = comments;
                    config.symbols = symbols;
                    config.docstrings = docstrings || all;
                    config.diagnostics = diagnostics || all;
                }
            }

            if let Some(sz) = chunk_size {
                config = config.with_chunking(sz);
            }

            let result = process(&source, &config).map_err(|e| e.to_string())?;
            let json = serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?;
            println!("{json}");
        }

        Commands::CacheDir => {
            let dir = tree_sitter_language_pack::cache_dir().map_err(|e| e.to_string())?;
            println!("{dir}");
        }

        Commands::Init { cache_dir, languages } => {
            let config = PackConfig {
                cache_dir: cache_dir.as_deref().map(std::path::PathBuf::from),
                languages: if languages.is_empty() {
                    None
                } else {
                    Some(languages.clone())
                },
                groups: None,
            };

            // Write language-pack.toml
            let toml_content = {
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
            };

            let path = std::path::Path::new("language-pack.toml");
            std::fs::write(path, &toml_content).map_err(|e| format!("Failed to write language-pack.toml: {e}"))?;
            println!("Created language-pack.toml");

            // Run init with the config to trigger downloads
            if config.languages.is_some() || config.groups.is_some() {
                tree_sitter_language_pack::init(&config).map_err(|e| e.to_string())?;
            }
        }

        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let bin_name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, bin_name, &mut io::stdout());
        }
    }

    Ok(())
}

fn main() {
    #[cfg(unix)]
    reset_sigpipe();
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

#[cfg(unix)]
#[allow(unsafe_code, reason = "restore SIGPIPE default disposition for Unix CLI semantics")]
fn reset_sigpipe() {
    // Restore the default SIGPIPE disposition so the kernel terminates the
    // process when stdout is closed by a consumer like `head` or `grep -m`.
    // Rust's default is to ignore SIGPIPE and surface broken-pipe writes as
    // panics, which is wrong for a Unix filter-style CLI.
    // SAFETY: single-threaded at startup; libc::signal is async-signal-safe.
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}
