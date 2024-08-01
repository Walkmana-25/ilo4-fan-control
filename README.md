
# ilo4-fan-control

ilo4 easy fan control software

## Overview

`ilo4-fan-control` is a software tool designed to easily control the fan speed of HPE servers using the iLO4 interface. This tool helps optimize cooling efficiency and reduce noise levels.

## Features

- Manual fan speed adjustment
- Automatic fan control based on temperature sensors
- Logging functionality for monitoring fan speed and temperature
- Easy installation and configuration

## Installation

1. Install Dependencies:

    Please install the following dependencies:

    - [iLo4 unlock](https://github.com/kendallgoto/ilo4_unlock)
    - sshpass
    - curl

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/ilo4-fan-control.git
    cd ilo4-fan-control
    ```

1. Change Setting:

    Please change the setting in main.py.
    For details, please read comments in main.py

    ```sh
    vim main.py
    ```

1. Run the following command to install the software:

    ```sh
    sudo bash install.sh
    ```


## Contributing

Bug reports and feature requests are welcome on the [Issues](https://github.com/yourusername/ilo4-fan-control/issues) page. Pull requests are also appreciated.

## License

This project is licensed under the Apache 2.0 License. See the `LICENSE` file for more details.

