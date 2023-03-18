use anyhow::Result;
pub mod api;
pub mod db;
pub mod metadata;
pub mod scheduler;
pub use crate::db::establish_connection;
use actix_web::rt::spawn;
pub mod settings;
use crate::db::create_connection_pool;
use log::info;
use std::collections::HashMap;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config = settings::get_config()?;
    let pool = create_connection_pool()?;

    info!(
        "{:?}",
        config
            .clone()
            .try_deserialize::<HashMap<String, String>>()?
    );

    let scheduler = spawn(scheduler::run_schedule(pool.clone()));

    let api = spawn(api::run(pool.clone()));

    scheduler.await?.unwrap();
    api.await?.unwrap();

    Ok(())
}
