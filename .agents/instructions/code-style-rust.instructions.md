# Rust Coding Standards (Baseline)

## Formatting
- Use `rustfmt` with project `.rustfmt.toml` (edition 2024, max_width 100, Unix newlines).
- Run `cargo fmt --all` before committing. CI enforces `cargo fmt --all -- --check`.
- Imports ordered by convention: std → external → crate.

## Naming Conventions
- **Modules/files**: `snake_case` (e.g., `converter.rs`, `cli.rs`).
- **Types (structs, traits, type aliases)**: `CamelCase` (e.g., `ConversionResult`, `MappingConfig`).
- **Functions/variables**: `snake_case` (e.g., `convert_all`, `input_dir`).
- **Constants/statics**: `UPPER_SNAKE_CASE` (e.g., `SUPPORTED_EXTENSIONS`).
- **Enums**: `E` prefix + `CamelCase` (e.g., `EError`), members `UPPER_SNAKE_CASE`.
  - Rationale: E-prefix convention carried over from TypeScript projects for visual distinction.
- **Traits**: No prefix (Rust native convention). Use `CamelCase` verb/adjective names.

## Error Handling
- **Application code**: Use `anyhow::Result<T>` and `?` operator.
- **Library code**: Define custom error enums with `thiserror::Error` derive.
- **Add context**: `.with_context(|| format!("..."))` at error boundaries.
- **No unwrap/expect in production**: Use `?` or proper error handling. `expect()` only for truly infallible cases with clear justification.

## Code Organization
- One module per domain concern (not per technical layer).
- `lib.rs` declares all modules; `main.rs` is a thin wrapper.
- Public API: mark functions as `pub` only when needed by other modules or integration tests.
- Inline unit tests in `#[cfg(test)] mod tests` blocks at the bottom of each source file.

## Documentation
- Code comments in English.
- `//!` module-level doc comments describe the module's purpose.
- `///` item-level doc comments for all public functions and types.
- Document "why", not "what" (the code already shows "what").

## Quality Gates
```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```
