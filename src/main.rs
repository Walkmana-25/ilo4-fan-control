use std::process;
use clap::{Parser, Subcommand};
use log::{info, error};
use anyhow::Result;

mod ssh;
mod cputemp;
mod config;
mod gen_ssh;
mod cmds;

/// HPE iLO4 Fan Control Utility
///
/// Command line interface for controlling fan speeds on HPE servers through
/// their iLO4 management interface. Supports automatic temperature-based 
/// fan speed control.
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    /// Set the log level for the application
    #[arg(short, long, default_value = "info")]
    log_level: String,
    
    /// iLO4 host address
    #[arg(long)]
    host: Option<String>,
    /// iLO4 username
    #[arg(long)]
    user: Option<String>,
    /// iLO4 password
    #[arg(long)]
    password: Option<String>, // Changed to Option<String>

    /// No interactive mode
    #[arg(long)]
    no_interactive: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Displays the current fan status
    Status,

    /// Generates sample configuration files and Validate the configuration
    Config {
        /// Path to the target configuration file
        #[arg(short, long)]
        path: String,
        
        /// Generate sample configuration file
        #[arg(short, long)]
        sample: bool,
        
        /// Validate the configuration file against the IloConfig schema
        #[arg(short, long)]
        validate: bool,
        
    },

    /// Daemon mode for continuous monitoring and control
    Daemon {
        /// Path to the configuration file
        #[arg(short, long)]
        config_path: String,
        
    }
}

/// Main entry point for the fan control application
///
/// Reads configuration, establishes connections to iLO interfaces,
/// monitors temperatures, and adjusts fan speeds accordingly.
fn main() -> Result<()> {
    let cli = Cli::parse();

    // ログの初期化
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&cli.log_level))
        .init();

    info!("Fan Control Utility for HPE iLO4\n");

    match &cli.command {
        Commands::Status => {
            cmds::status::show_status(
                cli.host.clone(),
                cli.user.clone(),
                cli.password.clone(),
            );
        }
        Commands::Config { path, sample, validate } => {
            if *sample && *validate {
                error!("Please use only one of --sample, --validate at a time");
                process::exit(1);
            } else if *sample {
                cmds::sample::show_sample(path.clone());
            } else if *validate {
                match cmds::config::toml_validation(path.clone()) {
                    Ok(_) => {
                        info!("TOML syntax validation passed");
                    }
                    Err(e) => {
                        error!("TOML syntax validation failed: {}", e);
                        process::exit(1);
                    }
                }
                match cmds::config::config_validation(path.clone()) {
                    Ok(_) => {
                        info!("Configuration validation passed");
                    }
                    Err(e) => {
                        error!("Configuration validation failed: {}", e);
                        process::exit(1);
                    }
                }
            } else {
                error!("Please specify --sample or --validate");
                process::exit(1);
            }   
        }
        
        Commands::Daemon { config_path } => {
            info!("Starting daemon with config path: {}", config_path);
            // Daemon logic here
            // For example, you can call a function to start the daemon
            // cmds::daemon::start_daemon(config_path.clone());
        }
    }

    Ok(())
}
