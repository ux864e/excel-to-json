//! Integration tests for the excel-to-json CLI binary.
//!
//! Uses `assert_cmd` to run the compiled binary as a subprocess
//! and `predicates` for composable output assertions.

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper: get the path to the test fixtures directory.
#[allow(dead_code)]
fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// Helper: create a temporary output directory.
fn temp_output_dir() -> (TempDir, PathBuf) {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let path = dir.path().to_path_buf();
    (dir, path)
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
        .failure();
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
        .success();

    // No Excel files found → no JSON output.
    let json_files: Vec<_> = std::fs::read_dir(&output)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "json")
                .unwrap_or(false)
        })
        .collect();
    assert!(json_files.is_empty());
}
