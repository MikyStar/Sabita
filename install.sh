#!/usr/bin/env bash
set -e

# --- CONFIGURATION: update these variables ---
REPO_OWNER="MikyStar"  # e.g., "myuser"
REPO_NAME="Sabita"                # e.g., "myproject"
BINARY_NAME="sabita"                    # base name of your binary (without the target triple)
# --------------------------------------------

# --- Determine platform target ---
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

# --- Get latest release info from GitHub ---
API_URL="https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest"
RELEASE_JSON=$(curl -s "$API_URL")

# --- Extract the asset URL matching the target using jq ---
# This looks for an asset whose name contains the target triple.
ASSET_URL=$(echo "$RELEASE_JSON" | jq -r --arg TARGET "$TARGET" '.assets[] | select(.name | test($TARGET)) | .browser_download_url')

if [ -z "$ASSET_URL" ]; then
    echo "No asset found for target: $TARGET"
    exit 1
fi

echo "Found asset URL: $ASSET_URL"

# --- Download the asset ---
OUTPUT_FILE="${BINARY_NAME}-${TARGET}"
echo "Downloading $OUTPUT_FILE ..."
curl -L -o "$OUTPUT_FILE" "$ASSET_URL"

# If the binary needs to be executable, update its permissions:
chmod +x "$OUTPUT_FILE"

echo "Download complete: $OUTPUT_FILE"
