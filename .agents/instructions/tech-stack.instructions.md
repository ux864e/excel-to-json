# Technical Stack

## Language & Toolchain
- **Rust**: edition 2024, MSRV 1.85
- **Package manager**: Cargo (comes with Rust)
- **Toolchain**: stable channel

## Runtime Dependencies
| Crate | Version | Purpose |
|-------|---------|---------|
| `clap` | 4.5 | CLI argument parsing (derive mode) |
| `anyhow` | 1.0 | Application-level error handling |
| `thiserror` | 2.0 | Library error type derivation |
| `serde` | 1.0 | Serialization framework |
| `serde_json` | 1.0 | JSON serialization/deserialization |
| `toml` | 0.8 | TOML config file parsing |
| `calamine` | 0.26 | Excel/ODS file reading |
| `tracing` | 0.1 | Structured logging |
| `tracing-subscriber` | 0.3 | Log output formatting + env-filter |
| `directories` | 6 | XDG config directory resolution |

## Dev Dependencies
| Crate | Version | Purpose |
|-------|---------|---------|
| `tempfile` | 3 | Temporary files/dirs for tests |
| `assert_cmd` | 2 | CLI binary integration testing |
| `predicates` | 3 | Composable output assertions |
| `pretty_assertions` | 1 | Colorized test failure diffs |

## Build & CI
- `cargo build` / `cargo build --release` — compilation
- `cargo test` — run all tests
- `cargo fmt --all -- --check` — format check
- `cargo clippy --all-targets -- -D warnings` — lint check
