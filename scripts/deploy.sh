#!/usr/bin/env bash
#
# deploy.sh — Build and deploy the excel-to-json binary to a target directory.
#
# Usage:
#   ./scripts/deploy.sh              # deploy with default settings
#   TARGET_DIR=/opt/tools ./scripts/deploy.sh   # override target directory
#
# The binary is renamed to "config-importer" (plus .exe on Windows).

set -euo pipefail

# ── Configuration ────────────────────────────────────────────────────────────

# Target directory for the deployed binary (relative to the repo root).
# Override via environment variable: TARGET_DIR=/some/path ./scripts/deploy.sh
TARGET_DIR="${TARGET_DIR:-../Cuddle/cuddle-app-backend/tools}"

# Name of the deployed binary (no extension — platform suffix is added automatically).
DEPLOY_NAME="config-importer"

# ── Resolve paths ────────────────────────────────────────────────────────────

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
TARGET_DIR="$(cd "$REPO_ROOT" && mkdir -p "$TARGET_DIR" && cd "$TARGET_DIR" && pwd)"

# ── Platform detection ───────────────────────────────────────────────────────

case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*)
        EXT=".exe"
        ;;
    *)
        EXT=""
        ;;
esac

BINARY_SRC="$REPO_ROOT/target/release/excel-to-json"
BINARY_DST="$TARGET_DIR/$DEPLOY_NAME$EXT"

# ── Build ────────────────────────────────────────────────────────────────────

echo "==> Building release binary..."
cd "$REPO_ROOT"
cargo build --release

if [[ ! -f "$BINARY_SRC" ]]; then
    echo "ERROR: Build did not produce expected binary at: $BINARY_SRC" >&2
    exit 1
fi

echo "    Binary: $BINARY_SRC"
echo "    Size:   $(du -h "$BINARY_SRC" | cut -f1)"

# ── Deploy ───────────────────────────────────────────────────────────────────

echo "==> Deploying to: $BINARY_DST"
cp "$BINARY_SRC" "$BINARY_DST"
chmod +x "$BINARY_DST"

echo "==> Done: $BINARY_DST"
ls -lh "$BINARY_DST"
