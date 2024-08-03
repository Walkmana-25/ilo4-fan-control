#! /usr/bin/env python3

import subprocess
import logging
import asyncio
import ssl
import json
import re

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

# No verify SSL certificate
ssl._create_default_https_context = ssl._create_unverified_context
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
    datefmt="%Y-%m-%d %H:%M:%S"
)

async def set_fan(fan_id: int, temp: int) -> None:
    fan_speed = 0
    for threshold in TEMP_THRESHOLD:
        if threshold["min"] <= temp < threshold["max"]:
            fan_speed = threshold["fan"]
            break

    logging.info(f"Fan ID: {fan_id} speed: {fan_speed}")

    ssh_option = [
        "-o", "StrictHostKeyChecking=no",
        "-o", "KexAlgorithms=+diffie-hellman-group-exchange-sha1,diffie-hellman-group14-sha1,diffie-hellman-group1-sha1",
        "-o", "HostKeyAlgorithms=+ssh-rsa",
        "-o", "Ciphers=+aes128-cbc,3des-cbc,aes192-cbc,aes256-cbc"
    ]
    ssh_cmd = ["sshpass", "-p", PASSWORD, "ssh", *ssh_option, f"{USER}@{ILOIP}"]
    fan_cmd = ["fan", "p", str(fan_id), "max", str(fan_speed)]

    cmd = ssh_cmd + fan_cmd

    logging.info(f"Set fan id: {fan_id} speed: {fan_speed}")

    await asyncio.create_subprocess_exec(*cmd, stdout=asyncio.subprocess.DEVNULL)

async def get_ilo_temp() -> dict[str, int]:
    url = f"https://{ILOIP}/redfish/v1/Chassis/1/Thermal"
    auth = f"{USER}:{PASSWORD}"

    # get CPU temperature from ilo restful api
    output = subprocess.run(
        [
            "curl", "--insecure", "-u", auth, "--location", url
        ],
        capture_output=True
    )

    # Parse json output
    output_json = json.loads(output.stdout.decode("utf-8"))

    cpu_temp: dict[str, int] = {}

    for temp_info in output_json["Temperatures"]:
        if temp_info["PhysicalContext"] == "CPU":

            cpu_id = None

            if re.fullmatch(r"\d{2}-CPU", temp_info["Name"]):
                cpu_id = 1

            else:
                cpu_match = re.search(r"\d{2}-CPU (\d)", temp_info["Name"])
                if cpu_match:
                    cpu_id = int(cpu_match.group(1))
                else:
                    logging.error("Failed to get CPU temperature")

            cpu_temp[f"CPU-{cpu_id}"] = int(temp_info["ReadingCelsius"])

    return cpu_temp


async def fan_control() -> None:
    cpu_temp = await get_ilo_temp()
    logging.info(f"CPU Temp: {cpu_temp}")

    task: list[asyncio.Task] = []
    
    for cpu_id, temp in cpu_temp.items():
        if temp > 0:
            if CPU2_INSTALLED and cpu_id == "CPU-2":
                for fan_id in CPU2_FAN:
                    task.append(asyncio.create_task(set_fan(fan_id, temp)))
            else:
                for fan_id in CPU1_FAN:
                    task.append(asyncio.create_task(set_fan(fan_id, temp)))
    
    await asyncio.gather(*task)

async def main() -> None:
    logging.info("Fan control start")
    logging.info("by ilo4-fan-control")

    while True:
        try:
            await fan_control()
        except Exception as e:
            logging.error(f"fan control failed: {e}")

        await asyncio.sleep(30)

asyncio.run(main())