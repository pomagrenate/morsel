#!/bin/bash
set -e

VERSION="${VERSION:-latest}"
REPO="yourusername/morsel"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

echo "Installing Morsel $VERSION..."

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)
    PLATFORM="linux"
    ;;
  Darwin)
    PLATFORM="macos"
    ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64)
    ARCH_SUFFIX="x86_64"
    ;;
  aarch64|arm64)
    ARCH_SUFFIX="arm64"
    ;;
  *)
    echo "Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

ASSET_NAME="morsel-${PLATFORM}-${ARCH_SUFFIX}"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET_NAME}.tar.gz"

echo "Downloading from $DOWNLOAD_URL..."

# Create temp directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Download
if command -v curl > /dev/null; then
  curl -L -o morsel.tar.gz "$DOWNLOAD_URL"
elif command -v wget > /dev/null; then
  wget -O morsel.tar.gz "$DOWNLOAD_URL"
else
  echo "Neither curl nor wget found"
  exit 1
fi

# Extract
tar xzf morsel.tar.gz

# Install
mkdir -p "$INSTALL_DIR"
mv morsel "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/morsel"

# Cleanup
cd -
rm -rf "$TMP_DIR"

echo "Morsel installed to $INSTALL_DIR/morsel"
echo "Add $INSTALL_DIR to your PATH if not already added"
