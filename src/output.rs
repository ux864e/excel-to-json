//! Output module: write JSON files and emit IPC messages on stdout/stderr.

use crate::config::Config;
use crate::converter::ConversionResult;
use anyhow::{Context, Result};
use serde::Serialize;
use std::io::{self, Write};
use std::path::Path;
use tracing::debug;

/// IPC message format — one JSON line per event, written to stdout.
///
/// Other processes can parse these line-delimited JSON messages
/// to track conversion progress.
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum IpcMessage {
    #[serde(rename = "progress")]
    Progress { file: String, status: String },
    #[serde(rename = "done")]
    Done { file: String, rows: usize },
    #[serde(rename = "error")]
    Error { file: String, message: String },
}

impl IpcMessage {
    /// Emit a progress message to stdout.
    pub fn progress(path: &Path, status: &str) {
        let msg = Self::Progress {
            file: path.display().to_string(),
            status: status.to_string(),
        };
        emit(&msg);
    }

    /// Emit a done message to stdout with the converted row count.
    pub fn done(path: &Path, rows: usize) {
        let msg = Self::Done {
            file: path.display().to_string(),
            rows,
        };
        emit(&msg);
    }

    /// Emit an error message to stderr.
    pub fn error_msg(path: &Path, message: &str) {
        let msg = Self::Error {
            file: path.display().to_string(),
            message: message.to_string(),
        };
        let line = serde_json::to_string(&msg).unwrap_or_default();
        eprintln!("{}", line);
    }
}

/// Write a single IPC line to stdout. Panics on serialization failure
/// (should be infallible for our enum).
fn emit(msg: &IpcMessage) {
    let line = serde_json::to_string(msg).expect("IPC message serialization should not fail");
    println!("{}", line);
    // Flush to ensure parent process receives the message immediately.
    let _ = io::stdout().flush();
}

/// Write all conversion results to JSON files on disk.
///
/// Output structure mirrors the input directory structure, with
/// `.json` files replacing the original Excel files.
pub fn emit_results(results: &[ConversionResult], cfg: &Config) -> Result<()> {
    std::fs::create_dir_all(&cfg.output_dir)
        .with_context(|| format!("Failed to create output dir: {}", cfg.output_dir.display()))?;

    for result in results {
        let output_path = cfg.output_dir.join(format!("{}.json", result.stem));
        debug!(path = %output_path.display(), "Writing output");

        let json = if cfg.pretty {
            serde_json::to_string_pretty(&result.sheets)?
        } else {
            serde_json::to_string(&result.sheets)?
        };

        std::fs::write(&output_path, json)
            .with_context(|| format!("Failed to write: {}", output_path.display()))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_message_serialization() {
        let msg = IpcMessage::Progress {
            file: "/tmp/test.xlsx".to_string(),
            status: "converting".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["type"], "progress");
        assert_eq!(parsed["file"], "/tmp/test.xlsx");
    }

    #[test]
    fn test_ipc_done_message() {
        let msg = IpcMessage::Done {
            file: "/tmp/test.xlsx".to_string(),
            rows: 42,
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["type"], "done");
        assert_eq!(parsed["rows"], 42);
    }
}
