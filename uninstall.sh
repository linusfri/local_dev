#!/bin/bash

# Sets path to usr local bin
USR_LOCAL_BIN=/usr/local/bin

# Check which shell and config file is active
SHELL_CONFIG_FILE=""
if [[ "$SHELL" =~ "/usr/bin/zsh" ]]; then
    SHELL_CONFIG_FILE=".zshrc"
elif [[ "$SHELL" =~ "/usr/bin/bash" ]]; then
    SHELL_CONFIG_FILE=".bashrc"
else
    echo "Unknown environment, exiting. Only debian based distros supported at the moment"
    exit 1
fi

# Removing local_dev directory and its contents
if [ -d "$HOME/local_dev" ]; then
    rm -rf "$HOME/local_dev"
    echo "Removed $HOME/local_dev"
else
    echo "$HOME/local_dev already removed"
fi

# Removing the binary from /usr/local/bin
if [ -f "$USR_LOCAL_BIN/local-dev" ]; then
    sudo rm -f $USR_LOCAL_BIN/local-dev
    echo "Removed $USR_LOCAL_BIN/local-dev"
else
    echo "$USR_LOCAL_BIN/local-dev already removed"
fi

# Removing the LOCAL_DEV_PATH environment variable from the shell config file
if [ -f "$HOME/$SHELL_CONFIG_FILE" ]; then
    sed -i '/^export LOCAL_DEV_PATH=/d' "$HOME/$SHELL_CONFIG_FILE"
    echo "Removed LOCAL_DEV_PATH environment variable from $SHELL_CONFIG_FILE"
else
    echo "Shell config file $SHELL_CONFIG_FILE not found."
fi