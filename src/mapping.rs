//! Config-driven field mapping: column rename, exclusion, and nesting.

use crate::config::MappingConfig;
use calamine::Data;
use serde_json::{Map, Value};
use tracing::warn;

/// Apply config-driven mapping to transform raw Excel rows into JSON values.
///
/// Steps:
/// 1. Rename columns according to `column_map`.
/// 2. Exclude columns listed in `exclude_columns`.
/// 3. Nest values into nested JSON paths defined by `nested_paths`.
pub fn apply_mapping(
    headers: &[String],
    rows: &[Vec<Data>],
    mapping: &MappingConfig,
) -> Vec<Value> {
    // Build effective column name list (after rename but before exclusion).
    let effective_headers: Vec<Option<&String>> = headers
        .iter()
        .map(|h| {
            if mapping.exclude_columns.contains(h) {
                None
            } else {
                Some(h)
            }
        })
        .collect();

    // Map each row to a JSON object.
    rows.iter()
        .map(|row| row_to_json(&effective_headers, row, mapping))
        .collect()
}

/// Convert a single data row to a JSON value, applying column rename,
/// exclusion, and nested path logic.
fn row_to_json(headers: &[Option<&String>], row: &[Data], mapping: &MappingConfig) -> Value {
    let mut flat: Map<String, Value> = Map::new();

    for (i, header_opt) in headers.iter().enumerate() {
        let Some(header) = header_opt else {
            continue; // excluded column
        };

        let cell_value = row.get(i).map(cell_to_json).unwrap_or(Value::Null);

        // Apply column rename if configured.
        let output_key = mapping
            .column_map
            .get(*header)
            .cloned()
            .unwrap_or_else(|| header.to_string());

        flat.insert(output_key, cell_value);
    }

    // Apply nested path transformations.
    apply_nesting(flat, &mapping.nested_paths)
}

/// Convert a calamine cell value to a JSON value.
fn cell_to_json(cell: &Data) -> Value {
    match cell {
        Data::Empty => Value::Null,
        Data::String(s) => Value::String(s.clone()),
        Data::Float(f) => {
            // Represent integers without the ".0" suffix when possible.
            let value = serde_json::Number::from_f64(*f).unwrap_or_else(|| {
                // Fallback: write as string to avoid precision loss
                serde_json::Number::from_f64(0.0).unwrap()
            });
            Value::Number(value)
        }
        Data::Int(i) => Value::Number(serde_json::Number::from(*i)),
        Data::Bool(b) => Value::Bool(*b),
        Data::DateTime(d) => Value::String(d.to_string()),
        Data::DateTimeIso(d) | Data::DurationIso(d) => Value::String(d.clone()),
        Data::Error(e) => {
            warn!("Cell error: {:?}", e);
            Value::Null
        }
    }
}

/// Transform flat key-value pairs into nested JSON structures
/// based on dot-separated path mappings.
///
/// Example: `{ "address.city": "city" }` transforms
/// `{ "city": "Beijing" }` into `{ "address": { "city": "Beijing" } }`.
fn apply_nesting(
    mut flat: Map<String, Value>,
    nested_paths: &std::collections::HashMap<String, String>,
) -> Value {
    // Build a new map for nested values, removing the moved flat keys.
    let mut nested: Map<String, Value> = Map::new();

    for (source_col, target_path) in nested_paths {
        if let Some(value) = flat.remove(source_col) {
            set_nested_value(&mut nested, target_path, value);
        }
    }

    // Merge nested into flat.
    for (key, value) in nested {
        flat.insert(key, value);
    }

    Value::Object(flat)
}

/// Set a value at a dot-separated path within a nested JSON object,
/// creating intermediate objects as needed.
fn set_nested_value(root: &mut Map<String, Value>, path: &str, value: Value) {
    let segments: Vec<&str> = path.split('.').collect();
    let mut current = root;

    for (i, segment) in segments.iter().enumerate() {
        if i == segments.len() - 1 {
            // Last segment: set the value.
            current.insert(segment.to_string(), value.clone());
        } else {
            // Intermediate segment: traverse or create.
            current = current
                .entry(segment.to_string())
                .or_insert_with(|| Value::Object(Map::new()))
                .as_object_mut()
                .expect("Nested path conflict: intermediate key is not an object");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_cell_to_json_string() {
        let result = cell_to_json(&Data::String("hello".to_string()));
        assert_eq!(result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_cell_to_json_int() {
        let result = cell_to_json(&Data::Int(42));
        assert_eq!(result, Value::Number(serde_json::Number::from(42)));
    }

    #[test]
    fn test_cell_to_json_empty() {
        let result = cell_to_json(&Data::Empty);
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_column_exclusion() {
        let headers = vec!["Name".to_string(), "Internal".to_string()];
        let rows = vec![vec![Data::String("Alice".to_string()), Data::Int(99)]];
        let mapping = MappingConfig {
            column_map: HashMap::new(),
            exclude_columns: vec!["Internal".to_string()],
            nested_paths: HashMap::new(),
        };
        let result = apply_mapping(&headers, &rows, &mapping);
        assert_eq!(result.len(), 1);
        let obj = result[0].as_object().unwrap();
        assert!(obj.contains_key("Name"));
        assert!(!obj.contains_key("Internal"));
    }

    #[test]
    fn test_column_rename() {
        let headers = vec!["姓名".to_string()];
        let rows = vec![vec![Data::String("张三".to_string())]];
        let mut column_map = HashMap::new();
        column_map.insert("姓名".to_string(), "name".to_string());
        let mapping = MappingConfig {
            column_map,
            exclude_columns: vec![],
            nested_paths: HashMap::new(),
        };
        let result = apply_mapping(&headers, &rows, &mapping);
        let obj = result[0].as_object().unwrap();
        assert_eq!(obj["name"], Value::String("张三".to_string()));
    }
}
