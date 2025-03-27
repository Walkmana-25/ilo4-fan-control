use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct CpuTemp {
    cpuid: u8,
    current: u8,
}

#[derive(Debug, PartialEq)]
pub struct Fan {
    name: String,
    current: u8,
    status: String,
}

#[derive(Debug, PartialEq)]
pub struct TempData {
    cpu_temps: Vec<CpuTemp>,
    high_temp_critical_reached_component: bool,
    high_temp_component_name: Vec<String>,
    num_fans: u8,
    fans: Vec<Fan>,
}

/// Retrieves temperature and fan data from an ILO interface
///
/// # Arguments
/// * `address` - The hostname or IP address of the ILO interface
///
/// # Returns
/// * `Result<TempData>` - Temperature and fan data or an error
///
/// This function makes an HTTPS request to the ILO's Redfish API endpoint
/// to get current temperature and fan status information.
pub async fn get_temp_data(address: &str, user: &str, password: &str) -> Result<TempData> {
    let url = format!("https://{}/redfish/v1/Chassis/1/Thermal", address);
    let json = get_ilo_data(&url, user, password).await?;
    let temp_data = json_parser(&json)?;
    Ok(temp_data)
}

/// Makes an HTTPS request to an ILO interface
///
/// # Arguments
/// * `url` - The complete URL to request from the ILO interface
///
/// # Returns
/// * `Result<String>` - The response body as a string or an error
///
/// This function creates an HTTPS client that accepts self-signed certificates
/// and makes a GET request to the specified URL.
async fn get_ilo_data(url: &str, user: &str, password: &str) -> Result<String> {
  let client = reqwest::Client::builder()
    .danger_accept_invalid_certs(true)
    .build()?;
  let resp = client.get(url)
    .basic_auth(user, Some(password))
    .send()
    .await?
    .text()
    .await?;
  Ok(resp)

}

/// Parses JSON response from ILO into TempData structure
///
/// # Arguments
/// * `json` - JSON string containing temperature and fan data
///
/// # Returns
/// * `Result<TempData>` - Parsed temperature and fan data or an error
///
/// This function extracts relevant temperature and fan information from
/// the ILO's JSON response and organizes it into a TempData structure.
fn json_parser(json: &str) -> Result<TempData> {
    let data: serde_json::Value = serde_json::from_str(json)?;

    // Get fan data
    let fan_data = match data.get("Fans") {
        None => {
            return Err(anyhow::anyhow!("No fan data found"));
        }
        Some(fan_data_raw) => fan_data_raw.as_array(),
    };
    // Get the array from
    // fan data to struct

    let mut fans: Vec<Fan> = Vec::new();

    match fan_data {
        None => {
            return Err(anyhow::anyhow!("No fan data found"));
        }
        Some(fan_data) => {
            for fan in fan_data {
                let fan_name = match fan.get("FanName") {
                    None => {
                        return Err(anyhow::anyhow!("No fan name found"));
                    }
                    Some(fan_name) => fan_name.as_str(),
                };
                let current_reading = match fan.get("CurrentReading") {
                    None => {
                        return Err(anyhow::anyhow!("No current reading found"));
                    }
                    Some(current_reading) => current_reading.as_u64(),
                };
                let status = match fan.get("Status") {
                    None => {
                        return Err(anyhow::anyhow!("No status found"));
                    }
                    Some(status) => status.get("Health").unwrap().as_str(),
                };
                let fan = Fan {
                    name: fan_name.unwrap().to_string(),
                    current: current_reading.unwrap() as u8,
                    status: status.unwrap().to_string(),
                };
                fans.push(fan);
            }
        }
    }

    // cpu temp data to struct
    let temperatures = match data.get("Temperatures") {
        None => {
            return Err(anyhow::anyhow!("No temperature data found"));
        }
        Some(temperatures) => temperatures.as_array().unwrap(),
    };

    let mut high_temp_critical_reached_component = false;
    let mut high_temp_component_name: Vec<String> = Vec::new();

    // check if high temp critical reached
    for temp in temperatures {
        // Get the current reading
        let current_reading = match temp.get("CurrentReading") {
            None => {
                return Err(anyhow::anyhow!("No current reading found"));
            }
            Some(current_reading) => current_reading.as_u64(),
        };

        //get UpperThresholdCritical
        let upper_threshold_critical = match temp.get("UpperThresholdCritical") {
            None => {
                return Err(anyhow::anyhow!("No UpperThresholdCritical found"));
            }
            Some(upper_threshold_critical) => upper_threshold_critical.as_u64(),
        };
        let current_component_name = match temp.get("Name") {
            None => {
                return Err(anyhow::anyhow!("No component name found"));
            }
            Some(current_component_name) => current_component_name.as_str(),
        };

        if current_reading.unwrap() > upper_threshold_critical.unwrap()
            && upper_threshold_critical.unwrap() != 0
        {
            high_temp_critical_reached_component = true;
            high_temp_component_name.push(current_component_name.unwrap().to_string());
        }
    }

    let mut cpu_temps: Vec<CpuTemp> = Vec::new();

    // Get the cpu temps
    for temp in temperatures {
        let physical_context = match temp.get("PhysicalContext") {
            None => {
                return Err(anyhow::anyhow!("No PhysicalContext found"));
            }
            Some(physical_context) => physical_context.as_str(),
        };

        if physical_context.unwrap() == "CPU" {
            let name = match temp.get("Name") {
                None => {
                    return Err(anyhow::anyhow!("No name found"));
                }
                Some(name) => name.as_str().unwrap(),
            };
            let current_reading = match temp.get("CurrentReading") {
                None => {
                    return Err(anyhow::anyhow!("No current reading found"));
                }
                Some(current_reading) => current_reading.as_u64().unwrap() as u8,
            };

            let cpu_temp = CpuTemp {
                cpuid: name
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u8>()
                    .unwrap(),
                current: current_reading,
            };

            cpu_temps.push(cpu_temp);
        }
    }

    let temp_data: TempData = TempData {
        cpu_temps,
        high_temp_critical_reached_component,
        high_temp_component_name,
        num_fans: fans.len() as u8,
        fans,
    };
    Ok(temp_data)
}

#[cfg(test)]
mod test {
    const ILO_JSON: &str = r###"
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
use super::*;

    #[test]
    fn test_json_parser() {
        let result = super::json_parser(ILO_JSON);
        assert!(result.is_ok());

    }
    
    #[test]
    fn test_json_parser_result() {
      let result = super::json_parser(ILO_JSON);
      
      let cpu_temps: Vec<super::CpuTemp> = vec![
        super::CpuTemp {
          cpuid: 1,
          current: 44,
        },
        super::CpuTemp {
          cpuid: 2,
          current: 47,
        },
      ];
      
      let fans: Vec<super::Fan> = vec![
        super::Fan {
          current: 11,
          name: "Fan 1".to_string(),
          status: "OK".to_string(),
        },
        super::Fan {
          current: 11,
          name: "Fan 2".to_string(),
          status: "OK".to_string(),
        },
        super::Fan {
          current: 11,
          name: "Fan 3".to_string(),
          status: "OK".to_string(),
        },
        super::Fan {
          current: 11,
          name: "Fan 4".to_string(),
          status: "OK".to_string(),
        },
        super::Fan {
          current: 11,
          name: "Fan 5".to_string(),
          status: "OK".to_string(),
        },
        super::Fan {
          current: 11,
          name: "Fan 6".to_string(),
          status: "OK".to_string(),
        },
        super::Fan {
          current: 11,
          name: "Fan 7".to_string(),
          status: "OK".to_string(),
        },
      ];
      
      let temp_data: super::TempData = super::TempData {
        cpu_temps,
        high_temp_critical_reached_component: false,
        high_temp_component_name: vec![],
        num_fans: 7,
        fans,
      };
      println!("temp_data: {:#?}", temp_data);
      println!("Result_data: {:#?}", result.as_ref().unwrap());

      
      assert_eq!(result.unwrap(), temp_data);
      
    }
    
    #[tokio::test]
    async fn test_async_function() {
        use std::process::Command;
        use std::thread::sleep;
        use std::time::Duration;
        use std::path::Path;

        // Start the test server as a background process
        println!("Starting test HTTPS server...");
        let server_dir = Path::new("/workspaces/ilo4-fan-control/test-https-server");
        let mut server_process = Command::new("python3")
            .arg("runserver.py")
            .current_dir(server_dir)
            .spawn()
            .expect("Failed to start test server");

        // Give the server a moment to start
        sleep(Duration::from_secs(2));
        
        // Run the actual test
        let result = get_ilo_data("https://localhost", "user", "pass").await;
        println!("Result: {:#?}", result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello-World-Test");
        
        // Clean up: kill the server process and wait for it to finish
        server_process.kill().expect("Failed to kill server process");
        server_process.wait().expect("Failed to wait for server process");
        println!("Test server stopped");
    }
    
}
