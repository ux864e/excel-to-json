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
use output::Summary;
use tracing::info;

/// Run the full conversion pipeline: parse CLI args, load config,
/// walk directories, convert Excel files, emit results, and
/// return a summary of the outcome.
///
/// If stdin contains a valid JSON `StdinInput`, its `outputDir`
/// overrides the CLI `--output`. Config names and descriptions
/// are extracted from Excel sheet metadata.
///
/// Returns `Err` only for global failures (config parse, input dir
/// I/O errors). Per-file conversion errors are collected in the
/// returned `Summary`.
pub fn run() -> Result<Summary> {
    let args = cli::Args::parse();
    let mut cfg = config::load(&args)?;

    // If stdin provides an outputDir, override CLI --output.
    if let Some(stdin_cfg) = config::read_stdin_config() {
        info!(
            output_dir = %stdin_cfg.output_dir.display(),
            "Using stdin output directory"
        );
        cfg.output_dir = stdin_cfg.output_dir;
    }

    info!(input_dir = %cfg.input_dir.display(), "Starting conversion");

    let files = walker::collect_files(&cfg)?;
    info!(count = files.len(), "Found Excel files");

    // Convert all files, collecting per-file errors (never fails globally).
    let conv_summary = converter::convert_all(&files, &cfg);

    // Write JSON output files.
    let (file_summaries, write_errors) = output::emit_results(&conv_summary.results, &cfg);

    // Merge conversion errors and write errors.
    let all_errors: Vec<_> = conv_summary
        .errors
        .into_iter()
        .chain(write_errors)
        .collect();

    let has_results = !file_summaries.is_empty();
    let has_errors = !all_errors.is_empty();

    // "success" when at least one config was produced, or when there
    // was nothing to do (empty directory, no errors).
    // "error" only when ALL configs failed or ALL writes failed.
    let status = if has_results {
        "success"
    } else if has_errors {
        "error"
    } else {
        "success"
    };

    let summary = Summary {
        status: status.to_string(),
        files: file_summaries,
        errors: all_errors,
        warnings: conv_summary.warnings,
    };

    // Emit the single JSON summary line to stdout.
    output::emit_summary(&summary);

    info!("Conversion complete");
    Ok(summary)
}
