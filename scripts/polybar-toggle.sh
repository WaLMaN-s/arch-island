#!/usr/bin/env bash
# Helper script to toggle Polybar on/off
if pgrep -x polybar >/dev/null; then
    echo "==> Stopping Polybar..."
    killall polybar polybar-autohide 2>/dev/null || true
    echo "✓ Polybar disabled."
else
    echo "==> Starting Polybar..."
    if [ -f "$HOME/.config/polybar/launch.sh" ]; then
        "$HOME/.config/polybar/launch.sh" >/dev/null 2>&1 &
    else
        polybar -c "$HOME/.config/polybar/config.ini" main >/dev/null 2>&1 &
    fi
    echo "✓ Polybar enabled."
fi
