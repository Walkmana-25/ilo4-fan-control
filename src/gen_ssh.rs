use crate::config::{IloConfig, TargetFans, TargetIlo};

/// Generates fan control commands for a specific target ILO configuration
///
/// # Arguments
///
/// * `target` - Target ILO configuration
/// * `current_temp` - Current temperature reading
///
/// # Returns
///
/// * `Vec<String>` - List of fan control commands
///
/// # Example
///
/// ```
/// use ilo4_fan_control::config::{FanConfig, TargetFans, TargetIlo};
/// use ilo4_fan_control::gen_ssh::generate_fan_commands;
/// let target_ilo = TargetIlo {
///     host: String::from("example_host"), // Added host initialization
///     user: String::from("admin"),
///     password: String::from("password"),
///     
///     target_fans: TargetFans::NumFans(4),
///     temprature_fan_config: vec![
///         FanConfig {
///             min_temp: 0,
///             max_temp: 30,
///             max_fan_speed: 50,
///         },
///         FanConfig {
///             min_temp: 31,
///             max_temp: 60,
///             max_fan_speed: 75,
///         },
///       
///       ]
/// };
///
/// let commands = generate_fan_commands(&target_ilo, 45);
/// ```
pub fn generate_fan_commands(target: &TargetIlo, current_temp: u8) -> Vec<String> {
    let mut commands = Vec::new();

    // Find the appropriate fan config based on current temperature
    let fan_config = target
        .temprature_fan_config
        .iter()
        .find(|config| current_temp >= config.min_temp && current_temp <= config.max_temp);

    // If no matching fan config found, return empty commands
    let fan_speed = match fan_config {
        Some(config) => {
            // Calculate the fan speed threshold (0-255 scale from percentage)
            ((config.max_fan_speed as f32) * 2.55).round() as u8
        }
        None => return commands,
    };

    // Generate commands for each fan
    match &target.target_fans {
        TargetFans::NumFans(count) => {
            for fan_number in 1..=*count {
                commands.push(format!("fan p {} max {}", fan_number, fan_speed));
            }
        }
        TargetFans::TargetFans(fans) => {
            for &fan_number in fans {
                commands.push(format!("fan p {} max {}", fan_number, fan_speed));
            }
        }
    }

    commands
}
