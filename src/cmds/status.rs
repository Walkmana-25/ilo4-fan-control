
use log::{
    info, error,debug
};
use tokio::runtime::Runtime;

pub fn show_status(host: Option<String>, user: Option<String>, password: Option<String>) {
    debug!("Showing status");

    

    // Initialize the Tokio runtime
    // Run show status runner for get temp
    let rt = Runtime::new();
    
    match rt {
        Ok(runtime) => {
            debug!("Runtime created successfully");
            runtime.block_on(show_status_runner());
        }
        Err(e) => {
            println!("A problem occurred.");
            println!("If you need more information, please set loglevel.");
            error!("Failed to create runtime: {}", e);
            std::process::exit(1);
        }
    }

}

async fn show_status_runner() {

}