#!/bin/bash
# Linuxy DEB Package Signing Script
# Open Source Project - Community Maintained

set -e

PACKAGE_NAME="linuxy"
VERSION="0.1.0"
MAINTAINER="Swadhin Biswas"
EMAIL="swadhinbiswas.cse@gmail.com"

echo "🔐 Linuxy Package Signing Script"
echo "================================"
echo ""

# Check if GPG is installed
if ! command -v gpg &> /dev/null; then
    echo "❌ GPG is not installed. Please install it first:"
    echo "   sudo apt install gnupg"
    exit 1
fi

# Check if GPG key exists
GPG_KEY=$(gpg --list-secret-keys --keyid-format LONG | grep "^sec" | head -1 | awk '{print $2}')

if [ -z "$GPG_KEY" ]; then
    echo "⚠️  No GPG signing key found. Creating one..."
    echo ""
    
    # Create GPG key batch file
    cat > gpg_key_batch <<EOF
%echo Generating GPG key for Linuxy
Key-Type: RSA
Key-Length: 4096
Subkey-Type: RSA
Subkey-Length: 4096
Name-Real: $MAINTAINER
Name-Email: $EMAIL
Expire-Date: 0
%no-protection
%commit
%echo Done
EOF
    
    gpg --batch --gen-key gpg_key_batch
    rm gpg_key_batch
    
    GPG_KEY=$(gpg --list-secret-keys --keyid-format LONG | grep "^sec" | head -1 | awk '{print $2}')
    echo "✅ GPG key created: $GPG_KEY"
fi

echo "✅ Using GPG key: $GPG_KEY"
echo ""

# Find the DEB file
DEB_FILE=$(find src-tauri/target/release/bundle/deb -name "*.deb" 2>/dev/null | head -1)

if [ -z "$DEB_FILE" ]; then
    echo "❌ No DEB package found. Please build first:"
    echo "   bun run tauri build"
    exit 1
fi

echo "📦 Found package: $DEB_FILE"
echo ""

# Sign the package
echo "🔏 Signing package..."
dpkg-sig --sign builder -k $GPG_KEY "$DEB_FILE"

if [ $? -eq 0 ]; then
    echo "✅ Package signed successfully!"
    echo ""
    echo "📝 Verify signature with:"
    echo "   dpkg-sig --verify $DEB_FILE"
else
    echo "❌ Failed to sign package"
    echo "💡 Install dpkg-sig: sudo apt install dpkg-sig"
    exit 1
fi

echo ""
echo "✨ Done! The signed package is ready for distribution."
