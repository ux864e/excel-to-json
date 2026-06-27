# DEV NOTES

## 2026-06-27 — Project Initialization
- Initialized Rust CLI project with cargo.
- Set up agent configuration (CLAUDE.md, AGENTS.md, .agents/instructions/).
- Configured rustfmt, clippy, CI pipeline.
- Established design document conventions (cuddle three-level structure, YAML frontmatter).
- Key decisions: calamine for Excel parsing, clap for CLI, anyhow+thiserror for errors, stdout NDJSON for IPC.
