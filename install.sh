#!/bin/bash
set -e

# ilo4-fan-control installation script
# This script downloads and installs the ilo4-fan-control binary

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Repository information
REPO="Walkmana-25/ilo4-fan-control"

echo -e "${BLUE}HPE iLO4 Fan Control Utility Installer${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Determine system architecture and OS
get_system_info() {
    echo -e "${BLUE}Detecting system...${NC}"
    
    # Detect OS
    OS="$(uname -s)"
    case "${OS}" in
        Linux*)     OS="linux"; IS_LINUX=true;;
        Darwin*)    OS="apple-darwin"; IS_LINUX=false;;
        *)          echo -e "${RED}Unsupported operating system: ${OS}${NC}"; exit 1;;
    esac
    
    # Detect architecture
    ARCH="$(uname -m)"
    case "${ARCH}" in
        x86_64*)    ARCH="x86_64";;
        amd64*)     ARCH="x86_64";;
        arm64*)     ARCH="aarch64";;
        aarch64*)   ARCH="aarch64";;
        *)          echo -e "${RED}Unsupported architecture: ${ARCH}${NC}"; exit 1;;
    esac
    
    PLATFORM="${ARCH}-unknown-${OS}"
    if [ "$OS" = "apple-darwin" ]; then
        PLATFORM="${ARCH}-${OS}"
    fi
    
    echo -e "${GREEN}Detected: ${PLATFORM}${NC}"
}

# Get the latest release version from GitHub
get_latest_version() {
    echo -e "${BLUE}Fetching latest release information...${NC}"
    
    if command -v curl &> /dev/null; then
        VERSION=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | 
                  grep -o '"tag_name": "[^"]*' | 
                  cut -d'"' -f4)
    elif command -v wget &> /dev/null; then
        VERSION=$(wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" | 
                  grep -o '"tag_name": "[^"]*' | 
                  cut -d'"' -f4)
    else
        echo -e "${RED}Error: Neither curl nor wget found. Please install one of them and try again.${NC}"
        exit 1
    fi
    
    if [ -z "$VERSION" ]; then
        echo -e "${RED}Error: Could not determine the latest version from GitHub API.${NC}"
        echo -e "${RED}Please check your internet connection and try again.${NC}"
        echo -e "${RED}If the problem persists, the GitHub repository may not have any releases,${NC}"
        echo -e "${RED}or there might be GitHub API rate limiting in effect.${NC}"
        exit 2
    else
        echo -e "${GREEN}Latest version: ${VERSION}${NC}"
    fi
}

# Download the binary
download_binary() {
    echo -e "${BLUE}Downloading ilo4-fan-control...${NC}"
    
    BINARY_URL="https://github.com/${REPO}/releases/download/${VERSION}/fctrl-${VERSION}-${PLATFORM}"
    DEST_PATH="/tmp/fctrl"
    
    echo -e "${BLUE}Downloading from: ${BINARY_URL}${NC}"
    
    # Download the binary
    if command -v curl &> /dev/null; then
        curl -L -o "${DEST_PATH}" "${BINARY_URL}"
    elif command -v wget &> /dev/null; then
        wget -O "${DEST_PATH}" "${BINARY_URL}"
    else
        echo -e "${RED}Error: Neither curl nor wget found. Please install one of them and try again.${NC}"
        exit 1
    fi
    
    # Make the binary executable
    chmod +x "${DEST_PATH}"
}

# Install the binary to the system
install_binary() {
    echo -e "${BLUE}Installing ilo4-fan-control...${NC}"
    
    # Determine install location
    INSTALL_DIR="/usr/local/bin"
    if [ ! -d "$INSTALL_DIR" ] || [ ! -w "$INSTALL_DIR" ]; then
        # Try alternative location if /usr/local/bin isn't writable
        INSTALL_DIR="$HOME/.local/bin"
        mkdir -p "$INSTALL_DIR"
    fi
    
    # Move binary to install location
    mv "/tmp/fctrl" "$INSTALL_DIR/fctrl"
    
    echo -e "${GREEN}Binary installed to ${INSTALL_DIR}/fctrl${NC}"
    
    # Add installation directory to PATH if not already in it (for ~/.local/bin)
    if [ "$INSTALL_DIR" = "$HOME/.local/bin" ]; then
        if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
            echo -e "${YELLOW}Adding $INSTALL_DIR to your PATH${NC}"
            if [ -f "$HOME/.bashrc" ]; then
                echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.bashrc"
                echo -e "${YELLOW}Please run 'source ~/.bashrc' or start a new terminal session to update your PATH${NC}"
            elif [ -f "$HOME/.zshrc" ]; then
                echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.zshrc"
                echo -e "${YELLOW}Please run 'source ~/.zshrc' or start a new terminal session to update your PATH${NC}"
            else
                echo -e "${YELLOW}Please add $INSTALL_DIR to your PATH manually${NC}"
            fi
        fi
    fi
}

# Set up configuration
setup_config() {
    echo -e "${BLUE}Setting up configuration...${NC}"
    
    # Create config directory if it doesn't exist
    CONFIG_DIR="/etc/ilo4-fan-control"
    USER_CONFIG_DIR="$HOME/.config/ilo4-fan-control"
    
    if [ -w "/etc" ]; then
        mkdir -p "$CONFIG_DIR"
        CONFIG_PATH="$CONFIG_DIR/config.toml"
    else
        mkdir -p "$USER_CONFIG_DIR"
        CONFIG_PATH="$USER_CONFIG_DIR/config.toml"
    fi
    
    # Download sample configuration from repository
    SAMPLE_CONFIG_URL="https://raw.githubusercontent.com/${REPO}/main/sample.toml"
    echo -e "${BLUE}Downloading sample configuration from ${SAMPLE_CONFIG_URL}...${NC}"
    
    if [ ! -f "$CONFIG_PATH" ]; then
        if command -v curl &> /dev/null; then
            curl -L -o "${CONFIG_PATH}" "${SAMPLE_CONFIG_URL}"
        elif command -v wget &> /dev/null; then
            wget -O "${CONFIG_PATH}" "${SAMPLE_CONFIG_URL}"
        else
            echo -e "${YELLOW}Unable to download sample configuration. Please manually create configuration file at: ${CONFIG_PATH}${NC}"
        fi
        
        if [ -f "$CONFIG_PATH" ]; then
            echo -e "${GREEN}Sample configuration downloaded to ${CONFIG_PATH}${NC}"
            echo -e "${YELLOW}Please edit this file to match your server settings before using the daemon mode${NC}"
            echo -e "${YELLOW}Edit the following fields in the configuration file:${NC}"
            echo -e "  ${YELLOW}- host: Set to your iLO hostname or IP address${NC}"
            echo -e "  ${YELLOW}- user: Set to your iLO username${NC}"
            echo -e "  ${YELLOW}- password: Set to your iLO password${NC}"
            echo -e "  ${YELLOW}- Adjust temperature and fan settings as needed${NC}"
        fi
    else
        echo -e "${GREEN}Configuration file already exists at: ${CONFIG_PATH}${NC}"
    fi
}

# Setup systemd service for auto-start (Linux only)
setup_systemd() {
    if [ "$IS_LINUX" = true ]; then
        echo -e "${BLUE}Setting up systemd service...${NC}"
        
        # Download systemd service file from repository or use local file if available
        SYSTEMD_SERVICE_PATH="/etc/systemd/system/ilo4-fan-control.service"
        
        # Check if we can write to the systemd directory
        if [ -w "/etc/systemd/system" ]; then
            # Download or copy the service file
            SERVICE_FILE_URL="https://raw.githubusercontent.com/${REPO}/main/ilo4-fan-control.service"
            
            if [ -f "./ilo4-fan-control.service" ]; then
                echo -e "${BLUE}Using local systemd service file${NC}"
                cp "./ilo4-fan-control.service" "$SYSTEMD_SERVICE_PATH"
            else
                echo -e "${BLUE}Downloading systemd service file from ${SERVICE_FILE_URL}...${NC}"
                if command -v curl &> /dev/null; then
                    curl -L -o "$SYSTEMD_SERVICE_PATH" "$SERVICE_FILE_URL"
                elif command -v wget &> /dev/null; then
                    wget -O "$SYSTEMD_SERVICE_PATH" "$SERVICE_FILE_URL"
                else
                    echo -e "${YELLOW}Unable to download systemd service file. Skipping systemd setup.${NC}"
                    return
                fi
            fi
            
            # Reload systemd to recognize the new service file
            systemctl daemon-reload
            
            echo -e "${GREEN}Systemd service installed at ${SYSTEMD_SERVICE_PATH}${NC}"
            echo -e "${YELLOW}To enable and start the service, run:${NC}"
            echo -e "  ${YELLOW}sudo systemctl enable --now ilo4-fan-control.service${NC}"
        else
            echo -e "${YELLOW}No write permission to /etc/systemd/system. Skipping systemd setup.${NC}"
            echo -e "${YELLOW}To manually set up systemd service, run:${NC}"
            echo -e "  ${YELLOW}sudo cp ./ilo4-fan-control.service /etc/systemd/system/${NC}"
            echo -e "  ${YELLOW}sudo systemctl daemon-reload${NC}"
            echo -e "  ${YELLOW}sudo systemctl enable --now ilo4-fan-control.service${NC}"
        fi
    fi
}

# Print usage instructions
print_usage() {
    echo -e "\n${BLUE}Usage Instructions:${NC}"
    echo -e "  ${YELLOW}Show current status:${NC}"
    echo -e "    fctrl status --host <ilo-ip> --user <username> --password <password>\n"
    echo -e "  ${YELLOW}Generate sample configuration:${NC}"
    echo -e "    fctrl config -p config.toml -s\n"
    echo -e "  ${YELLOW}Validate configuration:${NC}"
    echo -e "    fctrl config -p config.toml -v\n"
    echo -e "  ${YELLOW}Run daemon mode:${NC}"
    echo -e "    fctrl daemon -p config.toml\n"
    
    # Add systemd instructions for Linux
    if [ "$IS_LINUX" = true ]; then
        echo -e "  ${YELLOW}Control systemd service (Linux):${NC}"
        echo -e "    sudo systemctl start ilo4-fan-control.service   # Start the service"
        echo -e "    sudo systemctl stop ilo4-fan-control.service    # Stop the service"
        echo -e "    sudo systemctl restart ilo4-fan-control.service # Restart the service"
        echo -e "    sudo systemctl status ilo4-fan-control.service  # Check service status\n"
    fi
    
    echo -e "For more information, run: fctrl --help"
}

# Main installation flow
main() {
    get_system_info
    get_latest_version
    download_binary
    install_binary
    setup_config
    setup_systemd
    print_usage
    
    echo -e "\n${GREEN}Installation completed successfully!${NC}"
}

# Start installation
main