use log::{
    info, error, debug
};
use crate::config::IloConfig;
use std::fs;
use anyhow::{Error, Result};
use toml::Value;


pub fn config_validation(path: String) -> Result<String> {
    debug!("Validating config file");
    
    // Load the configuration from the specified path
    let config = match IloConfig::from_toml_file(path) {
        Ok(cfg) => cfg,
        Err(e) => {
            return Err(Error::msg(format!("Failed to load configuration: {}", e)));
        }
    };
    
    debug!("Configuration loaded successfully");
    
    // Validate the configuration
    match config.validate() {
        Ok(_) => {
            Ok("Configuration is valid".to_string())
        }
        Err(e) => {
            Err(Error::msg(format!("Configuration validation failed: {}", e)))
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