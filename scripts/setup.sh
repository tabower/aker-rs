#!/usr/bin/env bash
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

# Detect package manager
detect_pm() {
    if command -v pacman &>/dev/null; then
        echo "pacman"
    elif command -v apt &>/dev/null; then
        echo "apt"
    elif command -v dnf &>/dev/null; then
        echo "dnf"
    else
        error "Unsupported package manager"
    fi
}

# System packages
install_system_deps() {
    local pm
    pm=$(detect_pm)
    info "Package manager: $pm"

    case "$pm" in
        pacman)
            sudo pacman -S --needed --noconfirm \
                dtc \
                make
            ;;
        apt)
            sudo apt update
            sudo apt install -y \
                device-tree-compiler \
                gdb-multiarch \
                make
            ;;
        dnf)
            sudo dnf install -y \
                dtc \
                gdb-multiarch \
                make
            ;;
    esac

    info "System dependencies installed"
}

# Rust toolchain
install_rust_deps() {
    if ! command -v rustup &>/dev/null; then
        error "rustup not found. Install from https://rustup.rs"
    fi

    # Nightly toolchain
    info "Installing nightly toolchain..."
    rustup install nightly
    rustup default nightly

    # Target
    info "Adding RISC-V target..."
    rustup target add riscv64gc-unknown-none-elf

    # Components
    info "Adding rustup components..."
    rustup component add \
        llvm-tools \
        rust-src \
        clippy \
        rustfmt

    info "Rust toolchain configured"
}

# Cargo tools
install_cargo_tools() {
    local tools=(
        "cargo-binutils"
        "rustfilt"
    )

    for tool in "${tools[@]}"; do
        if cargo install --list | grep -q "^$tool "; then
            info "$tool already installed, skipping"
        else
            info "Installing $tool..."
            cargo install "$tool"
        fi
    done

    info "Cargo tools installed"
}

# Verify installation
verify() {
    info "Verifying installation..."

    local cmds=(
        "rustup:rustup"
        "cargo:cargo"
        "rust-objdump:rust-objdump"
        "rustfilt:rustfilt"
        "qemu-system-riscv64:qemu-system-riscv64"
        "dtc:dtc"
    )

    local ok=true
    for entry in "${cmds[@]}"; do
        local name="${entry%%:*}"
        local cmd="${entry##*:}"
        if command -v "$cmd" &>/dev/null; then
            info "  ✓ $name"
        else
            warn "  ✗ $name not found"
            ok=false
        fi
    done

    if $ok; then
        info "All dependencies ready!"
    else
        warn "Some dependencies missing, check above"
    fi
}

main() {
    echo ""
    echo "=============================="
    echo "  Environment Setup"
    echo "=============================="
    echo ""

    install_system_deps
    install_rust_deps
    install_cargo_tools
    verify

    echo ""
    info "Setup complete! Run 'make' to build."
}

main "$@"