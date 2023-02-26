use anyhow::{Result};
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

    Ok(())
}
