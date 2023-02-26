use actix_web::rt::time;
use anyhow::Result;
use log::info;
use std::path::Path;
use std::time::Duration;

pub async fn run_schedule() -> Result<()> {
    let mut interval = time::interval(Duration::from_secs(60 * 60));
    info!("Started timer");
    let library = crate::settings::get_config()?.get_string("library")?;
    let path = Path::new(library.as_str());
    loop {
        interval.tick().await;
        crate::metadata::scanner::scan(path)?;
    }
}
