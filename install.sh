#!/bin/bash

set -e

# Determine OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Convert architecture names
case $ARCH in
    x86_64)
        ARCH="amd64"
        ;
    aarch64|arm64)
        ARCH="arm64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Set binary name based on OS
case $OS in
    linux)
        BINARY_NAME="stork-asset-linux-$ARCH"
        ;;
    darwin)
        BINARY_NAME="stork-asset-macos-$ARCH"
        ;;
    *)
        echo "Unsupported operating system: $OS"
        exit 1
        ;;
esac

# Get latest release version from GitHub
LATEST_RELEASE=$(curl -s https://api.github.com/repos/henrymbaldwin/stork-asset-cli/releases/latest | grep "tag_name" | cut -d '"' -f 4)

# Create installation directory
INSTALL_DIR="/usr/local/bin"
sudo mkdir -p "$INSTALL_DIR"

echo "Downloading stork-asset..."
# Download the binary
sudo curl -L "https://github.com/YOUR_USERNAME/stork-asset/releases/download/$LATEST_RELEASE/$BINARY_NAME" -o "$INSTALL_DIR/stork-asset"

# Make it executable
sudo chmod +x "$INSTALL_DIR/stork-asset"

echo "stork-asset has been installed to $INSTALL_DIR/stork-asset"
echo "You can now run 'stork-asset' from anywhere in your terminal." 