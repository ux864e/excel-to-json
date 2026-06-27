//! Core conversion logic: Excel → JSON via calamine.

use crate::config::Config;
use crate::error::EError;
use crate::mapping;
use crate::output::IpcMessage;
use crate::walker::ExcelFile;
use anyhow::{Context, Result};
use calamine::{Data, Reader, open_workbook_auto};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use tracing::{debug, info};

/// Result of converting a single Excel file.
#[derive(Debug)]
pub struct ConversionResult {
    /// Source file stem (for output naming).
    pub stem: String,
    /// Converted JSON data: sheet name → array of row objects.
    pub sheets: HashMap<String, Vec<Value>>,
    /// Total row count across all sheets.
    pub total_rows: usize,
}

/// Convert all discovered Excel files.
pub fn convert_all(files: &[ExcelFile], cfg: &Config) -> Result<Vec<ConversionResult>> {
    let mut results = Vec::with_capacity(files.len());

    for file in files {
        IpcMessage::progress(&file.path, "converting");
        let result = convert_one(&file.path, &file.stem, cfg)
            .with_context(|| format!("Failed to convert: {}", file.path.display()))?;

        info!(
            stem = %file.stem,
            sheets = result.sheets.len(),
            rows = result.total_rows,
            "Converted"
        );
        IpcMessage::done(&file.path, result.total_rows);
        results.push(result);
    }

    Ok(results)
}

/// Convert a single Excel (or CSV) file.
pub fn convert_one(path: &Path, stem: &str, cfg: &Config) -> Result<ConversionResult> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "csv" => convert_csv(path, stem, cfg),
        "xlsx" | "xls" | "xlsb" | "ods" => convert_xlsx(path, stem, cfg),
        _ => Err(EError::UnsupportedFormat {
            path: path.display().to_string(),
        }
        .into()),
    }
}

/// Convert an Excel workbook using calamine.
fn convert_xlsx(path: &Path, stem: &str, cfg: &Config) -> Result<ConversionResult> {
    let mut workbook = open_workbook_auto(path).map_err(|e| EError::ExcelParse {
        path: path.display().to_string(),
        source: e,
    })?;

    let sheet_names = workbook.sheet_names().to_vec();
    let mut sheets = HashMap::new();
    let mut total_rows = 0;

    for sheet_name in &sheet_names {
        debug!(sheet = %sheet_name, stem = %stem, "Reading sheet");

        if let Ok(range) = workbook.worksheet_range(sheet_name) {
            let (headers, data_rows) = extract_headers_and_rows(&range)?;
            let sheet_data = mapping::apply_mapping(&headers, &data_rows, &cfg.mapping);
            let row_count = sheet_data.len();
            sheets.insert(sheet_name.clone(), sheet_data);
            total_rows += row_count;
        }
    }

    Ok(ConversionResult {
        stem: stem.to_string(),
        sheets,
        total_rows,
    })
}

/// Convert a CSV file using calamine.
fn convert_csv(path: &Path, stem: &str, cfg: &Config) -> Result<ConversionResult> {
    let mut workbook = open_workbook_auto(path).map_err(|e| EError::ExcelParse {
        path: path.display().to_string(),
        source: e,
    })?;

    let sheet_names = workbook.sheet_names().to_vec();
    let mut sheets = HashMap::new();
    let mut total_rows = 0;

    // CSV files have a single default sheet.
    let sheet_name = sheet_names.first().map(|s| s.as_str()).unwrap_or("Sheet1");

    if let Ok(range) = workbook.worksheet_range(sheet_name) {
        let (headers, data_rows) = extract_headers_and_rows(&range)?;
        let sheet_data = mapping::apply_mapping(&headers, &data_rows, &cfg.mapping);
        total_rows = sheet_data.len();
        sheets.insert(sheet_name.to_string(), sheet_data);
    }

    Ok(ConversionResult {
        stem: stem.to_string(),
        sheets,
        total_rows,
    })
}

/// Extract the header row and data rows from a calamine Range.
/// The first row is treated as headers.
fn extract_headers_and_rows(
    range: &calamine::Range<Data>,
) -> Result<(Vec<String>, Vec<Vec<Data>>)> {
    let mut rows_iter = range.rows();
    let header_row = rows_iter.next().ok_or_else(|| EError::Generic {
        message: "Empty sheet: no header row found".to_string(),
    })?;

    let headers: Vec<String> = header_row.iter().map(|cell| cell.to_string()).collect();

    let data_rows: Vec<Vec<Data>> = rows_iter.map(|row| row.to_vec()).collect();

    Ok((headers, data_rows))
}
