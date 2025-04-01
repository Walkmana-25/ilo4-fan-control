# ilo4-fan-control

ilo4 easy fan control software

## Overview

`ilo4-fan-control` is a software tool designed to easily control the fan speed of HPE servers using the iLO4 interface. This tool helps optimize cooling efficiency and reduce noise levels.

## Features

- Manual fan speed adjustment
- Automatic fan control based on temperature sensors
- Logging functionality for monitoring fan speed and temperature
- Easy installation and configuration
- Multi-platform support (Linux, Windows, macOS)
- ARM64 and AMD64 architecture support

## Requirements

Before using ilo4-fan-control, please ensure you meet the following requirements:

- **iLO4 with the unlock patch applied**: This tool requires your HPE server's iLO4 interface to have the [ilo4_unlock](https://github.com/kendallgoto/ilo4_unlock) patch applied. Without this patch, fan control commands will be rejected by the iLO interface.
- **SSH access to iLO**: Make sure SSH access is enabled on your iLO4 interface.
- **Valid iLO credentials**: You need administrator credentials for your iLO interface.
- **Dependencies**:
  - `curl` or `wget`: Used for downloading and making HTTP requests

Please note that modifying fan behavior can potentially lead to inadequate cooling if configured improperly. Always monitor server temperatures when using custom fan profiles.

## Installation

### Option 1: Install using automated scripts

#### Linux/macOS

```sh
curl -L https://raw.githubusercontent.com/walkmana-25/ilo4-fan-control/main/install.sh | bash
```

or if you have downloaded or cloned the repository:

```sh
bash ./install.sh
```

#### Windows

Open PowerShell as Administrator and run:

```powershell
Invoke-Expression (New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/walkmana-25/ilo4-fan-control/main/install.ps1')
```

or if you have downloaded or cloned the repository:

```powershell
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

### Option 2: Install using Cargo

If you prefer to build from source using Rust's package manager, Cargo, follow these steps:

1. Install Rust and Cargo:

   If you don't have Rust installed, you can install it using [rustup](https://rustup.rs/):

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   For Windows, download and run the rustup-init.exe from the [Rust website](https://www.rust-lang.org/tools/install).

2. Install Dependencies:

   Make sure the following are installed on your system:

   - Perl
   - Make (GNU Make)
   - [iLo4 unlock](https://github.com/kendallgoto/ilo4_unlock) (applied to your iLO interface)

   On Debian/Ubuntu:

   ```sh
   sudo apt install perl make build-essential
   ```

   On macOS:

   ```sh
   brew install perl make
   ```

   On Windows (using Chocolatey):

   ```powershell
   choco install perl make
   ```

3. Clone the repository and build:

   ```sh
   git clone https://github.com/walkmana-25/ilo4-fan-control.git
   cd ilo4-fan-control
   cargo build --release
   ```

4. Install the binary:

   ```sh
   # Install to ~/.cargo/bin (will be in your PATH if Rust is properly installed)
   cargo install --path .
   
   # Or manually copy the binary
   sudo cp target/release/fctrl /usr/local/bin/
   ```

5. Set up configuration:

   Create a directory for configuration:

   ```sh
   # System-wide (requires root)
   sudo mkdir -p /etc/ilo4-fan-control

   # Or user-specific
   mkdir -p ~/.config/ilo4-fan-control
   ```

   Copy the sample configuration file:

   ```sh
   # System-wide (requires root)
   sudo cp sample.toml /etc/ilo4-fan-control/config.toml

   # Or user-specific
   cp sample.toml ~/.config/ilo4-fan-control/config.toml
   ```

6. Edit the configuration file:

   ```sh
   # System-wide
   sudo nano /etc/ilo4-fan-control/config.toml

   # Or user-specific
   nano ~/.config/ilo4-fan-control/config.toml
   ```

   Edit the following fields in the configuration file:
   - `host`: Set to your iLO hostname or IP address
   - `user`: Set to your iLO username
   - `password`: Set to your iLO password
   - Adjust temperature and fan settings as needed

You can also download pre-built binaries from the [GitHub Releases](https://github.com/Walkmana-25/ilo4-fan-control/releases) page.

## Usage

### Fan Status

Show the current fan status of your server:

```sh
fctrl status --host <ilo-ip> --user <username> --password <password>
```

### Configuration

Generate a sample configuration file:

```sh
fctrl config -p config.toml -s
```

Validate your configuration file:

```sh
fctrl config -p config.toml -v
```

### Daemon Mode

Run in continuous monitoring and control mode:

```sh
fctrl daemon -p config.toml
```

## Configuration File

The configuration file uses TOML format and allows you to precisely control fan behavior based on temperature ranges. Below is a detailed explanation of each configuration option:

### Global Settings

```toml
# How frequently the daemon checks temperature and adjusts fan speed (in seconds)
run_period_seconds = 60
```

- `run_period_seconds`: Defines how often (in seconds) the daemon will check server temperatures and adjust fan speeds. Lower values provide more responsive control but increase system overhead. Recommended range is 30-120 seconds.

### Server Configuration

Each iLO server is defined as a target in the configuration file. You can configure multiple servers by adding multiple `[[targets]]` sections.

```toml
[[targets]]
# iLO hostname or IP address
host = "ILO_HOST_NAME_OR_IP_ADDRESS"
# iLO authentication username
user = "USERNAME"
# iLO authentication password
password = "PASSWORD"
```

- `host`: The hostname or IP address of your iLO interface (required)
- `user`: Username for iLO authentication (required)
- `password`: Password for iLO authentication (required)

### Fan Configuration

Fan settings are specified per server:

```toml
[targets.target_fans]
# Number of fans in the server to control
NumFans = 7
```

- `NumFans`: Specifies the number of fans in your server (typically 6-8 for HPE ProLiant servers)

### Temperature-based Fan Speed Settings

The core feature of ilo4-fan-control is defining temperature ranges and corresponding maximum fan speeds. Each temperature range is configured as a separate section:

```toml
# Temperature range configuration
[[targets.temprature_fan_config]]
# Minimum temperature threshold in Celsius
min_temp = 0
# Maximum temperature threshold in Celsius
max_temp = 55
# Maximum fan speed (percentage) within this temperature range
max_fan_speed = 20

# Additional temperature ranges with different fan speeds
[[targets.temprature_fan_config]]
min_temp = 55
max_temp = 60
max_fan_speed = 40

[[targets.temprature_fan_config]]
min_temp = 61
max_temp = 70
max_fan_speed = 70

[[targets.temprature_fan_config]]
min_temp = 71
max_temp = 100
max_fan_speed = 100
```

For each temperature range

- `min_temp`: The lower temperature bound in Celsius (inclusive)
- `max_temp`: The upper temperature bound in Celsius (inclusive)
- `max_fan_speed`: Maximum fan speed percentage (1-100) to use when temperature is within this range

The program uses these configurations to create a fan control curve. When the CPU temperature falls within a specific range, the fans will operate at or below the specified maximum speed for that range.

### Multi-Server Configuration Example

For environments with multiple servers, you can define multiple targets in the same configuration file:

```toml
# Global settings for all servers
run_period_seconds = 60

# First server configuration
[[targets]]
host = "192.168.1.100"
user = "admin"
password = "password123"

[targets.target_fans]
NumFans = 7

[[targets.temprature_fan_config]]
min_temp = 0
max_temp = 55
max_fan_speed = 20
# ... additional temperature ranges ...

# Second server configuration
[[targets]]
host = "192.168.1.101"
user = "admin"
password = "password456"

[targets.target_fans]
NumFans = 6  # Different server might have different number of fans

[[targets.temprature_fan_config]]
min_temp = 0
max_temp = 50  # Different temperature thresholds
max_fan_speed = 15  # Lower fan speeds for quieter operation
# ... additional temperature ranges ...
```

### Configuration Guidelines

1. **Temperature Ranges**: Ensure that your temperature ranges don't overlap and cover the entire expected temperature spectrum of your server.

2. **Fan Speeds**: Start with conservative values (higher fan speeds) and gradually lower them while monitoring temperatures to find the optimal balance between noise and cooling.

3. **Critical Temperatures**: Always include a high-temperature range (e.g., 80-100°C) with maximum fan speed (100%) as a safety measure.

4. **Testing**: After configuring, monitor server temperatures closely during the first few days, especially under varying workloads, to ensure adequate cooling.

You can validate your configuration file using:

```sh
fctrl config -p /path/to/config.toml -v
```

## Contributing

Bug reports and feature requests are welcome on the [Issues](https://github.com/walkmana-25/ilo4-fan-control/issues) page. Pull requests are also appreciated.

## License

ilo4-fan-control is licensed under the Apache License, Version 2.0 (the "License").
