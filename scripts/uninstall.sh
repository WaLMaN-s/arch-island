#!/usr/bin/env bash
set -e

echo "==> Uninstalling ARCH Island..."

# Kill any running instance
pkill -f "arch-island" || true

rm -f "$HOME/.local/bin/arch-island"
rm -f "$HOME/.local/share/applications/arch-island.desktop"
rm -f "/tmp/arch-island.lock"

echo "✓ Removed launcher binary and desktop entry."
echo "Note: Project source code in ~/Projects/ARCH or current folder remains intact."
echo "If you added 'exec --no-startup-id arch-island' to ~/.config/i3/config, remember to remove that line."
echo "Done!"
