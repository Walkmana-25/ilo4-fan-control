use crate::config::{FanConfig, IloConfig, TargetFans, TargetIlo};
use log::{debug, error, info};

pub fn show_sample(path: String, dual: bool) {
    let target_fans = TargetFans::NumFans(7);
    let fan_config = vec![
        FanConfig {
            min_temp: 0,
            max_temp: 55,
            max_fan_speed: 20,
        },
        FanConfig {
            min_temp: 55,
            max_temp: 60,
            max_fan_speed: 40,
        },
        FanConfig {
            min_temp: 61,
            max_temp: 70,
            max_fan_speed: 70,
        },
        FanConfig {
            min_temp: 71,
            max_temp: 100,
            max_fan_speed: 100,
        },
    ];
    let target_ilo = TargetIlo {
        host: String::from("ILO_HOST_NAME_OR_IP_ADDRESS"),
        user: String::from("USERNAME"),
        password: String::from("PASSWORD"),
        target_fans: target_fans.clone(),
        temprature_fan_config: fan_config.clone(),
    };
    let mut ilo_config = IloConfig {
        run_period_seconds: 60,
        targets: vec![target_ilo],
    };

    if dual {
        let target_ilo2 = TargetIlo {
            host: String::from("ILO_HOST2_NAME_OR_IP_ADDRESS"),
            user: String::from("USERNAME"),
            password: String::from("PASSWORD"),
            target_fans,
            temprature_fan_config: fan_config,
        };
        ilo_config.targets.push(target_ilo2);
    }

    debug!("Generated Sample Config: {:?}", ilo_config);

    // Save the sample configuration to a file
    match ilo_config.save_to_toml_file(&path) {
        Ok(_) => {
            info!("Sample configuration saved to {}", &path);
        }
        Err(e) => {
            error!("Failed to save sample configuration: {}", e);
        }
    }
}
