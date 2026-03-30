#!/bin/bash
set -e

echo "Starting build process for linuxy..."

echo "1. Installing Node dependencies..."
npm install

echo "2. Building React frontend..."
npm run build

echo "3. Building Tauri application (AppImage, deb, rpm)..."
# This requires Rust and Cargo to be installed
npx tauri build

echo "Build complete!"
echo "Outputs:"
echo "- AppImage: src-tauri/target/release/bundle/appimage/"
echo "- Debian (.deb): src-tauri/target/release/bundle/deb/"
echo "- RPM (.rpm): src-tauri/target/release/bundle/rpm/"
