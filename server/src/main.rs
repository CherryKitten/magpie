use crate::scanner::{insert_found_tracks, traverse_dir};

use std::io;

mod api;
mod config;
mod db;
mod metadata;
mod scanner;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let config = config::get_config();

    let tracks = traverse_dir(&config.test_path).unwrap();
    insert_found_tracks(tracks);

    println!("Starting Webserver on {}:{}", config.host, config.port);

    api::start_server(&config).await.expect("");

    println!("{}", config.host);

    println!("Shutting down..");
    Ok(())
}
