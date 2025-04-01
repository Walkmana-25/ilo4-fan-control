# ilo4-fan-control PowerShell installation script
# This script downloads and installs the ilo4-fan-control binary on Windows

# Repository information
$REPO = "Walkmana-25/ilo4-fan-control"

Write-Host "HPE iLO4 Fan Control Utility Installer" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

# Get the latest release version from GitHub
function Get-LatestVersion {
    Write-Host "Fetching latest release information..." -ForegroundColor Cyan
    
    try {
        $releaseInfo = Invoke-RestMethod -Uri "https://api.github.com/repos/$REPO/releases/latest" -UseBasicParsing
        $version = $releaseInfo.tag_name
        Write-Host "Latest version: $version" -ForegroundColor Green
        return $version
    } catch {
        Write-Host "Error fetching latest release information: $_" -ForegroundColor Yellow
        Write-Host "Using fallback version v2.0.0-alpha-8" -ForegroundColor Yellow
        return "v2.0.0-alpha-8"
    }
}

# Determine system architecture
function Get-SystemInfo {
    Write-Host "Detecting system..." -ForegroundColor Cyan
    
    # Detect architecture
    $ARCH = if ([System.Environment]::Is64BitOperatingSystem) {
        if ([System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture -eq [System.Runtime.InteropServices.Architecture]::Arm64) {
            "aarch64"
        } else {
            "x86_64"
        }
    } else {
        Write-Host "Error: 32-bit systems are not supported" -ForegroundColor Red
        exit 1
    }
    
    $PLATFORM = "$ARCH-pc-windows-msvc"
    
    Write-Host "Detected: $PLATFORM" -ForegroundColor Green
    return $PLATFORM
}

# Download the binary
function Download-Binary {
    param (
        [string]$Platform,
        [string]$Version
    )
    
    Write-Host "Downloading ilo4-fan-control..." -ForegroundColor Cyan
    
    $BINARY_URL = "https://github.com/$REPO/releases/download/$Version/fctrl-$Version-$Platform.exe"
    $DEST_PATH = "$env:TEMP\fctrl.exe"
    
    Write-Host "Downloading from: $BINARY_URL" -ForegroundColor Cyan
    
    try {
        Invoke-WebRequest -Uri $BINARY_URL -OutFile $DEST_PATH -UseBasicParsing
        Write-Host "Download completed successfully" -ForegroundColor Green
    } catch {
        Write-Host "Error downloading file: $_" -ForegroundColor Red
        exit 1
    }
    
    return $DEST_PATH
}

# Install the binary
function Install-Binary {
    param (
        [string]$SourcePath
    )
    
    Write-Host "Installing ilo4-fan-control..." -ForegroundColor Cyan
    
    # Determine install location
    $INSTALL_DIR = "$env:LOCALAPPDATA\ilo4-fan-control"
    
    # Create directory if it doesn't exist
    if (-not (Test-Path $INSTALL_DIR)) {
        New-Item -ItemType Directory -Path $INSTALL_DIR -Force | Out-Null
    }
    
    # Copy binary to install location
    Copy-Item -Path $SourcePath -Destination "$INSTALL_DIR\fctrl.exe" -Force
    
    # Add to PATH if not already in it
    $USER_PATH = [Environment]::GetEnvironmentVariable("Path", "User")
    
    if (-not $USER_PATH.Contains($INSTALL_DIR)) {
        Write-Host "Adding installation directory to PATH..." -ForegroundColor Yellow
        [Environment]::SetEnvironmentVariable("Path", "$USER_PATH;$INSTALL_DIR", "User")
        $env:Path = "$env:Path;$INSTALL_DIR"
    }
    
    Write-Host "Binary installed to $INSTALL_DIR\fctrl.exe" -ForegroundColor Green
    return $INSTALL_DIR
}

# Set up configuration
function Setup-Config {
    param (
        [string]$InstallDir
    )
    
    Write-Host "Setting up configuration..." -ForegroundColor Cyan
    
    # Create config directory
    $CONFIG_DIR = "$env:LOCALAPPDATA\ilo4-fan-control\config"
    
    if (-not (Test-Path $CONFIG_DIR)) {
        New-Item -ItemType Directory -Path $CONFIG_DIR -Force | Out-Null
    }
    
    $CONFIG_PATH = "$CONFIG_DIR\config.toml"
    
    # Download sample configuration from repository if it doesn't exist
    if (-not (Test-Path $CONFIG_PATH)) {
        $SAMPLE_CONFIG_URL = "https://raw.githubusercontent.com/$REPO/main/sample.toml"
        Write-Host "Downloading sample configuration from $SAMPLE_CONFIG_URL..." -ForegroundColor Cyan
        
        try {
            Invoke-WebRequest -Uri $SAMPLE_CONFIG_URL -OutFile $CONFIG_PATH -UseBasicParsing
            Write-Host "Sample configuration downloaded to $CONFIG_PATH" -ForegroundColor Green
            Write-Host "Please edit this file to match your server settings before using the daemon mode" -ForegroundColor Yellow
            Write-Host "Edit the following fields in the configuration file:" -ForegroundColor Yellow
            Write-Host "  - host: Set to your iLO hostname or IP address" -ForegroundColor Yellow
            Write-Host "  - user: Set to your iLO username" -ForegroundColor Yellow
            Write-Host "  - password: Set to your iLO password" -ForegroundColor Yellow
            Write-Host "  - Adjust temperature and fan settings as needed" -ForegroundColor Yellow
        } catch {
            Write-Host "Unable to download sample configuration: $_" -ForegroundColor Yellow
            Write-Host "Please manually create configuration file at: $CONFIG_PATH" -ForegroundColor Yellow
        }
    } else {
        Write-Host "Configuration file already exists at: $CONFIG_PATH" -ForegroundColor Green
    }
    
    return $CONFIG_PATH
}

# Print usage instructions
function Print-Usage {
    param (
        [string]$ConfigPath
    )
    
    Write-Host "`nUsage Instructions:" -ForegroundColor Cyan
    Write-Host "  Show current status:" -ForegroundColor Yellow
    Write-Host "    fctrl status --host <ilo-ip> --user <username> --password <password>`n"
    Write-Host "  Generate sample configuration:" -ForegroundColor Yellow
    Write-Host "    fctrl config -p config.toml -s`n"
    Write-Host "  Validate configuration:" -ForegroundColor Yellow
    Write-Host "    fctrl config -p config.toml -v`n"
    Write-Host "  Run daemon mode:" -ForegroundColor Yellow
    Write-Host "    fctrl daemon -p `"$ConfigPath`"`n"
    Write-Host "For more information, run: fctrl --help"
}

# Create a Windows shortcut
function Create-Shortcut {
    param (
        [string]$InstallDir,
        [string]$ConfigPath
    )
    
    Write-Host "Creating start menu shortcut..." -ForegroundColor Cyan
    
    $WshShell = New-Object -ComObject WScript.Shell
    $StartMenuPath = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\ilo4-fan-control"
    
    if (-not (Test-Path $StartMenuPath)) {
        New-Item -ItemType Directory -Path $StartMenuPath -Force | Out-Null
    }
    
    $Shortcut = $WshShell.CreateShortcut("$StartMenuPath\iLO4 Fan Control.lnk")
    $Shortcut.TargetPath = "cmd.exe"
    $Shortcut.Arguments = "/c `"$InstallDir\fctrl.exe daemon -p `"$ConfigPath`" & pause`""
    $Shortcut.WorkingDirectory = "$InstallDir"
    $Shortcut.Description = "HPE iLO4 Fan Control Utility"
    $Shortcut.Save()
    
    Write-Host "Shortcut created in Start Menu" -ForegroundColor Green
}

# Main installation flow
function Main {
    $Version = Get-LatestVersion
    $Platform = Get-SystemInfo
    $BinaryPath = Download-Binary -Platform $Platform -Version $Version
    $InstallDir = Install-Binary -SourcePath $BinaryPath
    $ConfigPath = Setup-Config -InstallDir $InstallDir
    Create-Shortcut -InstallDir $InstallDir -ConfigPath $ConfigPath
    Print-Usage -ConfigPath $ConfigPath
    
    Write-Host "`nInstallation completed successfully!" -ForegroundColor Green
    Write-Host "You may need to restart your PowerShell session for PATH changes to take effect." -ForegroundColor Yellow
}

# Start installation
Main