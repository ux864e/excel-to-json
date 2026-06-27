# Git Status Reminder

## Periodic Checks
- Check for uncommitted changes every 6 hours during active development.
- Check remote sync status every 6 hours.
- If uncommitted changes exist: notify user with file list.
- If local is ahead of remote: remind to push.
- If remote is ahead of local: remind to pull.

## Solo Mode Rules
- Direct commits to `main` are acceptable.
- Commit at logical boundaries (feature complete, fix applied, refactor done).
- Push after each commit session.

## Commit Message Format
Use [Conventional Commits](https://www.conventionalcommits.org/):
```
<type>: <description>

<optional body>
```

Types: `feat`, `fix`, `refactor`, `chore`, `docs`, `test`, `style`, `perf`.

Examples:
- `feat: add recursive directory traversal`
- `fix: handle empty Excel sheets gracefully`
- `refactor: extract mapping logic to separate module`
- `chore: update calamine to 0.26`
- `docs: add design docs for config-driven mapping`

## Git Health Hook
The `SessionStart` hook in `.claude/settings.local.json` runs `.claude/hooks/git-health-check.sh` to automate these checks.
