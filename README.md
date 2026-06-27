# excel-to-json

A CLI tool to convert Excel files (.xlsx, .xls, .xlsb, .ods, .csv) to JSON with config-driven field mapping.

## Features

- **Multi-format support**: Reads .xlsx, .xls, .xlsb, .ods, and .csv files via [calamine](https://github.com/tafia/calamine)
- **Directory traversal**: Recursively scan directories for Excel files
- **Config-driven mapping**: Define column renames, exclusions, and nested JSON paths in TOML
- **Multi-sheet handling**: Each sheet becomes a named key in the output JSON
- **IPC via stdout**: Line-delimited JSON messages for progress/result notification
- **CLI overrides**: All config file settings can be overridden by command-line flags

## Installation

```bash
cargo install excel-to-json
```

## Quick Start

```bash
# Convert all Excel files in current directory
excel-to-json

# Specify input and output directories
excel-to-json --input ./data --output ./json

# Use a config file for field mapping
excel-to-json --config ./mapping.toml
```

## Configuration

Create an `excel-to-json.toml` file for field mapping:

```toml
# Optional overrides (CLI args take precedence)
# input_dir = "./data"
# output_dir = "./output"
# recursive = true
# pretty = true

[mapping]
# Rename columns: "Excel header" → "JSON key"
[mapping.column_map]
"姓名" = "name"
"年龄" = "age"
"城市" = "city"

# Exclude these columns from output
exclude_columns = ["备注", "内部编号"]

# Nest values into JSON paths
[mapping.nested_paths]
"省份" = "address.province"
"城市" = "address.city"
```

## IPC Output Format

The tool emits line-delimited JSON on stdout during conversion:

```json
{"type":"progress","file":"/data/sales.xlsx","status":"converting"}
{"type":"done","file":"/data/sales.xlsx","rows":150}
{"type":"error","file":"/data/broken.xlsx","message":"Failed to parse"}
```

## Development

### Prerequisites
- Rust 1.85+ (edition 2024)

### Build
```bash
cargo build
cargo build --release
```

### Test
```bash
cargo test
cargo test -- --nocapture  # show stdout
```

### Code Quality
```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

## License

MIT
