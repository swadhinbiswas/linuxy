#!/usr/bin/env bash
set -e

# Linuxy Installation Script
# This script detects your OS and installs the appropriate Linuxy package (DEB, RPM, or AppImage).

REPO="swadhinbiswas/linuxy"
API_URL="https://api.github.com/repos/$REPO/releases/latest"

echo "========================================"
echo "    Installing Linuxy Application       "
echo "========================================"
echo ""

# Ensure curl is installed
if ! command -v curl &> /dev/null; then
    echo "Error: curl is required to run this script."
    exit 1
fi

# Detect Architecture
ARCH=$(uname -m)
if [ "$ARCH" = "x86_64" ]; then
    TAURI_ARCH="amd64"
    APPIMAGE_ARCH="amd64"
    RPM_ARCH="x86_64"
else
    echo "Warning: Linuxy officially supports x86_64 architectures."
    echo "Your architecture is $ARCH. The installation might fail if no release is found."
    TAURI_ARCH="$ARCH"
    APPIMAGE_ARCH="$ARCH"
    RPM_ARCH="$ARCH"
fi

echo "Fetching latest release information..."
# Fetch latest release data
RELEASE_DATA=$(curl -fsSL "$API_URL")
if [ -z "$RELEASE_DATA" ]; then
    echo "Error: Failed to fetch release information from GitHub."
    exit 1
fi

# Extract download URLs for different formats
DEB_URL=$(echo "$RELEASE_DATA" | grep -oP '"browser_download_url": "\K(.*\.deb)(?=")' | grep "$TAURI_ARCH" | head -n 1)
RPM_URL=$(echo "$RELEASE_DATA" | grep -oP '"browser_download_url": "\K(.*\.rpm)(?=")' | grep "$RPM_ARCH" | head -n 1)
APPIMAGE_URL=$(echo "$RELEASE_DATA" | grep -oP '"browser_download_url": "\K(.*\.AppImage)(?=")' | grep -i "$APPIMAGE_ARCH" | head -n 1)

# Fallbacks if architecture-specific grep fails (sometimes the naming convention varies)
if [ -z "$DEB_URL" ]; then DEB_URL=$(echo "$RELEASE_DATA" | grep -oP '"browser_download_url": "\K(.*\.deb)(?=")' | head -n 1); fi
if [ -z "$RPM_URL" ]; then RPM_URL=$(echo "$RELEASE_DATA" | grep -oP '"browser_download_url": "\K(.*\.rpm)(?=")' | head -n 1); fi
if [ -z "$APPIMAGE_URL" ]; then APPIMAGE_URL=$(echo "$RELEASE_DATA" | grep -oP '"browser_download_url": "\K(.*\.AppImage)(?=")' | head -n 1); fi

TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

install_deb() {
    echo "Detected Debian/Ubuntu-based system."
    if [ -z "$DEB_URL" ]; then
        echo "No DEB package found for the latest release. Falling back to AppImage..."
        install_appimage
        return
    fi
    echo "Downloading: $DEB_URL"
    curl -fL# -o linuxy.deb "$DEB_URL"
    echo "Installing DEB package (requires sudo)..."
    sudo apt-get install -y ./linuxy.deb
    echo "Linuxy installed successfully!"
}

install_rpm() {
    echo "Detected RPM-based system."
    if [ -z "$RPM_URL" ]; then
        echo "No RPM package found for the latest release. Falling back to AppImage..."
        install_appimage
        return
    fi
    echo "Downloading: $RPM_URL"
    curl -fL# -o linuxy.rpm "$RPM_URL"
    echo "Installing RPM package (requires sudo)..."
    if command -v dnf &> /dev/null; then
        sudo dnf install -y ./linuxy.rpm
    elif command -v zypper &> /dev/null; then
        sudo zypper install -y ./linuxy.rpm
    elif command -v yum &> /dev/null; then
        sudo yum localinstall -y ./linuxy.rpm
    else
        sudo rpm -i ./linuxy.rpm
    fi
    echo "Linuxy installed successfully!"
}

install_appimage() {
    echo "Using universal AppImage format."
    if [ -z "$APPIMAGE_URL" ]; then
        echo "Error: No AppImage found for the latest release."
        exit 1
    fi
    echo "Downloading: $APPIMAGE_URL"
    curl -fL# -o linuxy.AppImage "$APPIMAGE_URL"
    chmod +x linuxy.AppImage
    
    BIN_DIR="$HOME/.local/bin"
    mkdir -p "$BIN_DIR"
    
    echo "Installing to $BIN_DIR/linuxy"
    mv linuxy.AppImage "$BIN_DIR/linuxy"
    
    # Add to PATH if not already there
    if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
        echo "WARNING: $BIN_DIR is not in your PATH."
        echo "You may want to add 'export PATH=\"\$HOME/.local/bin:\$PATH\"' to your ~/.bashrc or ~/.zshrc"
    fi

    echo "Running self-registration to create desktop shortcuts..."
    # Run once in background so it triggers the desktop entry creation
    "$BIN_DIR/linuxy" --help >/dev/null 2>&1 || true

    echo "Linuxy AppImage installed successfully!"
    echo "You can now run 'linuxy' from your terminal or launch it from your application menu."
}

# OS Detection
if command -v apt-get &> /dev/null || command -v dpkg &> /dev/null; then
    install_deb
elif command -v dnf &> /dev/null || command -v zypper &> /dev/null || command -v rpm &> /dev/null; then
    install_rpm
else
    install_appimage
fi

# Cleanup
cd ~
rm -rf "$TMP_DIR"
echo ""
echo "Installation Complete! Welcome to Linuxy."
