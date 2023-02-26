use anyhow::Result;
pub mod db;
pub mod scheduler;
pub use crate::db::establish_connection;
use actix_web::rt::spawn;
pub mod settings;
use log::{error, info};
use std::collections::HashMap;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config = settings::get_config()?;

    info!(
        "{:?}",
        config
            .clone()
            .try_deserialize::<HashMap<String, String>>()?
    );

    if config.get_string("library").is_err() {
        error!("No library configured, exiting...");
        // If the library is not set correctly, we exit with EX_CONFIG (78)
        std::process::exit(78)
    }

    if establish_connection().is_err() {
        error!("Failed connecting to database, exiting...");
        // If the database connection fails, we exit with EX_UNAVAILABLE (69)
        std::process::exit(69);
    }

    spawn(scheduler::run_schedule()).await?;

    Ok(())
}
