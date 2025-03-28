use log::{
    info, error, debug
};
use crate::config::IloConfig;
use std::fs;
use anyhow::{Error, Result};
use toml::Value;


pub fn config_validation(path: String) {
    debug!("Validating config file");
    
    // Load the configuration from the specified path
    let config = IloConfig::from_toml_file(path).expect("Failed to load config");
    
    // Validate the configuration
    match config.validate() {
        Ok(_) => {
            info!("Configuration is valid");
        }
        Err(e) => {
            error!("Configuration validation failed: {}", e);
        }
    }
}

/// Validates that a file contains valid TOML syntax
/// 
/// Unlike config_validation, this function only checks if the file has valid TOML syntax
/// without validating it against the IloConfig structure.
///
/// # Arguments
///
/// * `path` - Path to the TOML file to validate
pub fn toml_validation(path: String) -> Result<String> {
    debug!("Validating TOML syntax in file: {}", path);
    
    // Read the file content
    match fs::read_to_string(&path) {
        Ok(content) => {
            // Try to parse the content as TOML
            match content.parse::<Value>() {
                Ok(_) => {
                    Ok("The file contains valid TOML syntax".to_string())
                }
                Err(e) => {
                    Err(Error::msg(format!("TOML syntax validation failed: {}", e)))
                }
            }
        }
        Err(e) => {
            Err(Error::msg(format!("Failed to read file '{}': {}", path, e)))
        }
    }
}