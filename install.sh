#!/bin/bash

set -e

BINARY_NAME="statcat"
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="$HOME/.config/$BINARY_NAME"
RELEASE_URL="https://github.com/FluffoCJ/StatCat/releases/download/V1.1.0-beta/statcat-v1.1.0-beta.tar.gz"

echo "Installing $BINARY_NAME..."

# Download the tarball
curl -L $RELEASE_URL -o /tmp/$BINARY_NAME.tar.gz

# Extract the tarball
mkdir -p /tmp/$BINARY_NAME
tar -xzvf /tmp/$BINARY_NAME.tar.gz -C /tmp/$BINARY_NAME

# Move the binary to the installation directory
sudo mv /tmp/$BINARY_NAME/$BINARY_NAME $INSTALL_DIR
sudo chmod +x $INSTALL_DIR/$BINARY_NAME

# Set up the configuration directory
mkdir -p $CONFIG_DIR
cp -n /tmp/$BINARY_NAME/config.toml $CONFIG_DIR/config.toml

# Cleanup
rm -rf /tmp/$BINARY_NAME /tmp/$BINARY_NAME.tar.gz

echo "$BINARY_NAME installed successfully!"
echo "Configuration file copied to $CONFIG_DIR/config.toml."

