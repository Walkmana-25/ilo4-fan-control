use std::thread;

use anyhow::Result;
use log::{debug, error, info};

use crate::config::TargetIlo;
use crate::cputemp;
use crate::ssh;
use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn start_daemon(config_path: String) -> Result<()> {
    debug!("Starting daemon with config path: {}", config_path);

    // Validate the configuration file
    debug!("Validating configuration file");
    match crate::cmds::config::config_check(config_path.clone()) {
        Ok(_) => {
            info!("Configuration validation passed");
        }
        Err(e) => {
            return Err(e);
        }
    }

    // Read the configuration files
    let config = crate::config::IloConfig::from_toml_file(&config_path)?;

    let running_interval = config.run_period_seconds;

    // Initialize the Tokio runtime
    // Run the daemon main function
    let rt = tokio::runtime::Runtime::new()?;
    loop {
        // Run the control function
        rt.block_on(daemon_main(config.clone()))?;

        // Sleep for the specified interval
        info!("Sleeping for {} seconds", running_interval);
        std::thread::sleep(std::time::Duration::from_secs(running_interval as u64));
    }
}

async fn daemon_main(config: crate::config::IloConfig) -> Result<()> {
    info!("Daemon main function started");

    let mut handles = Vec::new();

    for target in config.targets.iter() {
        let target_clone = target.clone();

        let handle = thread::spawn(move || {
            // 各スレッド内で新しいランタイムを作成
            let thread_rt =
                tokio::runtime::Runtime::new().expect("Failed to create runtime in thread");
            thread_rt.block_on(runner(target_clone))
        });

        handles.push(handle);
    }

    for handle in handles {
        handle
            .join()
            .map_err(|_| anyhow::anyhow!("Thread panicked"))??;
    }

    info!("All hosts completed successfully");
    Ok(())
}

async fn runner(config: TargetIlo) -> Result<()> {
    let password = config.password_base64.clone();
    let host = config.host.clone();
    let user = config.user.clone();

    info!("Fan controller for host: {}", &host);
    debug!("User: {}", &user);

    // Get the current temperature
    let temprature = cputemp::get_temp_data(&host, &user, &password).await?;

    info!(
        "Current CPU 0 Temp of {}: {:?}°C",
        &host, &temprature.cpu_temps[0].current
    );
    debug!("Detail data of {}:\n {}", &host, &temprature);

    let mut max_cpu_temp = 0;

    temprature.cpu_temps.iter().for_each(|temp| {
        if temp.current > max_cpu_temp {
            max_cpu_temp = temp.current;
        }
    });

    // Generate fan commands based on the current temperature
    let commands = crate::gen_ssh::generate_fan_commands(&config, max_cpu_temp);
    debug!("Fan control commands for {}: {:?}", &host, &commands);

    // Execute the fan control commands
    let mut client = ssh::SshClient::new(host.clone(), user, password);
    match client.connect() {
        Ok(_) => {
            debug!("Connected to {}", &host);
        }
        Err(e) => {
            error!("Failed to connect to {}: {}", &host, e);
            return Err(e);
        }
    }

    match client.exec(commands) {
        Ok(output) => {
            debug!("Fan control output for {}: {:?}", &host, output);
        }
        Err(e) => {
            error!("Failed to execute commands on {}: {}", &host, e);
            return Err(e);
        }
    }

    Ok(())
}
