# User Preferences

## Hard Rules (non-negotiable, across all projects)

1. **No hardcoded values**: All constants, strings, and config values must be declared in dedicated config files or constant modules. Never inline magic numbers or strings.
2. **No silent catches**: Every caught error must be logged or propagated. Never swallow exceptions silently.
3. **No FIXME/HACK/TODO in production code**: Use `NOTE(username):` for intentional deviations with explanation.
4. **English-only identifiers**: All variable names, function names, type names, and code comments must be in English.
5. **Prod logs at appropriate level**: Production logs use `info`/`warn`/`error` only. `debug`/`trace` for development.

## Dev Rhythm
- **Solo serial mode**: One feature at a time, complete before moving to next.
- **Tests after module logic**: Not strict TDD. Write tests after core logic is stable (unless explicitly asked for TDD).
- **Test ratio target**: Unit:Integration = 7:3 for CLI tools.

## Error Handling
- **Rust CLI**: `anyhow::Result` for application code; `thiserror` for library error enums.
- **Propagation with context**: Use `.context()` / `.with_context()` instead of bare `?` when the error chain needs clarity.
- **User-facing errors**: Clear, actionable error messages on stderr. No stack traces by default (use `RUST_BACKTRACE=1` for debugging).

## Git Workflow
- **Solo mode**: Commit directly to `main`. Push after logical units.
- **Team mode**: Feature branches + Pull Requests via GitHub Flow.
- **Commit style**: Conventional Commits (`feat:`, `fix:`, `refactor:`, `chore:`, `docs:`, `test:`).
- **Commit cadence**: Commit when a logical unit is complete OR >6 hours since last OR end of session.

## Session Workflows
### Start of Session
1. Run `git status` to check working tree state.
2. Review `TASKS.md` for pending items.
3. Review `DEV_NOTES.md` for recent decisions/issues.

### End of Session
1. Update `CHANGELOG.md` with completed work.
2. Update `TASKS.md` with remaining/pending items.
3. Update `DEV_NOTES.md` with any decisions or issues.
4. Commit and push all changes.
