#!/bin/bash
# Helper script to update AUR package version
# Usage: ./scripts/update-aur.sh 1.0.0

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 1.0.0"
    exit 1
fi

VERSION="$1"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AUR_DIR="$(dirname "$SCRIPT_DIR")/packaging/aur"

echo "Updating AUR package to version $VERSION..."

# Update PKGBUILD
sed -i "s/^pkgver=.*/pkgver=$VERSION/" "$AUR_DIR/PKGBUILD"
sed -i "s|linuxy-.*\.tar.gz::|linuxy-$VERSION.tar.gz::|" "$AUR_DIR/PKGBUILD"
sed -i "s|refs/tags/v.*|refs/tags/v$VERSION.tar.gz|" "$AUR_DIR/PKGBUILD"

# Update .SRCINFO
sed -i "s/^	pkgver = .*/	pkgver = $VERSION/" "$AUR_DIR/.SRCINFO"
sed -i "s|linuxy-.*\.tar.gz::|linuxy-$VERSION.tar.gz::|" "$AUR_DIR/.SRCINFO"
sed -i "s|refs/tags/v.*|refs/tags/v$VERSION.tar.gz|" "$AUR_DIR/.SRCINFO"

echo "✓ Updated PKGBUILD and .SRCINFO"
echo ""
echo "Next steps:"
echo "1. Review changes: git diff packaging/aur/"
echo "2. Update .SRCINFO checksums: cd packaging/aur && makepkg --printsrcinfo > .SRCINFO"
echo "3. Commit and push to AUR"
