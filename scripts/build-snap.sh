#!/bin/bash
set -e

echo "=== Building Linuxy Snap Package ==="

# Check for snapcraft
if ! command -v snapcraft &> /dev/null; then
    echo "snapcraft is not installed. Install it with:"
    echo "  sudo apt install snapcraft"
    echo "  or"
    echo "  sudo snap install snapcraft --classic"
    exit 1
fi

cd "$(dirname "$0")/.."

echo "Building snap package..."
snapcraft --use-lxd 2>&1

echo ""
echo "=== Snap build complete ==="
echo "Install with: sudo snap install ./linuxy_*.snap --dangerous"
