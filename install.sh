#!/bin/bash

# Function to download, extract the tar.gz file, and update PATH
install_pypes() {
    url=$1
    target_directory="$HOME/.local/bin" # Change this to your preferred directory

    echo "Downloading from $url"
    mkdir -p $target_directory
    curl -L $url | tar xz -C $target_directory

    # Add target directory to PATH if not already present
    if [[ ":$PATH:" != *":$target_directory:"* ]]; then
        echo "Adding $target_directory to your PATH"

        # Add path to .bashrc or .zshrc depending on the shell
        if [ -n "$BASH_VERSION" ]; then
            echo "export PATH=\"$target_directory:\$PATH\"" >> ~/.bashrc
            echo "Run 'source ~/.bashrc' to update your current session."
        elif [ -n "$ZSH_VERSION" ]; then
            echo "export PATH=\"$target_directory:\$PATH\"" >> ~/.zshrc
            echo "Run 'source ~/.zshrc' to update your current session."
        fi
    fi

    echo "Installation completed."
    echo "You can now run 'pypes' from anywhere after restarting your shell or sourcing your profile."
}

# Detect architecture and OS
arch=$(uname -m)
os=$(uname -s)

# Define the base URL for downloading the binaries
base_url="https://github.com/jaredzwick/agents/releases/download/v0.0.5"

case "$arch" in
    "aarch64")
        case "$os" in
            "Darwin") install_pypes "$base_url/pypes-aarch64-apple-darwin.tar.gz" ;;
            "Linux") install_pypes "$base_url/pypes-aarch64-unknown-linux-gnu.tar.gz" ;;
            *) echo "Unsupported OS for aarch64." ;;
        esac
        ;;
    "x86_64")
        case "$os" in
            "Darwin") install_pypes "$base_url/pypes-x86_64-apple-darwin.tar.gz" ;;
            "Linux") install_pypes "$base_url/pypes-x86_64-unknown-linux-gnu.tar.gz" ;;
            *) echo "Unsupported OS for x86_64." ;;
        esac
        ;;
    *)
        echo "Unsupported architecture."
        ;;
esac

