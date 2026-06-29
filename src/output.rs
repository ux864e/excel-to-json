//! Output module: write JSON files and emit a single summary line on stdout.

use crate::config::Config;
use crate::converter::{ConversionResult, ErrorEntry, WarningEntry};
use serde::Serialize;
use std::io::{self, Write};
use tracing::debug;

/// Convert a camelCase string to kebab-case.
///
/// Inserts a hyphen before each ASCII uppercase letter and lowercases it.
/// Strings without uppercase letters are returned unchanged.
///
/// # Examples
/// - `"aaaBbbCcc"` → `"aaa-bbb-ccc"`
/// - `"simple"` → `"simple"`
/// - `"myConfigName"` → `"my-config-name"`
fn camel_to_kebab(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for ch in s.chars() {
        if ch.is_ascii_uppercase() {
            result.push('-');
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
    }
    result
}

/// Single-line JSON summary emitted on stdout at the end of a run.
#[derive(Debug, Serialize)]
pub struct Summary {
    pub status: String,
    pub files: Vec<FileSummary>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorEntry>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<WarningEntry>,
}

/// A successfully converted config entry in the summary.
#[derive(Debug, Serialize)]
pub struct FileSummary {
    /// Config name from the Excel sheet metadata.
    #[serde(rename = "configName")]
    pub config_name: String,
    /// Output file path relative to the configured directory.
    pub path: String,
    /// Description from the Excel sheet metadata.
    pub description: String,
    /// Number of valid data rows written to the output file.
    #[serde(rename = "validRows")]
    pub valid_rows: usize,
    /// Total data rows in the sheet (excluding metadata rows).
    #[serde(rename = "inputRows")]
    pub input_rows: usize,
    /// Rows skipped: comment rows (// prefix) + duplicate ids.
    #[serde(rename = "skippedRows")]
    pub skipped_rows: usize,
    /// Rows with an invalid (unparseable) id.
    #[serde(rename = "failedRows")]
    pub failed_rows: usize,
}

/// Write the final summary as a single JSON line to stdout.
///
/// This is the only output on stdout — the calling process should
/// parse this line to determine the outcome.
pub fn emit_summary(summary: &Summary) {
    let line = serde_json::to_string(summary).expect("Summary serialization should not fail");
    println!("{}", line);
    let _ = io::stdout().flush();
}

/// Write all conversion results to JSON files on disk.
///
/// Each `ConfigOutput` from each sheet produces one file at
/// `<output_dir>/<config_name>.json`.
///
/// Returns structured summaries for the final output, and any
/// per-file write errors that occurred (never fails globally).
pub fn emit_results(
    results: &[ConversionResult],
    cfg: &Config,
) -> (Vec<FileSummary>, Vec<ErrorEntry>) {
    let output_dir = &cfg.output_dir;
    let mut file_summaries = Vec::new();
    let mut errors = Vec::new();

    for result in results {
        for config in &result.configs {
            let filename = format!("{}.json", camel_to_kebab(&config.config_name));
            let output_path = output_dir.join(&filename);

            // Ensure the output directory exists.
            if let Err(e) = std::fs::create_dir_all(output_dir) {
                errors.push(ErrorEntry {
                    file: Some(config.config_name.clone()),
                    message: format!(
                        "Failed to create output directory '{}': {:#}",
                        output_dir.display(),
                        e
                    ),
                });
                continue;
            }

            // Build the output JSON:
            // { "configName": "...", "description": "...", "items": [ rows ] }
            let data = serde_json::json!({
                "configName": &config.config_name,
                "description": &config.description,
                "items": &config.rows
            });

            let json = match if cfg.pretty {
                serde_json::to_string_pretty(&data)
            } else {
                serde_json::to_string(&data)
            } {
                Ok(j) => j,
                Err(e) => {
                    errors.push(ErrorEntry {
                        file: Some(config.config_name.clone()),
                        message: format!("JSON serialization error: {:#}", e),
                    });
                    continue;
                }
            };

            if let Err(e) = std::fs::write(&output_path, &json) {
                errors.push(ErrorEntry {
                    file: Some(config.config_name.clone()),
                    message: format!("Failed to write output file: {:#}", e),
                });
                continue;
            }

            debug!(path = %output_path.display(), "Wrote output");
            file_summaries.push(FileSummary {
                config_name: config.config_name.clone(),
                path: filename,
                description: config.description.clone(),
                valid_rows: config.valid_rows,
                input_rows: config.input_rows,
                skipped_rows: config.skipped_rows,
                failed_rows: config.failed_rows,
            });
        }
    }

    (file_summaries, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MappingConfig;
    use crate::converter::{ConfigOutput, ConversionResult, ErrorEntry};

    #[test]
    fn test_summary_serialization_success() {
        let summary = Summary {
            status: "success".to_string(),
            files: vec![FileSummary {
                config_name: "pet-types".to_string(),
                path: "pet-types.json".to_string(),
                description: "Available pet types".to_string(),
                valid_rows: 3,
                input_rows: 5,
                skipped_rows: 1,
                failed_rows: 1,
            }],
            errors: vec![],
            warnings: vec![],
        };
        let json = serde_json::to_string(&summary).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["status"], "success");
        assert_eq!(parsed["files"][0]["configName"], "pet-types");
        assert_eq!(parsed["files"][0]["path"], "pet-types.json");
        assert_eq!(parsed["files"][0]["description"], "Available pet types");
        assert_eq!(parsed["files"][0]["validRows"], 3);
        assert_eq!(parsed["files"][0]["inputRows"], 5);
        assert_eq!(parsed["files"][0]["skippedRows"], 1);
        assert_eq!(parsed["files"][0]["failedRows"], 1);
        // Empty errors/warnings should be omitted.
        assert!(parsed.get("errors").is_none());
        assert!(parsed.get("warnings").is_none());
    }

    #[test]
    fn test_summary_serialization_error() {
        let summary = Summary {
            status: "error".to_string(),
            files: vec![],
            errors: vec![ErrorEntry {
                file: None,
                message: "Input directory not found".to_string(),
            }],
            warnings: vec![],
        };
        let json = serde_json::to_string(&summary).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["status"], "error");
        assert_eq!(parsed["errors"][0]["message"], "Input directory not found");
        assert!(parsed["errors"][0].get("file").is_none());
        assert!(parsed.get("warnings").is_none());
    }

    #[test]
    fn test_emit_results_with_config_outputs() {
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let output_dir = tmp.path().join("output");
        let cfg = Config {
            input_dir: tmp.path().to_path_buf(),
            output_dir: output_dir.clone(),
            recursive: false,
            pretty: false,
            mapping: MappingConfig::default(),
        };

        let results = vec![ConversionResult {
            relative_path: std::path::PathBuf::from("data.xlsx"),
            stem: "data".to_string(),
            configs: vec![
                ConfigOutput {
                    config_name: "pet-types".to_string(),
                    description: "Available pet types".to_string(),
                    rows: vec![serde_json::json!({"id": 1, "name": "Fido"})],
                    input_rows: 1,
                    valid_rows: 1,
                    skipped_rows: 0,
                    failed_rows: 0,
                },
                ConfigOutput {
                    config_name: "food-types".to_string(),
                    description: "Available food types".to_string(),
                    rows: vec![serde_json::json!({"id": 1, "name": "Kibble"})],
                    input_rows: 1,
                    valid_rows: 1,
                    skipped_rows: 0,
                    failed_rows: 0,
                },
            ],
            total_rows: 2,
        }];

        let (summaries, errors) = emit_results(&results, &cfg);
        assert!(errors.is_empty());
        assert_eq!(summaries.len(), 2);

        assert_eq!(summaries[0].config_name, "pet-types");
        assert_eq!(summaries[0].path, "pet-types.json");
        assert_eq!(summaries[0].description, "Available pet types");
        assert_eq!(summaries[1].config_name, "food-types");
        assert_eq!(summaries[1].path, "food-types.json");

        // Verify output files exist directly in the output directory.
        assert!(output_dir.join("pet-types.json").exists());
        assert!(output_dir.join("food-types.json").exists());
    }

    #[test]
    fn test_camel_to_kebab() {
        assert_eq!(camel_to_kebab("aaaBbbCcc"), "aaa-bbb-ccc");
        assert_eq!(camel_to_kebab("simple"), "simple");
        assert_eq!(camel_to_kebab("myConfigName"), "my-config-name");
        assert_eq!(camel_to_kebab("alreadyKebab"), "already-kebab");
        assert_eq!(camel_to_kebab(""), "");
        assert_eq!(camel_to_kebab("alllowercase"), "alllowercase");
        assert_eq!(camel_to_kebab("a"), "a");
        assert_eq!(camel_to_kebab("ABC"), "-a-b-c");
    }

    #[test]
    fn test_emit_results_camelcase_config_name() {
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let output_dir = tmp.path().join("output");
        let cfg = Config {
            input_dir: tmp.path().to_path_buf(),
            output_dir: output_dir.clone(),
            recursive: false,
            pretty: false,
            mapping: MappingConfig::default(),
        };

        let results = vec![ConversionResult {
            relative_path: std::path::PathBuf::from("data.xlsx"),
            stem: "data".to_string(),
            configs: vec![ConfigOutput {
                config_name: "myConfigName".to_string(),
                description: "Test config".to_string(),
                rows: vec![serde_json::json!({"id": 1, "name": "Fido"})],
                input_rows: 1,
                valid_rows: 1,
                skipped_rows: 0,
                failed_rows: 0,
            }],
            total_rows: 1,
        }];

        let (summaries, errors) = emit_results(&results, &cfg);
        assert!(errors.is_empty());
        assert_eq!(summaries.len(), 1);

        // config_name in summary stays as the original camelCase.
        assert_eq!(summaries[0].config_name, "myConfigName");
        // Output path uses kebab-case filename.
        assert_eq!(summaries[0].path, "my-config-name.json");
        // The actual file on disk uses kebab-case.
        assert!(output_dir.join("my-config-name.json").exists());
    }
}
