# Rust Coding Standards — Project-Specific Extra Rules

## Excel & JSON Rules
1. **No hardcoded column names**: All Excel column name references must come from `MappingConfig` (via `excel-to-json.toml` or defaults). Never hardcode column name strings in conversion logic.
2. **Mapping logic in mapping.rs**: All field transformation logic (rename, exclude, nest) lives in `mapping.rs`. Converter only orchestrates, never transforms.

## IPC Message Format
3. **Stdout IPC format**: All stdout output is line-delimited JSON with `{"type":"...", ...}` schema. Each line is one complete JSON object.
4. **Stderr for diagnostics**: Use `tracing` macros (`info!`, `warn!`, `error!`) which output to stderr. Never write non-IPC content to stdout.

## Config File Rules
5. **TOML format**: Config file is TOML (`excel-to-json.toml`). All mapping configuration uses this format.
6. **CLI overrides config**: When a CLI flag is explicitly provided, it takes precedence over the config file value.
7. **Config file optional**: The tool must work without a config file (all CLI defaults + identity mapping).

## Dependency Rules
8. **Minimize dependencies**: Only add a dependency when the alternative is writing non-trivial code. Prefer std over external crates.
9. **No async unless necessary**: This is a synchronous CLI tool. Do not add `tokio` or other async runtimes unless a concrete use case exists (e.g., IPC server).

## File Organization
10. **One module per file**: No mega-files. If a module exceeds ~300 lines, consider splitting.
11. **Tests inline**: Unit tests go in the same file as the code they test, inside `#[cfg(test)] mod tests`.
