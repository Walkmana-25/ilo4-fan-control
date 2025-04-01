use anyhow::Ok;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use validator::{Validate, ValidationErrors};

/// Configuration for ILO fan control
///
/// This structure represents the top-level configuration for controlling fans
/// through the Integrated Lights-Out (ILO) interface.
#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct IloConfig {
    /// The period in seconds between fan control updates
    pub run_period_seconds: u8,
    /// List of ILO targets to control
    #[validate(nested)]
    pub targets: Vec<TargetIlo>,
}

/// Configuration for a single ILO target
///
/// This structure contains connection details and fan control settings
/// for a specific ILO interface.
#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct TargetIlo {
    /// The hostname or IP address of the ILO interface
    pub host: String,
    /// Username for ILO authentication
    pub user: String,
    /// Password for ILO authentication
    pub password: String,
    /// Fan control target configuration
    pub target_fans: TargetFans,
    /// Temperature-based fan speed configuration
    #[validate(nested)]
    pub temperature_fan_config: Vec<FanConfig>,
}

/// Configuration for temperature-based fan control
///
/// Defines the fan speed settings for specific temperature ranges.
#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct FanConfig {
    /// Minimum temperature threshold in Celsius
    pub min_temp: u8,
    /// Maximum temperature threshold in Celsius
    pub max_temp: u8,
    /// Maximum fan speed percentage (0-100)
    #[validate(range(min = 0, max = 100))]
    pub max_fan_speed: u8,
}

/// Fan target specification
///
/// Specifies either the number of fans to control or specific fan indices.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TargetFans {
    /// Control a specific number of fans (starting from index 0)
    NumFans(u8),
    /// Control specific fans by their indices
    TargetFans(Vec<u8>),
}

impl IloConfig {
    /// Validates the configuration according to the defined validation rules
    ///
    /// Checks that all nested configurations are valid, including fan speed ranges
    /// and any other constraints defined by the Validate trait.
    ///
    /// # Returns
    ///
    /// * `Result<(), ValidationErrors>` - Ok if validation passes, or ValidationErrors if it fails
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
        config.validate()?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_valid_config() -> IloConfig {
        IloConfig {
            run_period_seconds: 60,
            targets: vec![
                TargetIlo {
                    host: "192.168.1.100".to_string(),
                    user: "admin".to_string(),
                    password: "password123".to_string(),
                    target_fans: TargetFans::NumFans(3),
                    temperature_fan_config: vec![
                        FanConfig {
                            min_temp: 30,
                            max_temp: 50,
                            max_fan_speed: 50,
                        },
                        FanConfig {
                            min_temp: 51,
                            max_temp: 70,
                            max_fan_speed: 100,
                        },
                    ],
                },
                TargetIlo {
                    host: "192.168.1.101".to_string(),
                    user: "admin".to_string(),
                    password: "password456".to_string(),
                    target_fans: TargetFans::TargetFans(vec![1, 2]),
                    temperature_fan_config: vec![
                        FanConfig {
                            min_temp: 25,
                            max_temp: 40,
                            max_fan_speed: 30,
                        },
                        FanConfig {
                            min_temp: 41,
                            max_temp: 60,
                            max_fan_speed: 80,
                        },
                    ],
                },
            ],
        }
    }

    fn write_config_to_temp_file(config: &IloConfig) -> Result<NamedTempFile> {
        let mut temp_file = NamedTempFile::new()?;
        let toml_string = toml::to_string_pretty(config)?;
        write!(temp_file, "{}", toml_string)?;
        Ok(temp_file)
    }

    #[test]
    fn test_valid_config_validation() {
        let config = create_valid_config();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_fan_speed_validation() {
        let mut config = create_valid_config();
        // Set an invalid fan speed above 100%
        config.targets[0].temperature_fan_config[0].max_fan_speed = 120;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_load_from_toml_file() -> Result<()> {
        let config = create_valid_config();
        let temp_file = write_config_to_temp_file(&config)?;

        let loaded_config = IloConfig::from_toml_file(temp_file.path())?;

        assert_eq!(loaded_config.run_period_seconds, config.run_period_seconds);
        assert_eq!(loaded_config.targets.len(), config.targets.len());

        // Check first target details
        assert_eq!(loaded_config.targets[0].host, config.targets[0].host);
        assert_eq!(loaded_config.targets[0].user, config.targets[0].user);

        // Check if we correctly loaded the fan configuration
        let first_target_fan_config = &loaded_config.targets[0].temperature_fan_config[0];
        assert_eq!(first_target_fan_config.min_temp, 30);
        assert_eq!(first_target_fan_config.max_temp, 50);
        assert_eq!(first_target_fan_config.max_fan_speed, 50);

        Ok(())
    }

    #[test]
    fn test_save_to_toml_file() -> Result<()> {
        let config = create_valid_config();
        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path().to_path_buf();

        config.save_to_toml_file(&temp_path)?;

        // Read the file and parse it back
        let loaded_config = IloConfig::from_toml_file(&temp_path)?;

        // Verify the round trip worked correctly
        assert_eq!(loaded_config.run_period_seconds, config.run_period_seconds);
        assert_eq!(loaded_config.targets.len(), config.targets.len());

        Ok(())
    }

    #[test]
    fn test_target_fans_variants() {
        let config = create_valid_config();

        // Check NumFans variant
        match &config.targets[0].target_fans {
            TargetFans::NumFans(num) => assert_eq!(*num, 3),
            _ => panic!("Expected NumFans variant"),
        }

        // Check TargetFans variant
        match &config.targets[1].target_fans {
            TargetFans::TargetFans(fans) => {
                assert_eq!(fans.len(), 2);
                assert_eq!(fans[0], 1);
                assert_eq!(fans[1], 2);
            }
            _ => panic!("Expected TargetFans variant"),
        }
    }

    #[test]
    fn test_invalid_file_path() {
        let result = IloConfig::from_toml_file("/nonexistent/path/config.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_toml_content() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        write!(temp_file, "This is not valid TOML content")?;

        let result = IloConfig::from_toml_file(temp_file.path());
        assert!(result.is_err());

        Ok(())
    }
}
