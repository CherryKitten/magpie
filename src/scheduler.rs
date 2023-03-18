use crate::db::DbPool;
use actix_web::rt::time;
use anyhow::Result;
use log::info;
use std::path::Path;
use std::time::Duration;

pub async fn run_schedule(pool: DbPool) -> Result<()> {
    let mut interval = time::interval(Duration::from_secs(60 * 60));
    info!("Starting timer");
    let library = crate::settings::get_config()?.get_string("library")?;
    let path = Path::new(library.as_str());
    loop {
        interval.tick().await;
        info!("Starting metadata scan");
        crate::metadata::scanner::scan(path, pool.clone())?;
        info!("Finished metadata scan");
    }
}
