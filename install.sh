#!/usr/bin/env sh
set -e

require() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo "❌ Missing required dependency: $1"
        echo "👉 $2"
        exit 1
    fi
}

require git "Install from https://git-scm.com"
require python3 "Install Python 3.11+"
require node "Install Node.js from https://nodejs.org"
require npm "npm should come with Node.js"

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
