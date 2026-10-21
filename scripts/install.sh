#!/usr/bin/env bash
set -e

echo "=========================================================="
echo "  PROJECT ARCH - Modern Dynamic Island for Arch Linux + i3"
echo "  Installer & Setup Script"
echo "=========================================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$SCRIPT_DIR"

echo "==> 1. Checking system dependencies..."
MISSING_DEPS=()

for cmd in rustc cargo node npm pkg-config playerctl brightnessctl pactl nmcli picom; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        MISSING_DEPS+=("$cmd")
    fi
done

if [ ${#MISSING_DEPS[@]} -ne 0 ]; then
    echo "⚠️ Missing dependencies detected: ${MISSING_DEPS[*]}"
    echo "Please install them via pacman:"
    echo "  sudo pacman -S --needed rust nodejs npm webkit2gtk-4.1 playerctl brightnessctl libpulse networkmanager picom"
    exit 1
fi
echo "✓ All core CLI dependencies present."

echo "==> 2. Installing Frontend Node dependencies..."
npm install

echo "==> 3. Building ARCH Island production binary..."
npm run build
cargo build --release --manifest-path src-tauri/Cargo.toml

RELEASE_BIN="$SCRIPT_DIR/src-tauri/target/release/arch-island"

if [ ! -f "$RELEASE_BIN" ]; then
    echo "❌ Error: Binary not found at $RELEASE_BIN"
    exit 1
fi

echo "==> 4. Installing launcher to ~/.local/bin/arch-island..."
mkdir -p "$HOME/.local/bin"
cp -f "$RELEASE_BIN" "$HOME/.local/bin/arch-island"
chmod +x "$HOME/.local/bin/arch-island"

echo "==> 5. Creating Desktop Entry for Rofi/Application Launchers..."
mkdir -p "$HOME/.local/share/applications"
cat > "$HOME/.local/share/applications/arch-island.desktop" <<EOF
[Desktop Entry]
Name=ARCH Island
Comment=Dynamic Island for Arch Linux i3wm
Exec=$HOME/.local/bin/arch-island
Icon=utilities-system-monitor
Terminal=false
Type=Application
Categories=Utility;System;
EOF

chmod +x "$HOME/.local/share/applications/arch-island.desktop"

echo "=========================================================="
echo "🎉 Installation completed successfully!"
echo "You can now run:"
echo "  arch-island"
echo "or launch it from Rofi."
echo ""
echo "To enable autostart in i3, add this to ~/.config/i3/config:"
echo "  exec --no-startup-id arch-island"
echo "=========================================================="
