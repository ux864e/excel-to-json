//! Directory walker: discover Excel files in a directory tree.

use crate::config::Config;
use anyhow::Result;
use std::path::{Path, PathBuf};
use tracing::debug;

/// Supported file extensions for Excel files.
const SUPPORTED_EXTENSIONS: &[&str] = &["xlsx", "xls", "xlsb", "ods"];

/// Represents a discovered Excel file to be converted.
#[derive(Debug, Clone)]
pub struct ExcelFile {
    /// Absolute path to the file.
    pub path: PathBuf,
    /// Relative path from the input directory (e.g. "reports/sales.xlsx").
    pub relative_path: PathBuf,
    /// File name without extension (used for output naming).
    pub stem: String,
    /// Lowercase file extension.
    pub extension: String,
}

/// Recursively (or non-recursively) collect all Excel files
/// under the configured input directory.
pub fn collect_files(cfg: &Config) -> Result<Vec<ExcelFile>> {
    let mut files = Vec::new();
    walk_dir(&cfg.input_dir, cfg.recursive, &mut files, &cfg.input_dir)?;
    Ok(files)
}

fn walk_dir(
    dir: &Path,
    recursive: bool,
    files: &mut Vec<ExcelFile>,
    base_dir: &Path,
) -> Result<()> {
    let entries = std::fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() && recursive {
            walk_dir(&path, recursive, files, base_dir)?;
        } else if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_lower = ext.to_lowercase();
                if SUPPORTED_EXTENSIONS.contains(&ext_lower.as_str()) {
                    let stem = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let relative_path = path
                        .strip_prefix(base_dir)
                        .unwrap_or(std::path::Path::new(""))
                        .to_path_buf();

                    debug!(path = %path.display(), "Found Excel file");
                    files.push(ExcelFile {
                        path: path.clone(),
                        relative_path,
                        stem,
                        extension: ext_lower,
                    });
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Create a dummy Excel file inside a subdirectory and verify
    /// that relative_path is computed correctly.
    #[test]
    fn test_relative_path_in_subdirectory() {
        let tmp = TempDir::new().unwrap();
        let base = tmp.path().to_path_buf();

        // Create subdirectory structure: base/sub/file.xlsx
        let sub_dir = base.join("sub");
        std::fs::create_dir_all(&sub_dir).unwrap();
        let file_path = sub_dir.join("data.xlsx");
        std::fs::File::create(&file_path).unwrap();

        let cfg = Config {
            input_dir: base.clone(),
            output_dir: base.join("output"),
            recursive: true,
            pretty: false,
            mapping: crate::config::MappingConfig::default(),
        };

        let files = collect_files(&cfg).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].relative_path, PathBuf::from("sub/data.xlsx"));
        assert_eq!(files[0].stem, "data");
        assert_eq!(files[0].extension, "xlsx");
    }

    /// A file directly in the input dir gets a relative path of just the filename.
    #[test]
    fn test_relative_path_root_level() {
        let tmp = TempDir::new().unwrap();
        let base = tmp.path().to_path_buf();

        // Create file directly in base: base/file.xlsx
        let file_path = base.join("file.xlsx");
        std::fs::File::create(&file_path).unwrap();

        let cfg = Config {
            input_dir: base.clone(),
            output_dir: base.join("output"),
            recursive: false,
            pretty: false,
            mapping: crate::config::MappingConfig::default(),
        };

        let files = collect_files(&cfg).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].relative_path, PathBuf::from("file.xlsx"));
    }
}
