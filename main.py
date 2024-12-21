#! /usr/bin/env python3

import subprocess
import logging
import asyncio
import ssl
import json
import re

from config import (
    CPU2_INSTALLED,
    CPU1_FAN,
    CPU2_FAN,
    TEMP_THRESHOLD,
    USER,
    PASSWORD,
    ILOIP
)

#------------------------------------------------
# ilo4-fan-control
# Fan control for HP ProLiant Gen8/Gen9 servers patched HPT iLo4 firmware
# Before use this script, please install python3, sshpass and ilo4 unlock
# ilo4 unlock repository: https://github.com/kendallgoto/ilo4_unlock

# Repository: https://github.com/walkmana-25/ilo4-fan-control
# Copyright (c) 2024 walkmana-25
# License Apache-2.0 License. For details, see LICENSE file


# No verify SSL certificate
#ssl._create_default_https_context = ssl._create_unverified_context
ssl.create_default_context = ssl._create_unverified_context
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
    datefmt="%Y-%m-%d %H:%M:%S"
)

ssh_option = [
    "-o", "StrictHostKeyChecking=no",
    "-o", "KexAlgorithms=+diffie-hellman-group-exchange-sha1,diffie-hellman-group14-sha1,diffie-hellman-group1-sha1",
    "-o", "HostKeyAlgorithms=+ssh-rsa",
    "-o", "Ciphers=+aes128-cbc,3des-cbc,aes192-cbc,aes256-cbc"
]


def set_fan(fan_id: int, temp: int) -> list:
    fan_speed = 0
    for threshold in TEMP_THRESHOLD:
        if threshold["min"] <= temp < threshold["max"]:
            fan_speed = threshold["fan"]
            break

    logging.info(f"Fan ID: {fan_id} speed: {fan_speed}")

    fan_cmd = ["fan", "p", str(fan_id), "max", str(fan_speed)]


    logging.info(f"Set fan id: {fan_id} speed: {fan_speed}")
    
    return  fan_cmd

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

async def run_cmd_ssh(cmd: list[list[str]]) -> None:
    ssh_cmd = ["sshpass", "-p", PASSWORD, "ssh", *ssh_option, f"{USER}@{ILOIP}"]

    cmd = cmd.copy()
    cmd += [["exit"]]

    process = await asyncio.create_subprocess_exec(
        *ssh_cmd,
        stdin=asyncio.subprocess.PIPE,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE
    )

    for c in cmd:
        log = ""
        while True:
            o = await process.stdout.read(2)
            log += o.decode("utf-8")
            if not o:
                break
            if "</>hpiLO->" in log:
                break

        fan_command = " ".join(c)
        process.stdin.write(f"{fan_command}\n".encode("utf-8"))
        await process.stdin.drain()

    await process.wait()
    logging.info("Fan control done")

async def fan_control() -> None:
    cpu_temp = await get_ilo_temp()
    logging.info(f"CPU Temp: {cpu_temp}")

    cmds = []
    
    for cpu_id, temp in cpu_temp.items():
        if temp > 0:
            if CPU2_INSTALLED and cpu_id == "CPU-2":
                for fan_id in CPU2_FAN:
                    cmds.append(set_fan(fan_id, temp))
            else:
                for fan_id in CPU1_FAN:
                    cmds.append(set_fan(fan_id, temp))
    
    await run_cmd_ssh(cmds)
    

async def main() -> None:
    logging.info("Fan control start")
    logging.info("by ilo4-fan-control")
    logging.info("Target iLO4: %s", ILOIP)

    while True:
        try:
            await fan_control()
        except Exception as e:
            logging.error(f"fan control failed: {e}")

        await asyncio.sleep(30)

asyncio.run(main())