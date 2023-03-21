use anyhow::Result;
pub mod api;
pub mod db;
pub mod metadata;
pub mod scheduler;
pub use crate::db::establish_connection;
pub mod settings;
use crate::db::create_connection_pool;
use log::{error, info};
use std::collections::HashMap;
use tokio::{spawn, try_join};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = settings::get_config()?;
    let pool = create_connection_pool()?;

    info!(
        "{:?}",
        config
            .clone()
            .try_deserialize::<HashMap<String, String>>()?
    );

    let api = spawn(api::run(pool.clone()));
    let scheduler = spawn(scheduler::run_schedule(pool));

    let (api, scheduler) = try_join!(api, scheduler)?;

    if let Err(error) = api {
        error!("{error}");
        std::process::exit(1);
    }

    if let Err(error) = scheduler {
        error!("{error}");
        std::process::exit(1);
    }

    Ok(())
}
