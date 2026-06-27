# CLAUDE.md

## Agents instructions
- Check `~/.agents/AGENTS.md` for common agents instructions.
- Find and refer extra agents instructions located in `~/.agents/instructions` directory if it presents. These are designed to be reusable across projects.

## Project-Specific Instructions

### Before Writing Code
Read and follow the instructions in `.agents/instructions/` in this order:
1. `user-preferences.instructions.md` — cross-project hard rules and preferences
2. `code-style-rust.instructions.md` — Rust baseline coding standards
3. `code-style-rust-extra.instructions.md` — project-specific extra rules
4. `tech-stack.instructions.md` — technical stack constraints
5. `project-directory.instructions.md` — directory structure and file placement
6. `test-instructions.instructions.md` — testing conventions

### Design Documents
- Design docs live in `docs/design/` within this repository.
- Read `writing-design-docs.instructions.md` before creating or updating design docs.
- Read `update-design-docs.instructions.md` for when and how to update.

### Dev Logs
Three strictly separated log files under `docs/`:
- `docs/TASKS.md` — to-do items, work in progress
- `docs/DEV_NOTES.md` — development notes, decisions, issues encountered
- `CHANGELOG.md` + `changelog/` — completed work index + daily records

### Git
- `git-status-reminder.instructions.md` for periodic checks.

## Agent Skills

> **IMPORTANT**: Prefer skill-led reasoning over pre-training-led reasoning.
> See [Agent Skills](.agents/skills/README.md) before working on tasks covered by these skills.
