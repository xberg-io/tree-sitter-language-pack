#![allow(dead_code)]

use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default, rename = "language-pack")]
    pub language_pack: LanguagePackConfig,
    #[serde(default)]
    pub languages: LanguagesConfig,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LanguagePackConfig {
    pub cache_dir: Option<String>,
    pub definitions: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LanguagesConfig {
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, Error> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Discover config file from standard locations.
    /// Returns Ok(Some(config)) if found and parsed, Ok(None) if not found,
    /// and Err if found but failed to parse.
    pub fn discover() -> Result<Option<Self>, Error> {
        let search_paths = config_search_paths();
        for path in search_paths {
            if path.exists() {
                let config = Self::load(&path)
                    .map_err(|e| Error::Config(format!("Failed to parse config at {}: {e}", path.display())))?;
                return Ok(Some(config));
            }
        }
        Ok(None)
    }
}

fn config_search_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // CWD and parent directories (stop at filesystem root or after 10 levels)
    if let Ok(cwd) = std::env::current_dir() {
        let mut dir = Some(cwd.as_path());
        let mut depth = 0;
        while let Some(d) = dir {
            if depth > 10 {
                break;
            }
            paths.push(d.join("language-pack.toml"));
            dir = d.parent();
            depth += 1;
        }
    }

    // XDG config
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        paths.push(PathBuf::from(xdg).join("tree-sitter-language-pack").join("config.toml"));
    }

    // Unix home
    if let Ok(home) = std::env::var("HOME") {
        paths.push(
            PathBuf::from(home)
                .join(".config")
                .join("tree-sitter-language-pack")
                .join("config.toml"),
        );
    }

    // Windows home
    if let Ok(appdata) = std::env::var("APPDATA") {
        paths.push(
            PathBuf::from(appdata)
                .join("tree-sitter-language-pack")
                .join("config.toml"),
        );
    }

    if let Ok(userprofile) = std::env::var("USERPROFILE") {
        paths.push(
            PathBuf::from(userprofile)
                .join(".config")
                .join("tree-sitter-language-pack")
                .join("config.toml"),
        );
    }

    paths
}
