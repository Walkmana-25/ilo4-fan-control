

struct CpuTemp {
  cpuid: u8,
  current: u8
}

struct Fan {
  name: String,
  current: u8,
  status: String
}

struct TempData {
  cpu_temps: Vec<CpuTemp>,
  high_temp_critical_reached_component: bool,
  inlet: u8,
  num_fans: u8,


  


}



#[cfg(test)]
mod test {
    const ilo_json: String = r###"
{
  "@odata.context": "/redfish/v1/$metadata#Chassis/Members/1/Thermal$entity",
  "@odata.id": "/redfish/v1/Chassis/1/Thermal/",
  "@odata.type": "#Thermal.1.2.0.Thermal",
  "Fans": [
    {
      "CurrentReading": 11,
      "FanName": "Fan 1",
      "Oem": {
        "Hp": {
          "@odata.type": "#HpServerFan.1.0.0.HpServerFan",
          "Location": "System",
          "Type": "HpServerFan.1.0.0"
        }
      },
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Percent"
    },
    {
      "CurrentReading": 11,
      "FanName": "Fan 2",
      "Oem": {
        "Hp": {
          "@odata.type": "#HpServerFan.1.0.0.HpServerFan",
          "Location": "System",
          "Type": "HpServerFan.1.0.0"
        }
      },
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Percent"
    },
    {
      "CurrentReading": 11,
      "FanName": "Fan 3",
      "Oem": {
        "Hp": {
          "@odata.type": "#HpServerFan.1.0.0.HpServerFan",
          "Location": "System",
          "Type": "HpServerFan.1.0.0"
        }
      },
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Percent"
    },
    {
      "CurrentReading": 11,
      "FanName": "Fan 4",
      "Oem": {
        "Hp": {
          "@odata.type": "#HpServerFan.1.0.0.HpServerFan",
          "Location": "System",
          "Type": "HpServerFan.1.0.0"
        }
      },
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Percent"
    },
    {
      "CurrentReading": 11,
      "FanName": "Fan 5",
      "Oem": {
        "Hp": {
          "@odata.type": "#HpServerFan.1.0.0.HpServerFan",
          "Location": "System",
          "Type": "HpServerFan.1.0.0"
        }
      },
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Percent"
    },
    {
      "CurrentReading": 11,
      "FanName": "Fan 6",
      "Oem": {
        "Hp": {
          "@odata.type": "#HpServerFan.1.0.0.HpServerFan",
          "Location": "System",
          "Type": "HpServerFan.1.0.0"
        }
      },
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Percent"
    },
    {
      "CurrentReading": 11,
      "FanName": "Fan 7",
      "Oem": {
        "Hp": {
          "@odata.type": "#HpServerFan.1.0.0.HpServerFan",
          "Location": "System",
          "Type": "HpServerFan.1.0.0"
        }
      },
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Percent"
    }
  ],
  "Id": "Thermal",
  "Name": "Thermal",
  "Temperatures": [
    {
      "CurrentReading": 23,
      "Name": "01-Inlet Ambient",
      "Number": 1,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 15,
          "LocationYmm": 0,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "Intake",
      "ReadingCelsius": 23,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 42,
      "UpperThresholdFatal": 46,
      "UpperThresholdUser": 0
    },
    {
      "CurrentReading": 44,
      "Name": "02-CPU 1",
      "Number": 2,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 11,
          "LocationYmm": 5,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "CPU",
      "ReadingCelsius": 44,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 70,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 47,
      "Name": "03-CPU 2",
      "Number": 3,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 4,
          "LocationYmm": 5,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "CPU",
      "ReadingCelsius": 47,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 70,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 37,
      "Name": "04-P1 DIMM 1-6",
      "Number": 4,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 9,
          "LocationYmm": 5,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 37,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 89,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 38,
      "Name": "05-P1 DIMM 7-12",
      "Number": 5,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 14,
          "LocationYmm": 5,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 38,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 89,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 41,
      "Name": "06-P2 DIMM 1-6",
      "Number": 6,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 1,
          "LocationYmm": 5,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 41,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 89,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 40,
      "Name": "07-P2 DIMM 7-12",
      "Number": 7,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 6,
          "LocationYmm": 5,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 40,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 89,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 35,
      "Name": "08-HD Max",
      "Number": 8,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 10,
          "LocationYmm": 0,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 35,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 60,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 0,
      "Name": "09-Exp Bay Drive",
      "Number": 9,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 12,
          "LocationYmm": 0,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 0,
      "Status": {
        "State": "Absent"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 0,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 47,
      "Name": "10-Chipset",
      "Number": 10,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 13,
          "LocationYmm": 10,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 47,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 105,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 36,
      "Name": "11-PS 1 Inlet",
      "Number": 11,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 1,
          "LocationYmm": 10,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "PowerSupply",
      "ReadingCelsius": 36,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 0,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 40,
      "Name": "12-PS 2 Inlet",
      "Number": 12,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 4,
          "LocationYmm": 10,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "PowerSupply",
      "ReadingCelsius": 40,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 0,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 45,
      "Name": "13-VR P1",
      "Number": 13,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 10,
          "LocationYmm": 1,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 45,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 115,
      "UpperThresholdFatal": 120
    },
    {
      "CurrentReading": 51,
      "Name": "14-VR P2",
      "Number": 14,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 4,
          "LocationYmm": 1,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 51,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 115,
      "UpperThresholdFatal": 120
    },
    {
      "CurrentReading": 35,
      "Name": "15-VR P1 Mem",
      "Number": 15,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 9,
          "LocationYmm": 1,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 35,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 115,
      "UpperThresholdFatal": 120
    },
    {
      "CurrentReading": 34,
      "Name": "16-VR P1 Mem",
      "Number": 16,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 13,
          "LocationYmm": 1,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 34,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 115,
      "UpperThresholdFatal": 120
    },
    {
      "CurrentReading": 40,
      "Name": "17-VR P2 Mem",
      "Number": 17,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 2,
          "LocationYmm": 1,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 40,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 115,
      "UpperThresholdFatal": 120
    },
    {
      "CurrentReading": 38,
      "Name": "18-VR P2 Mem",
      "Number": 18,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 6,
          "LocationYmm": 1,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 38,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 115,
      "UpperThresholdFatal": 120
    },
    {
      "CurrentReading": 40,
      "Name": "19-PS 1 Internal",
      "Number": 19,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 1,
          "LocationYmm": 13,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "PowerSupply",
      "ReadingCelsius": 40,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 0,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 41,
      "Name": "20-PS 2 Internal",
      "Number": 20,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 4,
          "LocationYmm": 13,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "PowerSupply",
      "ReadingCelsius": 41,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 0,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 0,
      "Name": "21-PCI 1",
      "Number": 21,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 13,
          "LocationYmm": 13,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 0,
      "Status": {
        "State": "Absent"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 0,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 0,
      "Name": "22-PCI 2",
      "Number": 22,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 13,
          "LocationYmm": 13,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 0,
      "Status": {
        "State": "Absent"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 0,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 0,
      "Name": "23-PCI 3",
      "Number": 23,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 5,
          "LocationYmm": 12,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 0,
      "Status": {
        "State": "Absent"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 0,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 66,
      "Name": "24-HD Controller",
      "Number": 24,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 8,
          "LocationYmm": 8,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 66,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 100,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 0,
      "Name": "25-LOM Card",
      "Number": 25,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 14,
          "LocationYmm": 13,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 0,
      "Status": {
        "State": "Absent"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 0,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 45,
      "Name": "26-LOM",
      "Number": 26,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 7,
          "LocationYmm": 13,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 45,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 100,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 29,
      "Name": "27-Front Ambient",
      "Number": 27,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 9,
          "LocationYmm": 0,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "Intake",
      "ReadingCelsius": 29,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 65,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 44,
      "Name": "28-P/S 2 Zone",
      "Number": 28,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 3,
          "LocationYmm": 7,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 44,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 75,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 40,
      "Name": "29-Battery Zone",
      "Number": 29,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 7,
          "LocationYmm": 10,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 40,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 75,
      "UpperThresholdFatal": 80
    },
    {
      "CurrentReading": 44,
      "Name": "30-iLO Zone",
      "Number": 30,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 9,
          "LocationYmm": 14,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 44,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 90,
      "UpperThresholdFatal": 95
    },
    {
      "CurrentReading": 40,
      "Name": "31-PCI 1 Zone",
      "Number": 31,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 13,
          "LocationYmm": 13,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 40,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 70,
      "UpperThresholdFatal": 75
    },
    {
      "CurrentReading": 40,
      "Name": "32-PCI 2 Zone",
      "Number": 32,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 13,
          "LocationYmm": 13,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 40,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 70,
      "UpperThresholdFatal": 75
    },
    {
      "CurrentReading": 0,
      "Name": "33-PCI 3 Zone",
      "Number": 33,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 5,
          "LocationYmm": 12,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 0,
      "Status": {
        "State": "Absent"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 0,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 43,
      "Name": "34-HD Cntlr Zone",
      "Number": 34,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 11,
          "LocationYmm": 7,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 43,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 70,
      "UpperThresholdFatal": 75
    },
    {
      "CurrentReading": 36,
      "Name": "35-I/O Zone",
      "Number": 35,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 14,
          "LocationYmm": 11,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 36,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 75,
      "UpperThresholdFatal": 80
    },
    {
      "CurrentReading": 33,
      "Name": "36-Storage Batt",
      "Number": 36,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 8,
          "LocationYmm": 0,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "SystemBoard",
      "ReadingCelsius": 33,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 60,
      "UpperThresholdFatal": 0
    },
    {
      "CurrentReading": 41,
      "Name": "37-Fuse",
      "Number": 37,
      "Oem": {
        "Hp": {
          "@odata.type": "#HpSeaOfSensors.1.0.0.HpSeaOfSensors",
          "LocationXmm": 1,
          "LocationYmm": 8,
          "Type": "HpSeaOfSensors.1.0.0"
        }
      },
      "PhysicalContext": "PowerSupply",
      "ReadingCelsius": 41,
      "Status": {
        "Health": "OK",
        "State": "Enabled"
      },
      "Units": "Celsius",
      "UpperThresholdCritical": 100,
      "UpperThresholdFatal": 0
    }
  ],
  "Type": "ThermalMetrics.0.10.0",
  "links": {
    "self": {
      "href": "/redfish/v1/Chassis/1/Thermal/"
    }
  }
}

"###;
    
}