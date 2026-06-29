//! CLI argument definitions using clap derive.

use clap::Parser;
use std::path::PathBuf;

/// Convert Excel files (.xlsx, .csv) to JSON with config-driven mapping.
#[derive(Parser, Debug)]
#[command(
    name = "excel-to-json",
    version,
    about = "Convert Excel files to JSON with config-driven mapping",
    long_about = "Traverse directories, parse Excel files, apply config-driven field mappings, and output JSON."
)]
pub struct Args {
    /// Input directory to scan for Excel files (default: current directory)
    #[arg(short = 'i', long = "input", default_value = ".")]
    pub input_dir: PathBuf,

    /// Output directory for JSON files (default: ./output)
    #[arg(short = 'o', long = "output", default_value = "./output")]
    pub output_dir: PathBuf,

    /// Path to config file (default: excel-to-json.toml in input directory)
    #[arg(short = 'c', long = "config")]
    pub config_path: Option<PathBuf>,

    /// Recursively traverse subdirectories
    #[arg(short = 'r', long = "recursive", default_value = "true", action = clap::ArgAction::Set)]
    pub recursive: bool,

    /// Pretty-print JSON output
    #[arg(short = 'p', long = "pretty", default_value = "false", action = clap::ArgAction::Set)]
    pub pretty: bool,

    /// Verbosity level (-v, -vv, -vvv)
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    pub verbose: u8,
}

impl Args {
    /// Resolve the effective config path: explicit --config flag, or
    /// search in the input directory for `excel-to-json.toml`.
    pub fn resolve_config_path(&self) -> PathBuf {
        self.config_path
            .clone()
            .unwrap_or_else(|| self.input_dir.join("excel-to-json.toml"))
    }
}
