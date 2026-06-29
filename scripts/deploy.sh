#!/usr/bin/env bash
#
# deploy.sh — Build and deploy config-importer for the current platform.
#
# Usage:
#   ./scripts/deploy.sh
#   TARGET_DIR=/opt/tools ./scripts/deploy.sh

set -euo pipefail

# ── Configuration ────────────────────────────────────────────────────────────

TARGET_DIR="${TARGET_DIR:-../cuddle-service/cuddle-app-backend/tools}"
DEPLOY_NAME="config-importer"

# ── Resolve paths ────────────────────────────────────────────────────────────

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
mkdir -p "$TARGET_DIR"
TARGET_DIR="$(cd "$TARGET_DIR" && pwd)"

cd "$REPO_ROOT"

# ── Ensure Rust is installed ─────────────────────────────────────────────────

if ! command -v cargo &>/dev/null; then
    echo "==> Rust not found. Running setup..."
    bash "$SCRIPT_DIR/setup-rust.sh"
    # shellcheck source=/dev/null
    source "$HOME/.cargo/env"
fi

# ── Detect platform ──────────────────────────────────────────────────────────

ARCH=$(uname -m)
OS=$(uname -s)

case "$OS-$ARCH" in
    Darwin-arm64)
        SUFFIX="darwin-arm64"
        TARGET="aarch64-apple-darwin"
        ;;
    Darwin-x86_64)
        SUFFIX="darwin-x86_64"
        TARGET="x86_64-apple-darwin"
        ;;
    Linux-x86_64)
        SUFFIX="linux-x86_64"
        TARGET="x86_64-unknown-linux-gnu"
        ;;
    Linux-aarch64)
        SUFFIX="linux-arm64"
        TARGET="aarch64-unknown-linux-gnu"
        ;;
    *)
        echo "ERROR: Unsupported platform: $OS-$ARCH" >&2
        exit 1
        ;;
esac

echo "==> Platform: $SUFFIX"

# ── Build ────────────────────────────────────────────────────────────────────

if ! rustup target list --installed | grep -q "$TARGET"; then
    echo "    Installing target: $TARGET"
    rustup target add "$TARGET"
fi

cargo build --release --target "$TARGET"

# ── Deploy ───────────────────────────────────────────────────────────────────

SRC="$REPO_ROOT/target/$TARGET/release/excel-to-json"
DST="$TARGET_DIR/$DEPLOY_NAME-$SUFFIX"

if [[ ! -f "$SRC" ]]; then
    echo "ERROR: Build did not produce expected binary at: $SRC" >&2
    exit 1
fi

cp "$SRC" "$DST"
chmod +x "$DST"

echo "==> Deployed: $DST ($(du -h "$DST" | cut -f1))"
