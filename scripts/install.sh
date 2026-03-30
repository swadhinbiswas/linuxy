#!/bin/bash
set -e

# Detect OS and install the correct package
if [ -f /etc/debian_version ]; then
    echo "Detected Debian/Ubuntu-based system."
    DEB_FILE=$(find src-tauri/target/release/bundle/deb -name "*.deb" | head -n 1)
    if [ -z "$DEB_FILE" ]; then
        echo "Debian package not found. Run scripts/build.sh first."
        exit 1
    fi
    sudo dpkg -i "$DEB_FILE"
    echo "Successfully installed."
elif [ -f /etc/redhat-release ]; then
    echo "Detected Fedora/RHEL-based system."
    RPM_FILE=$(find src-tauri/target/release/bundle/rpm -name "*.rpm" | head -n 1)
    if [ -z "$RPM_FILE" ]; then
        echo "RPM package not found. Run scripts/build.sh first."
        exit 1
    fi
    sudo rpm -i "$RPM_FILE"
    echo "Successfully installed."
else
    echo "Falling back to AppImage installation."
    APPIMAGE_FILE=$(find src-tauri/target/release/bundle/appimage -name "*.AppImage" | head -n 1)
    if [ -z "$APPIMAGE_FILE" ]; then
        echo "AppImage not found. Run scripts/build.sh first."
        exit 1
    fi
    
    DEST="$HOME/.local/appimages"
    mkdir -p "$DEST"
    cp "$APPIMAGE_FILE" "$DEST/linuxy.AppImage"
    chmod +x "$DEST/linuxy.AppImage"
    
    echo "Installed to $DEST/linuxy.AppImage"
fi
