#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$SCRIPT_DIR"

echo "⚡ [1/3] Building Svelte frontend..."
npm run build

echo "⚡ [2/3] Compiling ARCH Island binary..."
cargo build --release --manifest-path src-tauri/Cargo.toml

echo "⚡ [3/3] Updating ~/.local/bin/arch-island & restarting..."
killall -9 arch-island 2>/dev/null || true
while pgrep -x arch-island >/dev/null; do sleep 0.1; done

cp -f "$SCRIPT_DIR/src-tauri/target/release/arch-island" "$HOME/.local/bin/arch-island"
chmod +x "$HOME/.local/bin/arch-island"

i3-msg exec "$HOME/.local/bin/arch-island"

echo "✓ ARCH Island updated and restarted successfully!"
