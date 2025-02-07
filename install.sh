#!/usr/bin/env bash
set -e

########################################
# Repository

REPO_OWNER="MikyStar"
REPO_NAME="Sabita"

########################################

# Determine platform target

OS=$(uname -s)
ARCH=$(uname -m)
TARGET=""

case "$OS" in
    Linux)
        if [ "$ARCH" = "x86_64" ]; then
            TARGET="x86_64-unknown-linux-musl"
        elif [ "$ARCH" = "aarch64" ]; then
            TARGET="aarch64-unknown-linux-gnu"
        else
            echo "Unsupported Linux architecture: $ARCH"
            exit 1
        fi
        ;;
    Darwin)
        if [ "$ARCH" = "x86_64" ]; then
            TARGET="x86_64-apple-darwin"
        elif [ "$ARCH" = "arm64" ]; then
            TARGET="aarch64-apple-darwin"
        else
            echo "Unsupported macOS architecture: $ARCH"
            exit 1
        fi
        ;;
    # TODO
    # MINGW*|MSYS*|CYGWIN*)
    #     if [ "$ARCH" = "x86_64" ]; then
    #         TARGET="x86_64-pc-windows-msvc"
    #     elif [ "$ARCH" = "aarch64" ]; then
    #         TARGET="aarch64-pc-windows-msvc"
    #     else
    #         echo "Unsupported Windows architecture: $ARCH"
    #         exit 1
    #     fi
    #     ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

echo "Detected target: $TARGET"

###################

# Get release data

API_URL="https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest"

RELEASE_JSON=$(curl -s "$API_URL")

# Search appropriate binary

BIN_URL=$(echo "$RELEASE_JSON" | jq -r --arg TARGET "$TARGET" '.assets[] | select(.name | test($TARGET)) | .browser_download_url')

if [ -z "$BIN_URL" ]; then
    echo "No asset found for target: $TARGET"
    exit 1
fi

echo "Found asset URL: $BIN_URL"

###################

# Install

BIN_NAME=$(echo "$RELEASE_JSON" | jq -r --arg TARGET "$TARGET" '.assets[] | select(.name | test($TARGET)) | .name')

echo "Downloading $BIN_NAME ..."
curl -L -o "$BIN_NAME" "$BIN_URL"

chmod +x "$BIN_NAME"

echo "Download complete: $BIN_NAME"
