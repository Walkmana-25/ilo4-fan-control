use log::{
    info, error, debug
};
use crate::config::{
    IloConfig,
    TargetIlo,
    TargetFans,
    FanConfig
};


pub fn show_sample(path: String) {
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
        target_fans,
        temprature_fan_config: fan_config,
        
    };
    let ilo_config = IloConfig {
        run_period_seconds: 60,
        targets: vec![target_ilo],
    };
    
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