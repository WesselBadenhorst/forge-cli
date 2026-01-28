#!/usr/bin/env sh
set -e

REPO="WesselBadenhorst/forge-cli"
BIN_NAME="forge"
INSTALL_DIR="/usr/local/bin"

echo "🔥 Installing Forge..."

# Detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$ARCH" in
    x86_64) ARCH="x86_64" ;;
    arm64|aarch64) ARCH="aarch64" ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

case "$OS" in
    linux) OS="unknown-linux-gnu" ;;
    darwin) OS="apple-darwin" ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

TARGET="$ARCH-$OS"
URL="https://github.com/$REPO/releases/latest/download/forge-$TARGET"

echo "➡️  Downloading Forge for $TARGET"
curl -fsSL "$URL" -o forge

chmod +x forge

echo "➡️  Installing to $INSTALL_DIR (may require sudo)"
sudo mv forge "$INSTALL_DIR/$BIN_NAME"

echo "✅ Forge installed successfully"
echo ""
echo "Run:"
echo "  forge --help"
