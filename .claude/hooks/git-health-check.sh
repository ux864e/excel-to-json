#!/bin/bash
# Git health check: notify if uncommitted changes exist or remote is out of sync.
# Runs as a SessionStart hook.

set -e

PROJECT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TIMESTAMP_FILE="$PROJECT_DIR/.claude/.git-check-timestamp"
SYNC_TIMESTAMP_FILE="$PROJECT_DIR/.claude/.git-sync-timestamp"
CHECK_INTERVAL_HOURS=6

cd "$PROJECT_DIR"

# Skip if not a git repo
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    exit 0
fi

now=$(date +%s)

# --- Uncommitted changes check ---
should_check_uncommitted=true
if [ -f "$TIMESTAMP_FILE" ]; then
    last_check=$(cat "$TIMESTAMP_FILE")
    elapsed=$(( (now - last_check) / 3600 ))
    if [ "$elapsed" -lt "$CHECK_INTERVAL_HOURS" ]; then
        should_check_uncommitted=false
    fi
fi

if $should_check_uncommitted; then
    if ! git diff-index --quiet HEAD -- 2>/dev/null || \
       [ -n "$(git ls-files --others --exclude-standard 2>/dev/null)" ]; then
        echo ""
        echo "┌─────────────────────────────────────────────┐"
        echo "│ ⚠ Git: Uncommitted changes detected          │"
        echo "│ Consider committing your work.               │"
        echo "└─────────────────────────────────────────────┘"
    fi
    echo "$now" > "$TIMESTAMP_FILE"
fi

# --- Remote sync check ---
should_check_sync=true
if [ -f "$SYNC_TIMESTAMP_FILE" ]; then
    last_sync=$(cat "$SYNC_TIMESTAMP_FILE")
    elapsed=$(( (now - last_sync) / 3600 ))
    if [ "$elapsed" -lt "$CHECK_INTERVAL_HOURS" ]; then
        should_check_sync=false
    fi
fi

if $should_check_sync; then
    remote=$(git remote get-url origin 2>/dev/null || true)
    if [ -n "$remote" ]; then
        # Fetch silently
        git fetch origin --quiet 2>/dev/null || true

        behind=$(git rev-list HEAD..origin/main --count 2>/dev/null || echo 0)
        ahead=$(git rev-list origin/main..HEAD --count 2>/dev/null || echo 0)

        if [ "$behind" -gt 0 ] 2>/dev/null; then
            echo ""
            echo "┌─────────────────────────────────────────────┐"
            echo "│ ⚠ Git: Remote is $behind commit(s) ahead.       │"
            echo "│ Consider running: git pull                   │"
            echo "└─────────────────────────────────────────────┘"
        fi
        if [ "$ahead" -gt 0 ] 2>/dev/null; then
            echo ""
            echo "┌─────────────────────────────────────────────┐"
            echo "│ ⚠ Git: Local is $ahead commit(s) ahead of remote.│"
            echo "│ Consider running: git push                   │"
            echo "└─────────────────────────────────────────────┘"
        fi
    fi
    echo "$now" > "$SYNC_TIMESTAMP_FILE"
fi
