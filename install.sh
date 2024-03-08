# Copying config files
if [ ! -d $HOME/local_dev ]; then
    mkdir $HOME/local_dev
fi

cp -r ./config $HOME/local_dev

# Adding to path
if [ "x$(grep '^export LOCAL_DEV_PATH' ~/.zshrc)" != "x" ]; then
    echo "The LOCAL_DEV_PATH environment variable is in you .bashrc / .zshrc. No changes made."
else
    echo "Adding LOCAL_DEV_PATH=~/local_dev/ to .bashrc / .zshrc"

    echo "\nexport LOCAL_DEV_PATH=$HOME/local_dev/" >> ~/.zshrc
fi

# Copying local dev binary to /usr/local/bin
sudo cp ./target/release/local_dev /usr/local/bin/bin-local-dev