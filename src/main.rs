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

    Config {
        /// Path to the configuration file
        #[arg(short, long)]
        path: String,
        
        /// Generate sample configuration file
        #[arg(short, long)]
        sample: bool,
    },

    /// Sets the fan control to automatic mode
    Auto,
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
        _ => {
            error!("Unknown command");
            process::exit(1);
        }
    }

    Ok(())
}
