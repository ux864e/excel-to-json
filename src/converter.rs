//! Core conversion logic: Excel → JSON via calamine.

use crate::config::Config;
use crate::error::EError;
use crate::mapping;
use crate::walker::ExcelFile;
use anyhow::{Context, Result};
use calamine::{Data, Reader, open_workbook_auto};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Per-file or global error for the output summary.
#[derive(Debug, Serialize)]
pub struct ErrorEntry {
    /// Present for per-file errors, absent for global errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    pub message: String,
}

/// Non-fatal warning for the output summary.
#[derive(Debug, Serialize)]
pub struct WarningEntry {
    pub file: String,
    pub message: String,
}

/// Output produced from a single sheet (tab) in an Excel file.
/// Each sheet becomes its own config file.
#[derive(Debug)]
pub struct ConfigOutput {
    /// Config name from Row 0, Col B of the sheet.
    pub config_name: String,
    /// Description from Row 1, Col B of the sheet.
    pub description: String,
    /// Converted data rows (JSON objects with id field).
    pub rows: Vec<Value>,
    /// Total data rows parsed from the sheet (rows after metadata).
    pub input_rows: usize,
    /// Rows successfully included in the output.
    pub valid_rows: usize,
    /// Rows skipped: comment rows (// prefix) + duplicate ids.
    pub skipped_rows: usize,
    /// Rows with an invalid (unparseable) id.
    pub failed_rows: usize,
}

/// Result of converting a single Excel file.
#[derive(Debug)]
pub struct ConversionResult {
    /// Relative path from the input directory (e.g. "reports/sales.xlsx").
    pub relative_path: PathBuf,
    /// Source file stem (for output naming).
    pub stem: String,
    /// Config outputs produced from sheets in this file.
    pub configs: Vec<ConfigOutput>,
    /// Total row count across all configs.
    pub total_rows: usize,
}

/// Aggregate result of converting all discovered files.
/// Never fails — per-file errors are collected instead.
#[derive(Debug)]
pub struct ConversionSummary {
    pub results: Vec<ConversionResult>,
    pub errors: Vec<ErrorEntry>,
    pub warnings: Vec<WarningEntry>,
}

/// Convert all discovered Excel files.
///
/// Per-file errors are collected into `ConversionSummary::errors`
/// rather than aborting the entire batch.
pub fn convert_all(files: &[ExcelFile], cfg: &Config) -> ConversionSummary {
    let mut results = Vec::with_capacity(files.len());
    let mut errors = Vec::new();
    let warnings = Vec::new();

    for file in files {
        match convert_one(&file.path, &file.stem, cfg) {
            Ok(mut result) => {
                result.relative_path = file.relative_path.clone();
                info!(
                    stem = %file.stem,
                    configs = result.configs.len(),
                    rows = result.total_rows,
                    "Converted"
                );
                results.push(result);
            }
            Err(e) => {
                warn!(
                    path = %file.path.display(),
                    error = %e,
                    "Skipping file due to conversion error"
                );
                errors.push(ErrorEntry {
                    file: Some(file.relative_path.display().to_string()),
                    message: format!("{:#}", e),
                });
            }
        }
    }

    ConversionSummary {
        results,
        errors,
        warnings,
    }
}

/// Convert a single Excel file.
pub fn convert_one(path: &Path, stem: &str, cfg: &Config) -> Result<ConversionResult> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "xlsx" | "xls" | "xlsb" | "ods" => convert_xlsx(path, stem, cfg),
        _ => Err(EError::UnsupportedFormat {
            path: path.display().to_string(),
        }
        .into()),
    }
}

/// Convert an Excel workbook. Each sheet produces a `ConfigOutput`.
fn convert_xlsx(path: &Path, stem: &str, cfg: &Config) -> Result<ConversionResult> {
    let mut workbook = open_workbook_auto(path).map_err(|e| EError::ExcelParse {
        path: path.display().to_string(),
        source: e,
    })?;

    let sheet_names = workbook.sheet_names().to_vec();
    let mut configs = Vec::with_capacity(sheet_names.len());
    let mut total_rows = 0;

    for sheet_name in &sheet_names {
        debug!(sheet = %sheet_name, stem = %stem, "Reading sheet");

        let range = workbook
            .worksheet_range(sheet_name)
            .with_context(|| format!("Failed to read sheet: {}", sheet_name))?;

        match parse_sheet_with_meta(&range, sheet_name, cfg) {
            Ok(config) => {
                total_rows += config.valid_rows;
                configs.push(config);
            }
            Err(e) => {
                warn!(
                    sheet = %sheet_name,
                    error = %e,
                    "Skipping sheet due to parse error"
                );
                // Continue to next sheet — don't abort the whole file.
            }
        }
    }

    if configs.is_empty() {
        return Err(EError::Generic {
            message: format!("No valid sheets found in: {}", path.display()),
        }
        .into());
    }

    Ok(ConversionResult {
        relative_path: PathBuf::new(),
        stem: stem.to_string(),
        configs,
        total_rows,
    })
}

/// Parse a single sheet with the metadata convention:
///
///   Row 0: Col A = label (ignored), Col B = configName
///   Row 1: Col A = label (ignored), Col B = description
///   Row 2: Headers (field definitions)
///   Row 3: Field comments (skipped)
///   Row 4+: Data rows (Col A = id, must be unique unsigned integer)
///
/// Rows whose id starts with `//` are treated as comments and skipped.
fn parse_sheet_with_meta(
    range: &calamine::Range<Data>,
    sheet_name: &str,
    cfg: &Config,
) -> Result<ConfigOutput> {
    let rows: Vec<Vec<Data>> = range.rows().map(|r| r.to_vec()).collect();

    if rows.len() < 5 {
        return Err(EError::Generic {
            message: format!(
                "Sheet '{}' has fewer than 5 rows; expected metadata in rows 0-3 + at least 1 data row",
                sheet_name
            ),
        }
        .into());
    }

    // Row 0: extract config_name from column B (index 1).
    let raw_name = cell_to_string(rows[0].get(1).unwrap_or(&Data::Empty))
        .trim()
        .to_string();
    if raw_name.is_empty() {
        return Err(EError::Generic {
            message: format!(
                "Sheet '{}': configName is empty in Row 0, Col B",
                sheet_name
            ),
        }
        .into());
    }

    let config_name = validate_config_name(&raw_name).map_err(|e| EError::Generic {
        message: format!("Sheet '{}': {}", sheet_name, e),
    })?;

    // Row 1: extract description from column B (index 1).
    let description = cell_to_string(rows[1].get(1).unwrap_or(&Data::Empty))
        .trim()
        .to_string();

    // Row 2: headers (field definitions).
    let headers: Vec<String> = rows[2].iter().map(cell_to_string).collect();

    if headers.is_empty() {
        return Err(EError::Generic {
            message: format!("Sheet '{}': no headers found in Row 2", sheet_name),
        }
        .into());
    }

    if headers[0] != "id" {
        return Err(EError::Generic {
            message: format!(
                "Sheet '{}': first header must be 'id', got '{}'",
                sheet_name, headers[0]
            ),
        }
        .into());
    }

    // Row 3: field comments — skipped.

    // Rows 4+: data rows.
    let input_rows = rows.len().saturating_sub(4);
    let mut json_rows = Vec::new();
    let mut seen_ids = HashSet::new();
    let mut skipped_rows = 0usize;
    let mut failed_rows = 0usize;

    for (row_idx, row) in rows.iter().enumerate().skip(4) {
        // Check if the id is a comment (starts with "//").
        let id_cell = row.first().unwrap_or(&Data::Empty);
        let id_str = cell_to_string(id_cell);

        if id_str.starts_with("//") {
            skipped_rows += 1;
            debug!(sheet = %sheet_name, row = row_idx + 1, id = %id_str, "Skipping comment row");
            continue;
        }

        // Parse id as u64.
        let id = match parse_id(id_cell) {
            Some(v) => v,
            None => {
                failed_rows += 1;
                warn!(
                    sheet = %sheet_name,
                    row = row_idx + 1,
                    id = %id_str,
                    "Skipping row with invalid id"
                );
                continue;
            }
        };

        // Check for duplicate id.
        if !seen_ids.insert(id) {
            skipped_rows += 1;
            warn!(
                sheet = %sheet_name,
                row = row_idx + 1,
                id = %id,
                "Skipping duplicate id"
            );
            continue;
        }

        // Build row data excluding the id column (index 0).
        // The id will be re-inserted after mapping.
        let data_row: Vec<Data> = row.iter().skip(1).cloned().collect();
        let data_headers: Vec<String> = headers.iter().skip(1).cloned().collect();

        // Apply mapping on non-id columns.
        let mapped = mapping::apply_mapping(&data_headers, &[data_row], &cfg.mapping);

        // Insert the id field.
        let mut obj = if let Some(first) = mapped.into_iter().next() {
            match first {
                Value::Object(map) => map,
                _ => serde_json::Map::new(),
            }
        } else {
            serde_json::Map::new()
        };

        obj.insert("id".to_string(), Value::Number(id.into()));

        json_rows.push(Value::Object(obj));
    }

    let valid_rows = json_rows.len();

    Ok(ConfigOutput {
        config_name,
        description,
        input_rows,
        valid_rows,
        skipped_rows,
        failed_rows,
        rows: json_rows,
    })
}

/// Parse a cell value as a u64 id.
///
/// Returns `None` for empty, error, non-numeric, or negative values.
fn parse_id(cell: &Data) -> Option<u64> {
    match cell {
        Data::Int(i) => {
            if *i >= 0 {
                Some(*i as u64)
            } else {
                None
            }
        }
        Data::Float(f) => {
            if *f >= 0.0 && (*f - f.trunc()).abs() < f64::EPSILON && *f <= u64::MAX as f64 {
                Some(*f as u64)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Validate a config name.
///
/// Rules:
/// - Must start with a lowercase letter [a-z].
/// - Body may contain letters [a-zA-Z], digits, and `_`.
/// - Must end with a letter or digit [a-zA-Z0-9].
fn validate_config_name(raw: &str) -> Result<String, String> {
    let name = raw.to_string();

    if name.is_empty() {
        return Err("configName is empty".to_string());
    }

    let first = name.chars().next().unwrap();
    if !first.is_ascii_lowercase() {
        return Err(format!(
            "configName '{}' must start with a lowercase letter",
            name
        ));
    }

    let last = name.chars().last().unwrap();
    if !last.is_ascii_alphanumeric() {
        return Err(format!(
            "configName '{}' must end with a letter or digit",
            name
        ));
    }

    for (i, ch) in name.chars().enumerate() {
        if !ch.is_ascii_alphanumeric() && ch != '_' {
            return Err(format!(
                "configName '{}' contains invalid character '{}' at position {}",
                name, ch, i
            ));
        }
    }

    Ok(name)
}

/// Convert a calamine `Data` cell to its string representation.
fn cell_to_string(cell: &Data) -> String {
    match cell {
        Data::Empty => String::new(),
        Data::String(s) => s.clone(),
        Data::Float(f) => {
            if *f == f.trunc() && f.abs() < 1e15 {
                format!("{}", *f as i64)
            } else {
                format!("{}", f)
            }
        }
        Data::Int(i) => format!("{}", i),
        Data::Bool(b) => format!("{}", b),
        Data::DateTime(d) => d.to_string(),
        Data::DateTimeIso(d) | Data::DurationIso(d) => d.clone(),
        Data::Error(e) => {
            warn!(error = %e, "Cell contains an error value");
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MappingConfig;
    use tempfile::TempDir;

    /// convert_all on an empty file list produces an empty summary.
    #[test]
    fn test_convert_all_empty() {
        let tmp = TempDir::new().unwrap();
        let cfg = Config {
            input_dir: tmp.path().to_path_buf(),
            output_dir: tmp.path().join("output"),
            recursive: false,
            pretty: false,
            mapping: MappingConfig::default(),
        };
        let files: Vec<ExcelFile> = vec![];

        let summary = convert_all(&files, &cfg);
        assert!(summary.results.is_empty());
        assert!(summary.errors.is_empty());
        assert!(summary.warnings.is_empty());
    }

    /// convert_all collects errors for non-existent files without aborting.
    #[test]
    fn test_convert_all_collects_errors() {
        let tmp = TempDir::new().unwrap();
        let cfg = Config {
            input_dir: tmp.path().to_path_buf(),
            output_dir: tmp.path().join("output"),
            recursive: false,
            pretty: false,
            mapping: MappingConfig::default(),
        };

        let files = vec![ExcelFile {
            path: tmp.path().join("nonexistent.xlsx"),
            relative_path: PathBuf::from("nonexistent.xlsx"),
            stem: "nonexistent".to_string(),
            extension: "xlsx".to_string(),
        }];

        let summary = convert_all(&files, &cfg);
        assert!(summary.results.is_empty());
        assert_eq!(summary.errors.len(), 1);
        assert_eq!(summary.errors[0].file.as_deref(), Some("nonexistent.xlsx"));
    }

    /// convert_all propagates relative_path from ExcelFile to ConversionResult.
    #[test]
    fn test_convert_all_propagates_relative_path() {
        let tmp = TempDir::new().unwrap();
        let cfg = Config {
            input_dir: tmp.path().to_path_buf(),
            output_dir: tmp.path().join("output"),
            recursive: false,
            pretty: false,
            mapping: MappingConfig::default(),
        };

        let files = vec![ExcelFile {
            path: tmp.path().join("data.xlsx"),
            relative_path: PathBuf::from("sub/data.xlsx"),
            stem: "data".to_string(),
            extension: "xlsx".to_string(),
        }];

        let summary = convert_all(&files, &cfg);
        assert!(!summary.errors.is_empty());
        assert_eq!(summary.errors[0].file.as_deref(), Some("sub/data.xlsx"));
    }

    #[test]
    fn test_parse_id_int() {
        assert_eq!(parse_id(&Data::Int(42)), Some(42));
        assert_eq!(parse_id(&Data::Int(0)), Some(0));
        assert_eq!(parse_id(&Data::Int(-1)), None);
    }

    #[test]
    fn test_parse_id_float() {
        assert_eq!(parse_id(&Data::Float(42.0)), Some(42));
        assert_eq!(parse_id(&Data::Float(0.0)), Some(0));
        assert_eq!(parse_id(&Data::Float(-1.0)), None);
        assert_eq!(parse_id(&Data::Float(42.5)), None); // not an integer
    }

    #[test]
    fn test_parse_id_non_numeric() {
        assert_eq!(parse_id(&Data::String("hello".to_string())), None);
        assert_eq!(parse_id(&Data::Empty), None);
        assert_eq!(parse_id(&Data::Bool(true)), None);
    }

    #[test]
    fn test_validate_config_name_valid() {
        assert_eq!(validate_config_name("pet_types").unwrap(), "pet_types");
        assert_eq!(validate_config_name("food_types").unwrap(), "food_types");
        assert_eq!(validate_config_name("item2").unwrap(), "item2");
        assert_eq!(validate_config_name("a_b_c3").unwrap(), "a_b_c3");
    }

    #[test]
    fn test_validate_config_name_hyphen_rejected() {
        // Hyphens are no longer allowed.
        assert!(validate_config_name("pet-types").is_err());
        assert!(validate_config_name("a-b").is_err());
    }

    #[test]
    fn test_validate_config_name_uppercase_body_ok() {
        // Uppercase allowed in body, but must start with lowercase.
        assert_eq!(
            validate_config_name("pet_Types").unwrap(),
            "pet_Types"
        );
        assert_eq!(
            validate_config_name("myConfigName").unwrap(),
            "myConfigName"
        );
        // Must start with lowercase — these are rejected.
        assert!(validate_config_name("Pet_Types").is_err());
        assert!(validate_config_name("FOOD").is_err());
    }

    #[test]
    fn test_validate_config_name_invalid_start() {
        assert!(validate_config_name("123abc").is_err());
        assert!(validate_config_name("_test").is_err());
    }

    #[test]
    fn test_validate_config_name_invalid_end() {
        assert!(validate_config_name("test_").is_err());
    }

    #[test]
    fn test_validate_config_name_invalid_chars() {
        assert!(validate_config_name("hello world").is_err());
        assert!(validate_config_name("test.name").is_err());
        assert!(validate_config_name("test$name").is_err());
        assert!(validate_config_name("test-name").is_err());
    }
}
