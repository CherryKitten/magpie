#![allow(unused, dead_code)]
use clap::Parser;
use color_eyre::Result;
mod cli;
mod config;
mod database;
mod scanner;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let cli = cli::Cli::parse();
    let config = config::Config::new(cli.config_path)?;

    use cli::Commands;
    match cli.command {
        Commands::Admin => {
            println!("Hello, Nya!");
            unimplemented!();
        }
        Commands::Server => {
            tracing::info!("Starting up Magpie server...");
            server::run(config);
        }
        Commands::Debug => {
            scanner::scan_library(config.library_path.into());
        }
    };

    Ok(())
}
