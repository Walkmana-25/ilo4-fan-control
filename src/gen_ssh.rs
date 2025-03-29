use crate::config::{TargetFans, TargetIlo};

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
            for fan_number in 0..=*count - 1 {
                commands.push(format!("fan p {} max {}", fan_number, fan_speed));
            }
        }
        TargetFans::TargetFans(fans) => {
            for &fan_number in fans {
                commands.push(format!("fan p {} max {}", fan_number - 1, fan_speed));
            }
        }
    }

    commands
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FanConfig;

    /// テスト用のTargetIloインスタンスを作成する補助関数
    fn create_test_target(fan_type: TargetFans) -> TargetIlo {
        TargetIlo {
            host: String::from("example.host.com"),
            user: String::from("admin"),
            password: String::from("password"),
            target_fans: fan_type,
            temprature_fan_config: vec![
                FanConfig {
                    min_temp: 0,
                    max_temp: 30,
                    max_fan_speed: 50, // 50% = 128 in 0-255 scale
                },
                FanConfig {
                    min_temp: 31,
                    max_temp: 60,
                    max_fan_speed: 75, // 75% = 191 in 0-255 scale
                },
                FanConfig {
                    min_temp: 61,
                    max_temp: 85,
                    max_fan_speed: 100, // 100% = 255 in 0-255 scale
                },
            ],
        }
    }

    #[test]
    fn test_generate_fan_commands_num_fans() {
        // NumFans形式でのテスト
        let target = create_test_target(TargetFans::NumFans(3));

        // 低温域でのテスト (0-30℃)
        let low_temp_commands = generate_fan_commands(&target, 25);
        assert_eq!(
            low_temp_commands.len(),
            3,
            "3つのファンコマンドが生成されるべき"
        );
        assert_eq!(low_temp_commands[0], "fan p 0 max 128");
        assert_eq!(low_temp_commands[1], "fan p 1 max 128");
        assert_eq!(low_temp_commands[2], "fan p 2 max 128");

        // 中温域でのテスト (31-60℃)
        let mid_temp_commands = generate_fan_commands(&target, 45);
        assert_eq!(mid_temp_commands.len(), 3);
        assert_eq!(mid_temp_commands[0], "fan p 0 max 191");
        assert_eq!(mid_temp_commands[1], "fan p 1 max 191");
        assert_eq!(mid_temp_commands[2], "fan p 2 max 191");

        // 高温域でのテスト (61-85℃)
        let high_temp_commands = generate_fan_commands(&target, 70);
        assert_eq!(high_temp_commands.len(), 3);
        assert_eq!(high_temp_commands[0], "fan p 0 max 255");
        assert_eq!(high_temp_commands[1], "fan p 1 max 255");
        assert_eq!(high_temp_commands[2], "fan p 2 max 255");
    }

    #[test]
    fn test_generate_fan_commands_target_fans() {
        // 特定のファン番号を指定するケース
        let target = create_test_target(TargetFans::TargetFans(vec![1, 3, 5]));

        // 中温域でのテスト
        let commands = generate_fan_commands(&target, 45);
        assert_eq!(commands.len(), 3, "3つのファンコマンドが生成されるべき");
        assert_eq!(commands[0], "fan p 0 max 191");
        assert_eq!(commands[1], "fan p 2 max 191");
        assert_eq!(commands[2], "fan p 4 max 191");
    }

    #[test]
    fn test_generate_fan_commands_out_of_range() {
        // 温度範囲外のケース
        let target = create_test_target(TargetFans::NumFans(2));

        // 設定された範囲よりも高温
        let high_out_commands = generate_fan_commands(&target, 90);
        assert_eq!(
            high_out_commands.len(),
            0,
            "範囲外の温度では空のコマンドリストが返されるべき"
        );

        // 最小温度と最大温度が厳密に比較されることを確認
        let target_with_gap = TargetIlo {
            host: String::from("example.com"),
            user: String::from("admin"),
            password: String::from("password"),
            target_fans: TargetFans::NumFans(1),
            temprature_fan_config: vec![
                FanConfig {
                    min_temp: 20,
                    max_temp: 30,
                    max_fan_speed: 50,
                },
                FanConfig {
                    min_temp: 40,
                    max_temp: 50,
                    max_fan_speed: 75,
                },
            ],
        };

        // 設定の隙間に当たる温度
        let gap_commands = generate_fan_commands(&target_with_gap, 35);
        assert_eq!(
            gap_commands.len(),
            0,
            "温度範囲の隙間では空のコマンドリストが返されるべき"
        );
    }

    #[test]
    fn test_fan_speed_calculation() {
        // ファン速度の計算が正しいことを検証
        let test_cases = [
            // (max_fan_speed_percentage, expected_0_255_value)
            (0, 0),
            (1, 3),     // 約2.55を四捨五入
            (10, 26),   // 25.5を四捨五入
            (25, 64),   // 63.75を四捨五入
            (50, 128),  // 127.5を四捨五入
            (75, 191),  // 191.25を四捨五入
            (99, 252),  // 252.45を四捨五入
            (100, 255), // 255
        ];

        for (percentage, expected) in test_cases.iter() {
            let target = TargetIlo {
                host: String::from("example.com"),
                user: String::from("admin"),
                password: String::from("password"),
                target_fans: TargetFans::NumFans(1),
                temprature_fan_config: vec![FanConfig {
                    min_temp: 0,
                    max_temp: 100,
                    max_fan_speed: *percentage,
                }],
            };

            let commands = generate_fan_commands(&target, 50); // Use a temperature in the valid range

            assert_eq!(commands.len(), 1);
            assert_eq!(commands[0], format!("fan p 0 max {}", expected));
        }
    }
}
