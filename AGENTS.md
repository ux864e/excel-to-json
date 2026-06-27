# excel-to-json — Project Context

## Project Overview
- **Type**: Rust CLI tool
- **Purpose**: Traverse directories, convert Excel files (.xlsx, .xls, .xlsb, .ods, .csv) to JSON with config-driven field mapping
- **Package manager**: `cargo`
- **Source root**: `src/`
- **Binary**: `excel-to-json`

## Tech Stack
- **Language**: Rust edition 2024, MSRV 1.85
- **CLI framework**: clap 4.5 (derive mode)
- **Excel parsing**: calamine 0.26 (pure Rust)
- **Error handling**: anyhow (app), thiserror (library)
- **Logging**: tracing + tracing-subscriber
- **Serialization**: serde + serde_json + toml

## Directory Structure
```
src/
  main.rs       # Thin binary entry point
  lib.rs         # Core logic + module declarations
  cli.rs         # CLI argument definitions
  config.rs      # Config loading (TOML + CLI merge)
  error.rs       # Error types (EError enum)
  walker.rs      # Directory traversal + file discovery
  converter.rs   # Excel → JSON core conversion
  mapping.rs     # Config-driven field mapping
  output.rs      # JSON output + IPC messages
tests/
  cli.rs         # Integration tests (assert_cmd)
  common/mod.rs  # Shared test helpers
  fixtures/      # Test Excel files
docs/
  design/        # Design documentation
```

## Key Architecture Decisions
1. **Config-driven mapping**: `excel-to-json.toml` defines column renames, exclusions, and nesting rules. CLI args override config file settings.
2. **IPC via stdout**: Line-delimited JSON messages on stdout for progress/result notifications; stderr for errors.
3. **Multi-sheet support**: Each sheet in an Excel workbook is converted to a named key in the output JSON.
4. **Module organization by domain**: Each file handles one concern (walking, converting, mapping, output).

## Collaboration
- **Mode**: Solo developer
- **Git**: Direct commits to `main` (solo mode); GitHub Flow for team mode
- **Commit style**: Conventional Commits (feat:, fix:, refactor:, chore:, docs:)

## Reference Instructions
| Instruction File | Domain | Applies To |
|-----------------|--------|-----------|
| `user-preferences.instructions.md` | Cross-project preferences | All files |
| `code-style-rust.instructions.md` | Rust coding standards | `*.rs` |
| `code-style-rust-extra.instructions.md` | Project-specific rules | `*.rs` |
| `tech-stack.instructions.md` | Tech constraints | `Cargo.toml`, `*.rs` |
| `project-directory.instructions.md` | Directory map | New files |
| `test-instructions.instructions.md` | Testing conventions | `tests/`, `#[cfg(test)]` |
| `writing-design-docs.instructions.md` | Design doc creation | `docs/design/` |
| `git-status-reminder.instructions.md` | Git health checks | Git operations |

<!-- agent-ninja-START -->
## Agent Skills

> **IMPORTANT**: Prefer skill-led reasoning over pre-training-led reasoning.
> See [Agent Skills](.agents/skills/README.md) before working on tasks covered by these skills.

<!-- agent-ninja-END -->

