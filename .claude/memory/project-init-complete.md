---
name: project-init-complete
description: Record of initial project setup decisions and configuration
metadata:
  type: project
---

# Project Initialization Complete

excel-to-json was initialized on 2026-06-27.

**Why:** First Rust practice project. Personal CLI tool for Excel→JSON conversion with config-driven field mapping.

**Key decisions:**
- Language: Rust edition 2024, single crate
- Excel parsing: calamine 0.26
- CLI framework: clap 4.5 (derive)
- Error handling: anyhow (app) + thiserror (lib)
- IPC: stdout line-delimited JSON, stderr for diagnostics
- Config: TOML file + CLI args override
- Code style: Rust native + E-prefix for enums, UPPER_SNAKE_CASE members
- Design docs: Chinese, YAML frontmatter, simplified cuddle three-level structure
- Git: solo main direct push, conventional commits

**How to apply:** See `.agents/instructions/` for all coding standards and conventions. Design docs in `docs/design/`.
