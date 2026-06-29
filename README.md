# excel-to-json (config-importer)

A CLI tool to convert Excel files (.xlsx, .xls, .xlsb, .ods) to JSON config files. Each sheet (tab) produces one config file, driven by a metadata-row convention. Designed to be called via stdin pipe from a parent process.

## Features

- **Multi-format support**: Reads .xlsx, .xls, .xlsb, and .ods files via [calamine](https://github.com/tafia/calamine)
- **Sheet → Config**: Each Excel tab becomes an independent config file
- **Metadata-driven**: configName, description, and field definitions extracted from sheet rows 0-3
- **`id` column enforcement**: Column A is always `id` — unique unsigned integer, `//` prefix for comments
- **Row statistics**: inputRows, validRows, skippedRows, failedRows per config
- **camelCase → kebab-case filenames**: Output filenames are automatically kebab-cased (e.g. configName `myConfigName` → file `my-config-name.json`)
- **Config-driven mapping**: Define column renames, exclusions, and nested JSON paths in TOML
- **Stdin integration**: Read `outputDir` from piped JSON; write directly to that directory
- **Single-line JSON summary**: One status line on stdout for programmatic consumption
- **Per-file error resilience**: Errors on individual files/sheets are collected without aborting the batch

## Excel Tab Convention

Each sheet must follow this structure:

| Row | Col A | Col B | Purpose |
|-----|-------|-------|---------|
| 0 | label (ignored) | **configName** | Config identifier (validated, forced lowercase) |
| 1 | label (ignored) | **description** | Human-readable description |
| 2 | `id` | field names... | Field definitions (headers) |
| 3 | comments | comments | Field comments (skipped) |
| 4+ | id value | data... | Data rows |

### configName Rules

- Must start with `[a-z]` (lowercase)
- Body: `[a-zA-Z0-9_]`
- Must end with `[a-zA-Z0-9]`

### `id` Column Rules

- Column A (first column) is always `id`
- Must be a unique unsigned integer
- Rows whose `id` starts with `//` are comment rows — skipped
- Duplicate `id` values — skipped with warning
- Invalid/unparseable `id` — counted as `failedRows`

## Stdin Input

When called from a parent process, pipe a JSON object to stdin:

```json
{"outputDir": "/absolute/path/to/output"}
```

The tool writes output files to `<outputDir>/<configName>.json`. If configName contains uppercase letters (camelCase), the filename is automatically converted to kebab-case (e.g. `myConfigName` → `my-config-name.json`).

Without stdin, the CLI `--output` flag is used (legacy mode for manual testing).

## Quick Start

```bash
# Pipe config via stdin
echo '{"outputDir":"/tmp/out"}' | excel-to-json --input ./data

# Without stdin (uses --output)
excel-to-json --input ./data --output ./json
```

## Configuration

Create an `excel-to-json.toml` file for field mapping:

```toml
[mapping]
# Rename columns: "Excel header" → "JSON key"
[mapping.column_map]
"姓名" = "name"
"年龄" = "age"

# Exclude these columns from output
exclude_columns = ["备注"]

# Nest values into JSON paths
[mapping.nested_paths]
"城市" = "address.city"
```

Note: The `id` column is always included regardless of `exclude_columns`.

## Output Format

### Stdout: Single-line JSON Summary

```json
{
  "status": "success",
  "files": [
    {
      "configName": "pet-types",
      "path": "pet-types.json",
      "description": "Available pet types",
      "validRows": 4,
      "inputRows": 6,
      "skippedRows": 2,
      "failedRows": 0
    }
  ],
  "errors": [
    {"file": "bad.xlsx", "message": "No valid sheets found"}
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `status` | string | `"success"` (≥1 config or empty dir) or `"error"` (all failed) |
| `files[].configName` | string | Config name from sheet Row 0, Col B (validated + lowercased) |
| `files[].path` | string | Output filename (`<configName>.json`) |
| `files[].description` | string | From sheet Row 1, Col B |
| `files[].validRows` | number | Rows successfully included in output |
| `files[].inputRows` | number | Total data rows in the sheet |
| `files[].skippedRows` | number | Comment rows + duplicate ids |
| `files[].failedRows` | number | Rows with invalid id |
| `errors[]` | array | Per-file/sheet errors (omitted when empty) |
| `warnings[]` | array | Non-fatal warnings (omitted when empty) |

### Disk: JSON Config Files

Output at `<outputDir>/<configName>.json` (camelCase configNames are kebab-cased in the filename):

```json
{
  "configName": "pet-types",
  "description": "Available pet types",
  "items": [
    {"id": 1, "name": "Fido", "type": "dog"},
    {"id": 2, "name": "Whiskers", "type": "cat"}
  ]
}
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | At least one config produced, or nothing to do (empty directory) |
| 1 | All configs failed, or a global error occurred |

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

### Deploy
```bash
# Default target: ../cuddle-app-backend/tools
./scripts/deploy.sh

# Custom target
TARGET_DIR=/opt/myapp/tools ./scripts/deploy.sh
```

## License

MIT
