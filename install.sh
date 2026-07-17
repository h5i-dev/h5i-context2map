#!/usr/bin/env sh
set -e

REPO="h5i-dev/h5i-ctx2img"
BINARY="ctx2img"
INSTALL_DIR="${CTX2IMG_INSTALL_DIR:-/usr/local/bin}"

# ── detect OS ────────────────────────────────────────────────────────────────
OS="$(uname -s)"
case "$OS" in
  Linux)  os="linux" ;;
  Darwin) os="macos" ;;
  *)
    echo "Unsupported OS: $OS (Windows: grab the zip from https://github.com/${REPO}/releases)" >&2
    exit 1
    ;;
esac

# ── detect arch ──────────────────────────────────────────────────────────────
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64 | amd64)  arch="x86_64" ;;
  arm64 | aarch64) arch="aarch64" ;;
  *)
    echo "Unsupported architecture: $ARCH" >&2
    exit 1
    ;;
esac

# ── map to release target triple ─────────────────────────────────────────────
case "${os}-${arch}" in
  linux-x86_64)  target="x86_64-unknown-linux-gnu" ;;
  macos-x86_64)  target="x86_64-apple-darwin" ;;
  macos-aarch64) target="aarch64-apple-darwin" ;;
  *)
    echo "No prebuilt binary for ${os}-${arch} — build from source: cargo install --path crates/ctx2img-cli" >&2
    exit 1
    ;;
esac

# ── resolve latest version ───────────────────────────────────────────────────
VERSION="${CTX2IMG_VERSION:-}"
if [ -z "$VERSION" ]; then
  VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name"' | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')"
fi

if [ -z "$VERSION" ]; then
  echo "Could not determine latest version. Set CTX2IMG_VERSION=vX.Y.Z to override." >&2
  exit 1
fi

# ── download and install ─────────────────────────────────────────────────────
STAGING="${BINARY}-${VERSION}-${target}"
URL="https://github.com/${REPO}/releases/download/${VERSION}/${STAGING}.tar.gz"

echo "Installing ${BINARY} ${VERSION} (${target}) → ${INSTALL_DIR}/${BINARY}"

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

curl -fsSL "$URL" -o "${TMP}/${STAGING}.tar.gz"
tar -xzf "${TMP}/${STAGING}.tar.gz" -C "$TMP"

if [ -w "$INSTALL_DIR" ]; then
  mv "${TMP}/${STAGING}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
else
  sudo mv "${TMP}/${STAGING}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
fi

echo "✔  ${BINARY} ${VERSION} installed — run: ${BINARY} paint --help"
