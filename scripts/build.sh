#!/bin/bash
set -e

echo "Starting build process for linuxy (Tauri 2)..."

echo "1. Installing Node dependencies..."
bun install

echo "2. Building React frontend..."
bun run build

echo "3. Building Tauri application..."

# Set NO_STRIP=1 for linuxdeploy on systems with binutils >= 2.42
# (avoids .relr.dyn section stripping errors on Fedora 43+)
export NO_STRIP=1
export APPIMAGE_EXTRACT_AND_RUN=1
export ARCH=x86_64

bunx tauri build

echo "Build complete!"
echo "Outputs:"
echo "- AppImage: src-tauri/target/release/bundle/appimage/"
echo "- Debian (.deb): src-tauri/target/release/bundle/deb/"
echo "- RPM (.rpm): src-tauri/target/release/bundle/rpm/"
if [ -d "src-tauri/target/release/bundle/msi" ]; then
  echo "- Windows MSI: src-tauri/target/release/bundle/msi/"
fi
if [ -d "src-tauri/target/release/bundle/nsis" ]; then
  echo "- Windows NSIS: src-tauri/target/release/bundle/nsis/"
fi
if [ -d "src-tauri/target/release/bundle/dmg" ]; then
  echo "- macOS DMG: src-tauri/target/release/bundle/dmg/"
fi
