//! Directory walker: discover Excel files in a directory tree.

use crate::config::Config;
use anyhow::Result;
use std::path::PathBuf;
use tracing::debug;

/// Supported file extensions for Excel files.
const SUPPORTED_EXTENSIONS: &[&str] = &["xlsx", "xls", "xlsb", "ods", "csv"];

/// Represents a discovered Excel file to be converted.
#[derive(Debug, Clone)]
pub struct ExcelFile {
    /// Absolute path to the file.
    pub path: PathBuf,
    /// File name without extension (used for output naming).
    pub stem: String,
    /// Lowercase file extension.
    pub extension: String,
}

/// Recursively (or non-recursively) collect all Excel files
/// under the configured input directory.
pub fn collect_files(cfg: &Config) -> Result<Vec<ExcelFile>> {
    let mut files = Vec::new();
    walk_dir(&cfg.input_dir, cfg.recursive, &mut files)?;
    Ok(files)
}

fn walk_dir(dir: &PathBuf, recursive: bool, files: &mut Vec<ExcelFile>) -> Result<()> {
    let entries = std::fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() && recursive {
            walk_dir(&path, recursive, files)?;
        } else if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_lower = ext.to_lowercase();
                if SUPPORTED_EXTENSIONS.contains(&ext_lower.as_str()) {
                    let stem = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    debug!(path = %path.display(), "Found Excel file");
                    files.push(ExcelFile {
                        path: path.clone(),
                        stem,
                        extension: ext_lower,
                    });
                }
            }
        }
    }

    Ok(())
}
