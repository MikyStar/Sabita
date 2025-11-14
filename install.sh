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

BIN_URL=""
BIN_NAME=""

# Read through the JSON and find matching asset
while IFS= read -r line; do
    # Check if this line contains "name" field with our TARGET
    if echo "$line" | grep -q "\"name\"" && echo "$line" | grep -q "$TARGET"; then
        # Extract the name value
        BIN_NAME=$(echo "$line" | sed -n 's/.*"name": *"\([^"]*\)".*/\1/p')
        # Signal that we found a match
        FOUND_MATCH=1
    fi
    
    # If we found a match and now see browser_download_url, extract it
    if [ -n "$FOUND_MATCH" ] && echo "$line" | grep -q "\"browser_download_url\""; then
        BIN_URL=$(echo "$line" | sed -n 's/.*"browser_download_url": *"\([^"]*\)".*/\1/p')
        # Once we have both, we can break
        break
    fi
done <<< "$RELEASE_JSON"


if [ -z "$BIN_URL" ]; then
    echo "No asset found for target: $TARGET"
    exit 1
fi

echo "Found asset URL: $BIN_URL"

###################

# Install


echo "Downloading $BIN_NAME ..."
curl -L -o "$BIN_NAME" "$BIN_URL"

chmod +x "$BIN_NAME"

echo "Download complete: $BIN_NAME"
