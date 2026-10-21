#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$SCRIPT_DIR"

echo "==> Building frontend assets..."
npm run build

echo "==> Compiling optimized release binary..."
cargo build --release --manifest-path src-tauri/Cargo.toml

echo "✓ Release binary ready at: src-tauri/target/release/arch-island"
