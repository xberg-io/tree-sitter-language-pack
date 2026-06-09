#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LanguageDefinition {
    pub repo: String,
    #[serde(default)]
    pub rev: Option<String>,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub directory: Option<String>,
    #[serde(default)]
    pub generate: Option<bool>,
    #[serde(default)]
    pub abi_version: Option<u32>,
    #[serde(default)]
    pub extensions: Vec<String>,
    /// Override for the C symbol name when it differs from the language name.
    #[serde(default)]
    pub c_symbol: Option<String>,
    /// Known ambiguous extensions mapped to the other languages they could belong to.
    /// Key: extension, Value: list of alternative language names.
    /// Example: `{"m": ["matlab"]}` on the `objc` definition means `.m` could also be MATLAB.
    #[serde(default)]
    pub ambiguous: BTreeMap<String, Vec<String>>,
}

pub type LanguageDefinitions = BTreeMap<String, LanguageDefinition>;

pub fn load_definitions(json: &str) -> Result<LanguageDefinitions, serde_json::Error> {
    serde_json::from_str(json)
}
