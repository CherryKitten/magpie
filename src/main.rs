use std::collections::HashMap;

use log::{error, info};
use tokio::{spawn, try_join};

pub use error::{Error, Result};

use crate::db::create_connection_pool;

pub mod api;
pub mod db;
pub mod error;
pub mod metadata;
pub mod scheduler;
pub mod settings;

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
