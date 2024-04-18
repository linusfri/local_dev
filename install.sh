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

# Checking if local dev binary exists
if [ ! -f "./target/release/local_dev" ]; then
    echo "No binary in ./target/release/local_dev. Please build local dev first."
    exit 1
fi

# Adding to path
if [ "x$(grep '^export LOCAL_DEV_PATH' ~/$SHELL_CONFIG_FILE)" != "x" ]; then
    echo "The LOCAL_DEV_PATH environment variable is in your $SHELL_CONFIG_FILE. No changes made."
else
    echo "Adding LOCAL_DEV_PATH=~/local_dev/ to $SHELL_CONFIG_FILE"

    echo -e "export LOCAL_DEV_PATH=$HOME/local_dev/" >> ~/$SHELL_CONFIG_FILE
fi

# TODO: accept user input to export following variables:
#   - GIT_HOST_URL=<string>
#   - GIT_HOST_TOKEN=<string>
#   - USER_AGENT=<string>
# 
# These must be set in order to make github integration work.
# Future work will add support for other VCS providers to.

# Copying config files
if [ ! -d "$HOME/local_dev" ]; then
    mkdir $HOME/local_dev
fi

cp -r ./config $HOME/local_dev
echo "Copied config files to $HOME/local_dev"

# Copying binary to /usr/local/bin
sudo cp ./target/release/local_dev $USR_LOCAL_BIN/local-dev
echo "Copied ./target/release/local_dev to $USR_LOCAL_BIN/local-dev"