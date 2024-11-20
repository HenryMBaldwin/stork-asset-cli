#!/bin/bash

set -e

# Determine OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Convert architecture names
case $ARCH in
    x86_64)
        ARCH="amd64"
        ;;
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
LATEST_RELEASE=$(curl -s -H "User-Agent: stork-asset-install-script" https://api.github.com/repos/henrymbaldwin/stork-asset-cli/releases/latest | grep "tag_name" | cut -d '"' -f 4)

# Create installation directory
INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "$INSTALL_DIR"

# Ensure ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo "Adding ~/.local/bin to PATH in your shell configuration..."
    
    # Detect shell and update rc file
    SHELL_RC=""
    if [[ $SHELL == *"zsh"* ]]; then
        SHELL_RC="$HOME/.zshrc"
    elif [[ $SHELL == *"bash"* ]]; then
        SHELL_RC="$HOME/.bashrc"
    fi
    
    if [ -n "$SHELL_RC" ]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
        echo "Please restart your terminal or run: source $SHELL_RC"
    else
        echo "Warning: Could not detect shell configuration file."
        echo "Please add ~/.local/bin to your PATH manually."
    fi
fi

echo "Downloading stork-asset..."
# Download the binary
curl -L "https://github.com/henrymbaldwin/stork-asset-cli/releases/download/$LATEST_RELEASE/$BINARY_NAME" -o "$INSTALL_DIR/stork-asset"

# Make it executable
chmod +x "$INSTALL_DIR/stork-asset"

echo "stork-asset has been installed to $INSTALL_DIR/stork-asset"
echo "You can now run 'stork-asset' from anywhere in your terminal."

# Check if ~/.local/bin is in the current PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo "Note: You may need to restart your terminal or run:"
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
fi 