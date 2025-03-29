use log::{
    info, error, debug
};
use anyhow::Result;

pub fn start_daemon(config_path: String) -> Result<()> {
    debug!("Starting daemon with config path: {}", config_path);
    
    // Read the configuration files
    let config = crate::config::IloConfig::from_toml_file(&config_path)?;
    
    let running_interval = config.run_period_seconds;
    
    // Initialize the Tokio runtime
    // Run the daemon main function
    let rt = tokio::runtime::Runtime::new()?;
    loop {
        // Run the control function
        info!("Running control function");
        rt.block_on(daemon_main(config.clone()))?;
        
        // Sleep for the specified interval
        info!("Sleeping for {} seconds", running_interval);
        std::thread::sleep(std::time::Duration::from_secs(running_interval as u64));
    }
    
    
    
}

async fn daemon_main(config: crate::config::IloConfig) -> Result<()> {
    info!("Daemon main function started");
    
    // Daemon logic here
    // For example, you can call a function to start the daemon
    // cmds::daemon::start_daemon(config_path.clone());
    
    Ok(())
}