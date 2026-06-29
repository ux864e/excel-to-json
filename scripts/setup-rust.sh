#!/usr/bin/env bash
#
# setup-rust.sh — Install and initialize the Rust toolchain.
#
# Idempotent: safe to run even if Rust is already installed.
#
# Usage:
#   ./scripts/setup-rust.sh

set -euo pipefail

echo "==> Checking Rust environment..."

# ── Install rustup if missing ────────────────────────────────────────────────

if ! command -v rustup &>/dev/null; then
    echo "    rustup not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    # shellcheck source=/dev/null
    source "$HOME/.cargo/env"
else
    echo "    rustup: $(rustup --version)"
fi

# ── Ensure stable toolchain ──────────────────────────────────────────────────

if ! rustup toolchain list | grep -q "stable"; then
    echo "    Installing stable toolchain..."
    rustup toolchain install stable
fi

rustup default stable
echo "    Toolchain: $(rustup default)"

# ── Update ───────────────────────────────────────────────────────────────────

rustup update

# ── Install platform targets ─────────────────────────────────────────────────

ARCH=$(uname -m)
OS=$(uname -s)

case "$OS-$ARCH" in
    Darwin-arm64)
        TARGETS=("aarch64-apple-darwin" "x86_64-apple-darwin")
        ;;
    Darwin-x86_64)
        TARGETS=("x86_64-apple-darwin" "aarch64-apple-darwin")
        ;;
    Linux-x86_64)
        TARGETS=("x86_64-unknown-linux-gnu")
        ;;
    Linux-aarch64)
        TARGETS=("aarch64-unknown-linux-gnu")
        ;;
    *)
        echo "ERROR: Unsupported platform: $OS-$ARCH" >&2
        exit 1
        ;;
esac

for target in "${TARGETS[@]}"; do
    if ! rustup target list --installed | grep -q "$target"; then
        echo "    Installing target: $target"
        rustup target add "$target"
    else
        echo "    Target already installed: $target"
    fi
done

echo ""
echo "==> Rust environment ready."
rustc --version
cargo --version
echo "    Installed targets:"
rustup target list --installed
