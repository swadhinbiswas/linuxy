#!/bin/bash
set -e

echo "=== Building Linuxy Flatpak Package ==="

# Check for flatpak-builder
if ! command -v flatpak-builder &> /dev/null; then
    echo "flatpak-builder is not installed. Install it with:"
    echo "  sudo apt install flatpak-builder"
    echo "  or"
    echo "  sudo dnf install flatpak-builder"
    exit 1
fi

cd "$(dirname "$0")/.."

# Ensure Flathub remote is added
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

echo "Building flatpak..."
flatpak-builder --force-clean --user --install-deps-from=flathub --install build-dir flatpak/com.linuxy.App.yaml 2>&1

echo ""
echo "=== Flatpak build complete ==="
echo "Run with: flatpak run com.linuxy.App"
