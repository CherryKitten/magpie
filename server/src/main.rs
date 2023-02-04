use crate::scanner::do_scan;
use actix_web::rt::time;
use std::time::Duration;
use std::{io};
use actix_web::rt::spawn;


mod api;
mod config;
mod db;
mod metadata;
mod scanner;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let config = config::get_config();

    println!("Starting Webserver on {}:{}", config.host, config.port);

    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(60 * 60));
        println!("Started timer");
        loop {
            interval.tick().await;
            do_scan();
        }
    });

    api::start_server(&config).await.expect("");

    println!("{}", config.host);

    println!("Shutting down..");
    Ok(())
}
