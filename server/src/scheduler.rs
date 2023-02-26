use actix_web::rt::time;
use log::{info, warn};
use std::time::Duration;

pub async fn run_schedule() {
    let mut interval = time::interval(Duration::from_secs(60 * 60));
    info!("Started timer");
    loop {
        interval.tick().await;
        warn!("Not implemented")
    }
}
