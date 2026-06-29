#!/usr/bin/env bash
#
# setup-rust.sh — Install and initialize the Rust toolchain.
#
# Idempotent: safe to run even if Rust is already installed.
# Falls back to USTC mirrors if the default installer times out.
#
# Usage:
#   ./scripts/setup-rust.sh

set -euo pipefail

RUSTUP_TIMEOUT=60  # seconds before falling back to USTC mirror

echo "==> Checking Rust environment..."

# ── Install rustup if missing ────────────────────────────────────────────────

if ! command -v rustup &>/dev/null; then
    echo "    rustup not found. Installing..."

    install_with_timeout() {
        # Run rustup installer with a timeout. If it exceeds RUSTUP_TIMEOUT
        # seconds (likely stuck on official servers), kill it and return 1.
        local pid
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
            | sh -s -- -y --default-toolchain stable &
        pid=$!

        local elapsed=0
        while kill -0 "$pid" 2>/dev/null; do
            sleep 2
            elapsed=$((elapsed + 2))
            if [[ $elapsed -ge $RUSTUP_TIMEOUT ]]; then
                echo ""
                echo "    Timed out after ${RUSTUP_TIMEOUT}s. Switching to USTC mirror..."
                kill "$pid" 2>/dev/null || true
                wait "$pid" 2>/dev/null || true
                return 1
            fi
        done

        wait "$pid"
    }

    if ! install_with_timeout; then
        export RUSTUP_DIST_SERVER="https://mirrors.ustc.edu.cn/rust-static"
        export RUSTUP_UPDATE_ROOT="https://mirrors.ustc.edu.cn/rust-static/rustup"
        echo "    Retrying with USTC mirror..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
            | sh -s -- -y --default-toolchain stable
    fi

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

# ── Ensure PATH in shell profile ─────────────────────────────────────────────

CARGO_PATH_LINE='export PATH="$HOME/.cargo/bin:$PATH"'

ensure_path_in_profile() {
    local profile="$1"
    if [[ -f "$profile" ]]; then
        if grep -q '.cargo/bin' "$profile"; then
            return 0  # already configured
        fi
    fi
    return 1  # not configured
}

# Determine which profile file to update.
if [[ "$OS" == "Darwin" ]]; then
    PROFILE="$HOME/.zshrc"
else
    PROFILE="$HOME/.bashrc"
fi

if ! ensure_path_in_profile "$PROFILE"; then
    echo ""
    echo "==> Adding Rust to PATH in $PROFILE"
    {
        echo ""
        echo "# Rust"
        echo "$CARGO_PATH_LINE"
    } >>"$PROFILE"
    echo "    Added: $CARGO_PATH_LINE"
else
    echo "    PATH already configured in $PROFILE"
fi
