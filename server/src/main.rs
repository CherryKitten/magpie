use crate::metadata::scanner::do_scan;
use actix_web::rt::spawn;
use actix_web::rt::time;
use log::{error, info};
use std::io;
use std::time::Duration;

mod api;
mod config;
mod db;
mod metadata;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    let config = config::get_config();

    info!("Starting Webserver on {}:{}", config.host, config.port);

    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(60 * 60));
        info!("Started timer");
        loop {
            interval.tick().await;
            do_scan();
        }
    });

    match api::start_server(&config).await {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e)
        }
    }

    info!("Shutting down..");
    Ok(())
}
