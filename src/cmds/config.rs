use log::debug;
use crate::config::IloConfig;
use std::fs;
use anyhow::{Error, Result};
use toml::Value;


/// Validates the configuration file at the specified path
/// 
/// # Arguments
/// * `path` - Path to the configuration file to validate
/// 
/// # Returns
/// * `Result<String>` - Ok if validation passes, or an error message if it fails
/// 
/// # Example
/// ```no_run
/// 
/// use ilo4_fan_control::cmds::config_check;
/// 
/// let path = "path/to/config.toml".to_string();
/// match config_check(path) {
///    Ok(message) => {
///       println!("Validation passed: {}", message);
///   }
///   Err(e) => {
///      println!("Validation failed: {}", e);
///   }
/// }
/// ```
/// 
pub fn config_check(path: String) -> Result<String> {

    debug!("Checking configuration at path: {}", path);

    match toml_validation(path.clone()) {
        Ok(_) => {
            debug!("TOML syntax validation passed");
        }
        Err(e) => {
            debug!("TOML syntax validation failed: {}", e);
            
            return Err(Error::msg(format!("TOML syntax validation failed: {}", e)));
        }
    }
    match config_validation(path.clone()) {
        Ok(_) => {
            debug!("Configuration validation passed");
            Ok("Configuration validation passed".to_string())
        }
        Err(e) => {
            debug!("Configuration validation failed: {}", e);
            Err(Error::msg(format!("Configuration validation failed: {}", e)))
        }
    }
}


/// Validates the configuration content against the IloConfig structure
/// 
/// This function loads the configuration from the specified path and
/// validates it against the IloConfig structure's requirements.
///
/// # Arguments
///
/// * `path` - Path to the configuration file to validate
///
/// # Returns
///
/// * `Result<String>` - Ok with success message if validation passes, or Err with error details
///
/// # Example
///
/// ```no_run
/// use ilo4_fan_control::cmds::config_validation;
///
/// let path = "path/to/config.toml".to_string();
/// match config_validation(path) {
///     Ok(message) => println!("Validation result: {}", message),
///     Err(e) => println!("Validation failed: {}", e)
/// }
/// ```
fn config_validation(path: String) -> Result<String> {
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
fn toml_validation(path: String) -> Result<String> {
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