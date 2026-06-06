#!/bin/bash
#
# Installation script for 'x' CLI tool
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/michaelmunson/scripting/main/install.sh | bash
#
# Or download and run:
#   wget https://raw.githubusercontent.com/michaelmunson/scripting/main/install.sh
#   bash install.sh
#
# Requirements:
#   - Rust/Cargo (will prompt if not installed)
#   - Git (for cloning the repository)
#   - Internet connection

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

REPO_URL="https://github.com/michaelmunson/x.sh.git"
BINARY_NAME="x"
INSTALL_DIR="${HOME}/.local/bin"
TEMP_DIR=$(mktemp -d)

IN_REPO=false
if [ -d ".git" ] && [ -f "Cargo.toml" ]; then
    IN_REPO=true
fi

cleanup() {
    if [ -d "$TEMP_DIR" ]; then
        rm -rf "$TEMP_DIR"
    fi
}
trap cleanup EXIT

error() {
    echo -e "${RED}Error:${NC} $1" >&2
    exit 1
}

info() {
    echo -e "${GREEN}Info:${NC} $1"
}

warn() {
    echo -e "${YELLOW}Warning:${NC} $1"
}

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

check_rust() {
    if ! command_exists cargo; then
        error "Rust/Cargo is not installed. Please install Rust first:\n  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    fi
    
    info "Found Rust $(rustc --version 2>/dev/null || echo 'unknown')"
}

setup_install_dir() {
    if [ ! -d "$INSTALL_DIR" ]; then
        mkdir -p "$INSTALL_DIR"
        info "Created directory: $INSTALL_DIR"
    fi
    
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        warn "$INSTALL_DIR is not in your PATH"
        echo "Add this to your ~/.bashrc, ~/.zshrc, or ~/.profile:"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    fi
}

build_binary() {
    if [ "$IN_REPO" = true ]; then
        info "Detected we're in the repository, using current directory..."
        BUILD_DIR="$(pwd)"
    else
        info "Cloning repository from $REPO_URL..."
        if ! command_exists git; then
            error "Git is not installed. Please install git first."
        fi
        
        if ! git clone "$REPO_URL" "$TEMP_DIR/repo" 2>/dev/null; then
            error "Failed to clone repository. Please check:\n  1. The REPO_URL is correct in the script\n  2. You have internet connectivity\n  3. Git is properly installed"
        fi
        BUILD_DIR="$TEMP_DIR/repo"
    fi
    
    cd "$BUILD_DIR" || error "Failed to enter project directory (./)"
    
    info "Building $BINARY_NAME (this may take a few minutes on first run)..."
    if ! cargo build --release; then
        error "Build failed. Please check the error messages above."
    fi
    
    BINARY_PATH="$BUILD_DIR/target/release/$BINARY_NAME"
    
    if [ ! -f "$BINARY_PATH" ]; then
        error "Binary not found at expected location: $BINARY_PATH"
    fi
    
    info "Build successful!"
}

install_binary() {
    local target_path="$INSTALL_DIR/$BINARY_NAME"
    
    if [ -f "$target_path" ]; then
        warn "$BINARY_NAME is already installed at $target_path"
        if [ -t 0 ]; then
            read -p "Overwrite? (y/N): " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                info "Installation cancelled."
                exit 0
            fi
        else
            info "Overwriting existing installation..."
        fi
    fi
    
    info "Installing $BINARY_NAME to $INSTALL_DIR..."
    
    if cp "$BINARY_PATH" "$target_path"; then
        chmod +x "$target_path"
        info "Installation complete!"
    else
        error "Failed to copy binary to $INSTALL_DIR"
    fi
}

verify_installation() {
    if command_exists "$BINARY_NAME"; then
        info "Verification successful!"
        echo ""
        echo "You can now use '$BINARY_NAME' from anywhere in your terminal."
        echo "Try running: $BINARY_NAME --help"
    else
        warn "Installation completed, but '$BINARY_NAME' is not in PATH."
        warn "Make sure $INSTALL_DIR is in your PATH and restart your terminal."
    fi
}

main() {
    echo "Installing $BINARY_NAME..."
    echo ""
    
    check_rust
    setup_install_dir
    build_binary
    install_binary
    verify_installation
}

main

