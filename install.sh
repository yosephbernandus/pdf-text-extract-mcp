#!/bin/sh
set -e

REPO="yosephbernandus/pdf-text-extract-mcp"
INSTALL_DIR="/usr/local/bin"

# Detect OS and arch
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux)  PLATFORM="linux" ;;
  darwin) PLATFORM="darwin" ;;
  *)      echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
  x86_64|amd64)  ARCH="x86_64" ;;
  aarch64|arm64) ARCH="aarch64" ;;
  *)             echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

BINARY="pdf-mcp-${PLATFORM}-${ARCH}"
URL="https://github.com/${REPO}/releases/latest/download/${BINARY}"

echo "Downloading ${BINARY}..."
curl -fsSL "$URL" -o pdf-mcp
chmod +x pdf-mcp
sudo mv pdf-mcp "$INSTALL_DIR/pdf-mcp"
echo "Installed pdf-mcp to ${INSTALL_DIR}/pdf-mcp"
echo ""
echo "Add to Claude Code:"
echo "  claude mcp add pdf-text-extract -- pdf-mcp"
