//! excel-to-json: Convert Excel files to JSON with config-driven mapping.
//!
//! This library provides the core logic for directory traversal,
//! Excel parsing, config-driven field mapping, and JSON output.
//! The binary entry point is in `main.rs`.

pub mod cli;
pub mod config;
pub mod converter;
pub mod error;
pub mod mapping;
pub mod output;
pub mod walker;

use anyhow::Result;
use clap::Parser;
use tracing::info;

/// Run the full conversion pipeline: parse CLI args, load config,
/// walk directories, convert Excel files, and emit results.
pub fn run() -> Result<()> {
    let args = cli::Args::parse();
    let cfg = config::load(&args)?;

    info!(input_dir = %cfg.input_dir.display(), "Starting conversion");

    let files = walker::collect_files(&cfg)?;
    info!(count = files.len(), "Found Excel files");

    let results = converter::convert_all(&files, &cfg)?;
    output::emit_results(&results, &cfg)?;

    info!("Conversion complete");
    Ok(())
}
