#!/bin/bash
# Build RPM package from DEB for Linuxy
# Requires: fpm (gem install fpm)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TAURI_TARGET="$PROJECT_ROOT/src-tauri/target"

echo "=== Building RPM from DEB package ==="

# Check if fpm is installed
if ! command -v fpm &> /dev/null; then
    echo "Warning: fpm is not installed. Skipping RPM build."
    echo "To build RPM, install fpm with: gem install fpm"
    echo "Or on Arch Linux: sudo pacman -S ruby base-devel && gem install fpm"
    exit 0
fi

# Find the latest DEB package
DEB_PACKAGE=$(ls -t "$TAURI_TARGET/release/bundle/deb/"*.deb 2>/dev/null | head -n1)

if [ -z "$DEB_PACKAGE" ]; then
    echo "Error: No DEB package found. Run 'npm run tauri build' first."
    exit 1
fi

echo "Found DEB package: $DEB_PACKAGE"

# Create RPM output directory
RPM_OUTPUT="$TAURI_TARGET/release/bundle/rpm"
mkdir -p "$RPM_OUTPUT"

# Extract version from DEB filename
VERSION=$(basename "$DEB_PACKAGE" | sed 's/linuxy_\([0-9.]*\)_amd64.deb/\1/')
echo "Version: $VERSION"

# Convert DEB to RPM using fpm
echo "Converting DEB to RPM..."
fpm --input-type deb \
    --output-type rpm \
    --name linuxy \
    --version "$VERSION" \
    --architecture x86_64 \
    --url "https://github.com/swadhinbiswas/linuxy" \
    --description "One-click Linux Application Manager with Firejail sandboxing" \
    --maintainer "Swadhin Biswas" \
    --license MIT \
    --depends firejail \
    --depends xdg-utils \
    --package "$RPM_OUTPUT/linuxy-${VERSION}-1.x86_64.rpm" \
    "$DEB_PACKAGE"

echo "=== RPM build complete ==="
echo "RPM package: $RPM_OUTPUT/linuxy-${VERSION}-1.x86_64.rpm"
ls -la "$RPM_OUTPUT/"*.rpm
