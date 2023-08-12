use std::path::Path;
use std::time::Duration;

use log::{error, info};
use tokio::time;

use crate::db::DbPool;
use crate::Result;

pub async fn run_schedule(pool: DbPool) -> Result<()> {
    let mut interval = time::interval(Duration::from_secs(60 * 60));
    info!("Starting timer");
    let library = crate::settings::get_config()?.get_string("library")?;
    let path = Path::new(library.as_str());
    loop {
        interval.tick().await;
        info!("Starting metadata scan");
        match crate::metadata::scanner::scan(path, pool.clone()) {
            Ok(_) => {}
            Err(error) => {
                error!("{error}")
            }
        }
        info!("Finished metadata scan");
    }
}
