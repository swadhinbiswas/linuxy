#!/bin/bash
# Build RPM package from DEB for Linuxy
# Requires: fpm (gem install fpm)
# Run scripts/build.sh first to generate DEB

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TAURI_TARGET="$PROJECT_ROOT/src-tauri/target"

echo "=== Building RPM from DEB package ==="

# Check if fpm is installed and usable
if ! command -v fpm &> /dev/null; then
    echo "Error: fpm is not installed."
    echo "To build RPM, install fpm with: gem install fpm"
    echo "Or on Arch Linux: sudo pacman -S ruby base-devel && gem install fpm"
    exit 1
fi

# Check if rpmbuild is available
if ! command -v rpmbuild &> /dev/null; then
    echo "Error: rpmbuild is not installed."
    echo "Install it with:"
    echo "  Debian/Ubuntu: sudo apt install rpm"
    echo "  Fedora: sudo dnf install rpm-build"
    echo "  Arch: sudo pacman -S rpm-tools"
    echo "  Alpine: sudo apk add rpm-build"
    exit 1
fi

if ! fpm --version &> /dev/null; then
    echo "Error: fpm is present in PATH but unusable."
    echo "The Ruby environment cannot load the fpm gem for this executable."
    echo "In CI, install it system-wide with: sudo gem install --no-document fpm"
    exit 1
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

# Extract version from DEB filename (supports linuxy_X.Y.Z_amd64.deb and Linuxy_X.Y.Z_amd64.deb)
VERSION=$(basename "$DEB_PACKAGE" | sed 's/[Ll]inuxy_\([0-9.]*\)_amd64.deb/\1/')
echo "Version: $VERSION"

# Convert DEB to RPM using fpm
echo "Converting DEB to RPM..."
fpm --input-type deb \
    --output-type rpm \
    --no-auto-depends \
    --name linuxy \
    --version "$VERSION" \
    --architecture x86_64 \
    --url "https://github.com/swadhinbiswas/linuxy" \
    --description "One-click Linux Application Manager with Firejail sandboxing" \
    --maintainer "Swadhin Biswas" \
    --license MIT \
    --depends firejail \
    --depends xdg-utils \
    --depends gtk3 \
    --depends webkit2gtk4.1 \
    --depends libappindicator-gtk3 \
    --package "$RPM_OUTPUT/linuxy-${VERSION}-1.x86_64.rpm" \
    "$DEB_PACKAGE"

echo "=== RPM build complete ==="
echo "RPM package: $RPM_OUTPUT/linuxy-${VERSION}-1.x86_64.rpm"
ls -la "$RPM_OUTPUT/"*.rpm
