# Project Directory Map

## Source Code (`src/`)
| File | Purpose | Key Exports |
|------|---------|------------|
| `main.rs` | Binary entry point. Thin wrapper calling `run()`. | `main()` |
| `lib.rs` | Library root. Module declarations + `run()`. | `run()` |
| `cli.rs` | CLI argument definitions (clap derive). | `Args` |
| `config.rs` | Config loading. Merge TOML file + CLI overrides. | `Config`, `ConfigFile`, `MappingConfig`, `load()` |
| `error.rs` | Custom error enum (thiserror). | `EError` |
| `walker.rs` | Directory traversal, file discovery. | `collect_files()`, `ExcelFile` |
| `converter.rs` | Core Excel→JSON conversion. | `convert_all()`, `ConversionResult` |
| `mapping.rs` | Field mapping: rename, exclude, nest. | `apply_mapping()` |
| `output.rs` | JSON file output + stdout IPC messages. | `emit_results()`, `IpcMessage` |

## Tests (`tests/`)
| Path | Purpose |
|------|---------|
| `cli.rs` | Integration tests (assert_cmd). Tests the compiled binary. |
| `common/mod.rs` | Shared test helpers. Not a test target itself. |
| `fixtures/` | Test data files (Excel, CSV) for integration tests. |

## Configuration (root)
| File | Purpose |
|------|---------|
| `Cargo.toml` | Package manifest, dependencies, profiles. |
| `Cargo.lock` | Locked dependency versions (committed for binary). |
| `.rustfmt.toml` | Rust formatting configuration. |
| `clippy.toml` | Clippy lint thresholds. |
| `.gitignore` | Git ignore rules. |
| `.gitattributes` | Git line-ending normalization. |

## Agent Configuration
| Path | Purpose |
|------|---------|
| `CLAUDE.md` | Agent entry point. Directs to instruction files. |
| `AGENTS.md` | Project context and reference table. |
| `.agents/instructions/` | Detailed instruction files by domain. |
| `.claude/settings.json` | Agent permissions configuration. |
| `.claude/settings.local.json` | Local overrides (gitignored). |
| `.claude/memory/` | Persistent knowledge store. |

## Documentation (`docs/`)
| Path | Purpose |
|------|---------|
| `docs/design/` | Design documentation — internal (Chinese, YAML frontmatter). |
| `docs/release/` | External documentation — API docs, user guides, public references. |

## Dev Records
| File | Purpose |
|------|---------|
| `docs/TASKS.md` | To-do list and work in progress. |
| `docs/DEV_NOTES.md` | Development decisions, issues, observations. |
| `CHANGELOG.md` | Changelog index table. |
| `changelog/` | Daily change records (`CHANGELOG_yyyymmdd.md`). |
