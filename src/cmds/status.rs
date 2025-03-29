
use log::{
    info, error, debug
};
use tokio::runtime::Runtime;

pub fn show_status(host: Option<String>, user: Option<String>, password: Option<String>) {
    debug!("Showing status");
    
    let (host, user, password) = crate::cmds::utils::get_connection_info(host, user, password);  

    // Initialize the Tokio runtime
    // Run show status runner for get temp
    let rt = Runtime::new();
    
    match rt {
        Ok(runtime) => {
            debug!("Runtime created successfully");
            runtime.block_on(show_status_runner(
                host,
                user,
                password
            ));
        }
        Err(e) => {
            error!("Failed to create runtime: {}", e);
            std::process::exit(1);
        }
    }

}

async fn show_status_runner(host: String, user: String, password: String) {
    info!("Connecting to iLO4 at {}@{}", user, host);
    
    match crate::cputemp::get_temp_data(host.as_str(), user.as_str(), password.as_str()).await {
        Ok(temp_data) => {
            info!("Temperature data:\n {}", temp_data);
        }
        Err(e) => {
            error!("Failed to get temperature data: {}", e);
        }
    
        
    }
    

}