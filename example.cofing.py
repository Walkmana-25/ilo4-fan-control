#------------------------------------------------
# ilo4-fan-control
# Fan control for HP ProLiant Gen8/Gen9 servers patched HPT iLo4 firmware
# Before use this script, please install python3, sshpass and ilo4 unlock
# ilo4 unlock repository: https://github.com/kendallgoto/ilo4_unlock

# Repository: https://github.com/walkmana-25/ilo4-fan-control
# Copyright (c) 2024 walkmana-25
# License Apache-2.0 License. For details, see LICENSE file


# Configuration

# If your server has 2 CPU, set CPU2_INSTALLED to True
CPU2_INSTALLED = False

# Define fan ID for CPU1 and CPU2
CPU1_FAN = [0, 1, 2, 3]
CPU2_FAN = [4, 5, 6, 7]

# Fan speed threshold
# min: minimum temperature celsius
# max: maximum temperature celsius
# fan: fan speed (0-255)
TEMP_THRESHOLD = [
    {"min": 0, "max": 55, "fan": 50},
    {"min": 55, "max": 60, "fan": 70},
    {"min": 60, "max": 65, "fan": 100},
    {"min": 65, "max": 70, "fan": 150},
    {"min": 70, "max": 100, "fan": 255},
]

# iLO credential
# USER: iLO username
# PASSWORD: iLO password
# ILOIP: iLO IP address
# This user must be able to access ssh and ilo restful api
USER = "sshuser"
PASSWORD = "sshpass"
ILOIP = "iloip"

#------------------------------------------------

