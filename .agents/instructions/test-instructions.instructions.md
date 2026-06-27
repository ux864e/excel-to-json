# Test Instructions

## Test Framework
- **Rust built-in test framework**: `#[test]` attribute, `cargo test`.
- **Integration testing**: `assert_cmd` (run binary as subprocess) + `predicates` (output assertions).
- **Assertions**: Standard `assert!` / `assert_eq!` macros; `pretty_assertions` for colorized diffs.

## Test Organization
### Unit Tests
- Location: Inline in source files, inside `#[cfg(test)] mod tests { ... }` blocks.
- Access: `use super::*;` to access parent module's items.
- Naming: `test_<function_or_scenario>`.

### Integration Tests
- Location: `tests/cli.rs` (main test file). Each `.rs` file in `tests/` is compiled as a separate crate.
- Access: `use excel_to_json::<module>;` for library API; or `Command::cargo_bin("excel-to-json")` for binary testing.
- Fixtures: `tests/fixtures/` for test Excel and CSV files.

## Test Data
- **Temporary directories**: Use `tempfile::TempDir` for test output directories. Never hardcode paths.
- **Test fixtures**: Small Excel/CSV files in `tests/fixtures/`. Document what each fixture represents.
- **Mocking**: Not needed for this project. Tests use real file I/O with temp directories.

## What to Test
| Module | Unit Tests | Integration Tests |
|--------|-----------|-------------------|
| `mapping.rs` | Column rename, exclusion, nesting, cell-to-JSON conversion | — |
| `output.rs` | IPC message serialization format | — |
| `walker.rs` | File extension filtering | — |
| `converter.rs` | — | Real Excel file conversion |
| `config.rs` | TOML parsing, CLI override logic | Config file + CLI arg merge |
| `cli.rs` | — | --help, --version, error cases |

## Running Tests
```bash
cargo test                          # All tests
cargo test --lib                    # Unit tests only
cargo test --test cli               # Integration tests only
cargo test test_column_rename       # Single test by name
cargo test -- --nocapture           # Show stdout during tests
```

## CI Test Gate
```bash
cargo test --all-targets
```
All tests must pass in CI. No flaky tests allowed.
