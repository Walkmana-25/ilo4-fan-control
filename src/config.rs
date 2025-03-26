use anyhow::Ok;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct IloConfig {
    run_period_seconds: u8,
    targets: Vec<TargetIlo>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct TargetIlo {
    host: String,
    user: String,
    password: String,
    target_fans: TargetFans,
    temprature_fan_config: Vec<FanConfig>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct FanConfig {
    min_temp: u8,
    max_temp: u8,
    #[validate(range(min = 0, max = 100))]
    max_fan_speed: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TargetFans {
    NumFans(u8),
    TargetFans(Vec<u8>),
}


impl IloConfig {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        <Self as Validate>::validate(self)
    }
    
    /// Loads configuration from a TOML file
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the TOML configuration file
    /// 
    /// # Returns
    /// 
    /// * `Result<IloConfig>` - Parsed ILO configuration or an error
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use ilo4_fan_control::config::IloConfig;
    /// 
    /// let config = IloConfig::from_toml_file("config.toml").expect("Failed to load config");
    /// ```
    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: IloConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    /// Saves configuration to a TOML file
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to save the TOML configuration file
    /// 
    /// # Returns
    /// 
    /// * `Result<()>` - Success or an error
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use ilo4_fan_control::config::IloConfig;
    /// 
    /// let config = IloConfig::from_toml_file("config.toml").unwrap();
    /// config.save_to_toml_file("new_config.toml").expect("Failed to save config");
    /// ```
    pub fn save_to_toml_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(path, toml_string)?;
        Ok(())
    }
}