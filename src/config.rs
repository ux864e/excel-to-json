//! Configuration loading: merge TOML config file with CLI overrides.

use crate::cli::Args;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

/// Merged runtime configuration, combining TOML file settings
/// with CLI argument overrides.
#[derive(Debug, Clone)]
pub struct Config {
    /// Root directory to scan for Excel files.
    pub input_dir: PathBuf,
    /// Directory to write JSON output files.
    pub output_dir: PathBuf,
    /// Whether to recurse into subdirectories.
    pub recursive: bool,
    /// Whether to pretty-print JSON output.
    pub pretty: bool,
    /// Field mapping rules from config file.
    pub mapping: MappingConfig,
}

/// Top-level structure of the TOML config file.
#[derive(Debug, Clone, Deserialize)]
pub struct ConfigFile {
    /// Optional: override input directory.
    pub input_dir: Option<PathBuf>,
    /// Optional: override output directory.
    pub output_dir: Option<PathBuf>,
    /// Optional: override recursive flag.
    pub recursive: Option<bool>,
    /// Optional: override pretty-print flag.
    pub pretty: Option<bool>,
    /// Field mapping configuration.
    #[serde(default)]
    pub mapping: MappingConfig,
}

/// Configuration-driven field mapping rules.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct MappingConfig {
    /// Maps source column names to output JSON key names.
    /// Example: `{ "姓名": "name", "年龄": "age" }`
    #[serde(default)]
    pub column_map: std::collections::HashMap<String, String>,
    /// Columns to exclude from output.
    #[serde(default)]
    pub exclude_columns: Vec<String>,
    /// Nested object paths: source column → dot-separated JSON path.
    /// Example: `{ "城市": "address.city" }`
    #[serde(default)]
    pub nested_paths: std::collections::HashMap<String, String>,
}

/// Configuration passed via stdin (pipe) from the parent process.
#[derive(Debug, Clone, Deserialize)]
pub struct StdinInput {
    /// Absolute path for the output directory.
    #[serde(rename = "outputDir")]
    pub output_dir: PathBuf,
}

/// Attempt to read and parse a `StdinInput` from stdin.
///
/// Returns `None` if stdin is a terminal (no piped input) or if
/// the input is empty or cannot be parsed.
pub fn read_stdin_config() -> Option<StdinInput> {
    use std::io::{IsTerminal, Read};

    let stdin = std::io::stdin();
    if stdin.is_terminal() {
        return None;
    }

    let mut buf = String::new();
    stdin.lock().read_to_string(&mut buf).ok()?;
    if buf.trim().is_empty() {
        return None;
    }

    serde_json::from_str(&buf).ok()
}

/// Load and merge configuration from TOML file and CLI arguments.
/// CLI arguments take precedence over file settings.
pub fn load(args: &Args) -> Result<Config> {
    let config_path = args.resolve_config_path();

    // Load TOML file if it exists; use defaults otherwise.
    let file_cfg = if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config: {}", config_path.display()))?;
        toml::from_str::<ConfigFile>(&content)
            .with_context(|| format!("Failed to parse config: {}", config_path.display()))?
    } else {
        ConfigFile {
            input_dir: None,
            output_dir: None,
            recursive: None,
            pretty: None,
            mapping: MappingConfig::default(),
        }
    };

    Ok(Config {
        input_dir: args.input_dir.clone(),
        output_dir: file_cfg.output_dir.unwrap_or(args.output_dir.clone()),
        recursive: file_cfg.recursive.unwrap_or(args.recursive),
        pretty: file_cfg.pretty.unwrap_or(args.pretty),
        mapping: file_cfg.mapping,
    })
}
