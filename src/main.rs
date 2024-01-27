#![allow(unused)]
use std::path::PathBuf;

use crate::settings::Settings;
use api::AppState;
use axum::extract::Path;
use clap::Parser;
use cli::{Cli, Commands};
use color_eyre::Result;
use scanner::scan_library;

mod api;
mod cli;
mod db;
mod scanner;
mod settings;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let config = Settings::new(cli.config_path)?;

    match cli.command {
        Commands::Admin => {
            let db = db::connect(config.database_url.clone())?;
            scan_library();

        }
        Commands::Server => {
            let pool = db::get_connection_pool(config.database_url.clone())?;
            let state = AppState { config };
            let app = api::router().with_state(state);
            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
    }

    Ok(())
}
