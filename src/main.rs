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
    #[arg(short, long, default_value = "off")]
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

    /// Sets the fan speed manually
    Set {
        /// Fan speed percentage (0-100)
        #[arg(short, long)]
        speed: u8,
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

    info!("Starting HPE iLO4 Fan Control Utility");

    match &cli.command {
        Commands::Status => {
            println!("現在のファンステータスを取得しています...");
            // ここにステータス取得のロジックを実装
        }
        Commands::Set { speed } => {
            if *speed > 100 {
                error!("速度は0-100%の範囲で指定してください");
                process::exit(1);
            }
            println!("ファン速度を{}%に設定しています...", speed);
            // ここに速度設定のロジックを実装
        }
        Commands::Auto => {
            println!("ファンを自動モードに切り替えています...");
            // ここに自動モード切替のロジックを実装
        }
    }

    Ok(())
}
