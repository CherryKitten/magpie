use log::{error, info};
use magpie::api;
use magpie::db::{create_connection_pool, run_migrations};
use magpie::scheduler;
use magpie::settings;
use magpie::Result;
use std::collections::HashMap;
use tokio::{spawn, try_join};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = settings::get_config()?;
    let pool = create_connection_pool()?;

    run_migrations(&mut pool.get()?)?;

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
