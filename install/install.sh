#!/bin/bash
# PohLang Installation Script for Linux/macOS
# Usage: curl -sSL https://raw.githubusercontent.com/AlhaqGH/PohLang/main/install/install.sh | bash

set -e

VERSION="v0.5.2"
REPO="AlhaqGH/PohLang"

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
    PLATFORM="linux-x64"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
    PLATFORM="macos-x64"
else
    echo "❌ Unsupported operating system: $OSTYPE"
    exit 1
fi

echo "🚀 Installing PohLang $VERSION for $OS..."
echo ""

# Create temp directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Download
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/pohlang-$VERSION-$PLATFORM.tar.gz"
echo "📥 Downloading from $DOWNLOAD_URL..."
curl -L -o pohlang.tar.gz "$DOWNLOAD_URL"

# Extract
echo "📦 Extracting..."
tar -xzf pohlang.tar.gz

# Install to /usr/local/bin (requires sudo)
echo "📋 Installing to /usr/local/bin (may require sudo password)..."
sudo mv pohlang /usr/local/bin/

# Verify installation
if command -v pohlang &> /dev/null; then
    echo ""
    echo "✅ PohLang installed successfully!"
    echo ""
    echo "Try it out:"
    echo "  pohlang --version"
    echo ""
    echo "Create a test program:"
    echo "  echo 'Start Program' > hello.poh"
    echo "  echo 'Write \"Hello from PohLang!\"' >> hello.poh"
    echo "  echo 'End Program' >> hello.poh"
    echo "  pohlang --run hello.poh"
    echo ""
    echo "📚 Documentation: https://github.com/$REPO"
else
    echo "❌ Installation failed. Please install manually from:"
    echo "   https://github.com/$REPO/releases"
    exit 1
fi

# Cleanup
cd /
rm -rf "$TMP_DIR"
