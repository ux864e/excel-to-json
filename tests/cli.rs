//! Integration tests for the excel-to-json CLI binary.
//!
//! Uses `assert_cmd` to run the compiled binary as a subprocess
//! and `predicates` for composable output assertions.

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Helper: get the path to the test fixtures directory.
fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// Helper: create a temporary output directory.
fn temp_output_dir() -> (TempDir, PathBuf) {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let path = dir.path().to_path_buf();
    (dir, path)
}

/// Helper: copy the sample .xlsx fixture into the given directory with the
/// specified filename.
fn copy_fixture(dir: &Path, name: &str) {
    let src = fixtures_dir().join("sample.xlsx");
    let dst = dir.join(name);
    std::fs::copy(&src, &dst).expect("Failed to copy fixture file");
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("excel-to-json").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("--input"))
        .stdout(predicate::str::contains("--output"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("excel-to-json").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("excel-to-json"));
}

#[test]
fn test_missing_input_dir() {
    let (_dir, output) = temp_output_dir();

    let mut cmd = Command::cargo_bin("excel-to-json").unwrap();
    cmd.arg("--input")
        .arg("/nonexistent/path/xyz")
        .arg("--output")
        .arg(output.to_str().unwrap())
        .assert()
        .failure()
        .stdout(predicate::str::contains(r#""status":"error""#));
}

#[test]
fn test_empty_input_dir() {
    let input = TempDir::new().expect("Failed to create temp dir");
    let (_output_dir, output) = temp_output_dir();

    let mut cmd = Command::cargo_bin("excel-to-json").unwrap();
    cmd.arg("--input")
        .arg(input.path().to_str().unwrap())
        .arg("--output")
        .arg(output.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""status":"success""#))
        .stdout(predicate::str::contains(r#""files":[]"#));
}

#[test]
fn test_single_xlsx_conversion() {
    let input = TempDir::new().expect("Failed to create temp dir");
    let (_output_dir, output) = temp_output_dir();

    copy_fixture(input.path(), "data.xlsx");

    let mut cmd = Command::cargo_bin("excel-to-json").unwrap();
    cmd.arg("--input")
        .arg(input.path().to_str().unwrap())
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--recursive=false")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""status":"success""#))
        .stdout(predicate::str::contains(r#""configName":"sample_config""#))
        .stdout(predicate::str::contains(r#""path":"sample_config.json""#))
        .stdout(predicate::str::contains(
            r#""description":"Sample config for testing""#,
        ))
        .stdout(predicate::str::contains(r#""validRows":1"#))
        .stdout(predicate::str::contains(r#""inputRows":1"#));

    // Output file at configured path.
    let output_file = output.join("sample_config.json");
    assert!(output_file.exists());

    // Verify JSON content is wrapped under config name.
    let content = std::fs::read_to_string(&output_file).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(parsed["configName"], "sample_config");
    assert_eq!(parsed["description"], "Sample config for testing");
    let rows = &parsed["items"];
    assert_eq!(rows[0]["id"], 1);
    assert_eq!(rows[0]["Name"], "Alice");
    assert_eq!(rows[0]["Age"], "30");
    assert_eq!(rows[0]["City"], "NYC");
}

#[test]
fn test_multi_sheet_configs() {
    let input = TempDir::new().expect("Failed to create temp dir");
    let (_output_dir, output) = temp_output_dir();

    // Copy the multi-sheet fixture.
    let src = fixtures_dir().join("multi-sheet.xlsx");
    let dst = input.path().join("data.xlsx");
    std::fs::copy(&src, &dst).expect("Failed to copy multi-sheet fixture");

    let mut cmd = Command::cargo_bin("excel-to-json").unwrap();
    cmd.arg("--input")
        .arg(input.path().to_str().unwrap())
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--recursive=false")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""status":"success""#))
        .stdout(predicate::str::contains(r#""configName":"pet_types""#))
        .stdout(predicate::str::contains(r#""configName":"food_types""#))
        .stdout(predicate::str::contains(
            r#""description":"Available pet types""#,
        ))
        .stdout(predicate::str::contains(
            r#""description":"Available food types""#,
        ));

    // Both config files should exist.
    let base = output;
    assert!(base.join("pet_types.json").exists());
    assert!(base.join("food_types.json").exists());

    // Verify pet_types content: comment row (//comment) and duplicate id (2) are skipped.
    let pet_content = std::fs::read_to_string(base.join("pet_types.json")).unwrap();
    let pet: serde_json::Value = serde_json::from_str(&pet_content).unwrap();
    assert_eq!(pet["configName"], "pet_types");
    assert_eq!(pet["description"], "Available pet types");
    let pet_rows = pet["items"].as_array().unwrap();
    // 6 data rows - 1 comment - 1 duplicate = 4 effective rows
    assert_eq!(pet_rows.len(), 4);
    let ids: Vec<i64> = pet_rows.iter().map(|r| r["id"].as_i64().unwrap()).collect();
    assert_eq!(ids, vec![1, 2, 3, 4]);
}

#[test]
fn test_comment_rows_skipped() {
    let input = TempDir::new().expect("Failed to create temp dir");
    let (_output_dir, output) = temp_output_dir();

    // Use multi-sheet fixture which has a //comment row in the Pets sheet.
    let src = fixtures_dir().join("multi-sheet.xlsx");
    let dst = input.path().join("data.xlsx");
    std::fs::copy(&src, &dst).unwrap();

    let mut cmd = Command::cargo_bin("excel-to-json").unwrap();
    cmd.arg("--input")
        .arg(input.path().to_str().unwrap())
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--recursive=false")
        .assert()
        .success();

    // Verify the comment row (id=//comment) is NOT in the output.
    let pet_file = output.join("pet_types.json");
    let content = std::fs::read_to_string(&pet_file).unwrap();
    // The string "skip me" was in the comment row and should not appear.
    assert!(!content.contains("skip me"));
}

#[test]
fn test_duplicate_id_skipped() {
    let input = TempDir::new().expect("Failed to create temp dir");
    let (_output_dir, output) = temp_output_dir();

    let src = fixtures_dir().join("multi-sheet.xlsx");
    let dst = input.path().join("data.xlsx");
    std::fs::copy(&src, &dst).unwrap();

    let mut cmd = Command::cargo_bin("excel-to-json").unwrap();
    cmd.arg("--input")
        .arg(input.path().to_str().unwrap())
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--recursive=false")
        .assert()
        .success();

    // Duplicate id=2 row has name="Duplicate" — should not appear.
    let pet_file = output.join("pet_types.json");
    let content = std::fs::read_to_string(&pet_file).unwrap();
    assert!(!content.contains("Duplicate"));
}

#[test]
fn test_mixed_success_failure() {
    let input = TempDir::new().expect("Failed to create temp dir");
    let (_output_dir, output) = temp_output_dir();

    // One valid new-format .xlsx file.
    copy_fixture(input.path(), "good.xlsx");

    // One empty .xlsx file — calamine can't parse it.
    let bad_path = input.path().join("bad.xlsx");
    std::fs::File::create(&bad_path).unwrap();

    let mut cmd = Command::cargo_bin("excel-to-json").unwrap();
    cmd.arg("--input")
        .arg(input.path().to_str().unwrap())
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--recursive=false")
        .assert()
        .success() // At least one file succeeded → success
        .stdout(predicate::str::contains(r#""status":"success""#))
        .stdout(predicate::str::contains(r#""configName":"sample_config""#))
        .stdout(predicate::str::contains("bad.xlsx"));
}

#[test]
fn test_all_files_fail() {
    let input = TempDir::new().expect("Failed to create temp dir");
    let (_output_dir, output) = temp_output_dir();

    // Create only an empty .xlsx file — calamine will fail to parse it.
    let bad_path = input.path().join("broken.xlsx");
    std::fs::File::create(&bad_path).unwrap();

    let mut cmd = Command::cargo_bin("excel-to-json").unwrap();
    cmd.arg("--input")
        .arg(input.path().to_str().unwrap())
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--recursive=false")
        .assert()
        .failure()
        .stdout(predicate::str::contains(r#""status":"error""#))
        .stdout(predicate::str::contains("broken.xlsx"));
}
