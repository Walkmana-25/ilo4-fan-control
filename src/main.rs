use std::process;
use clap::{Parser, Subcommand};
use log::{info, error};
use anyhow::{Result, Context};

mod ssh;
mod cputemp;

/// HP iLO4サーバー用ファン制御ユーティリティ
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// ログレベルの設定
    #[arg(short, long, default_value = "info")]
    log_level: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 現在のファン状態を表示
    Status {
        /// 詳細情報を表示
        #[arg(short, long)]
        verbose: bool,
    },

    /// ファン速度を手動で設定
    Set {
        /// ファンの速度 (0-100%)
        #[arg(short, long)]
        speed: u8,
    },

    /// 自動モードへ切り替え
    Auto,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // ログの初期化
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&cli.log_level))
        .init();

    info!("iLO4 ファン制御ユーティリティを開始します");

    match &cli.command {
        Commands::Status { verbose } => {
            println!("現在のファンステータスを取得しています...");
            if *verbose {
                println!("詳細モードが有効です");
            }
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
