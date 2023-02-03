use std::io;
use std::path::Path;
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::scanner::traverse_dir;

mod metadata;
mod scanner;
mod api;
mod db;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::get_config();

    let test_path = Path::new("../test_data/music");

    let mut tracks = vec![];

    println!("Hello, {}!", test_path.display());

    tracks.append(&mut traverse_dir(test_path).unwrap());

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("../test_data/key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("../test_data/cert.pem")
        .unwrap();

    println!("Starting Webserver on {}:{}", config.host, config.port);
    HttpServer::new(move || {
        let cors = Cors::permissive()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        App::new()
            .app_data(web::Data::new(crate::api::AppState {
                app_name: "Magpie".to_string(),
                conn: (db::establish_connection()),
            }))
            .service(crate::api::hello)
            .service(crate::api::musictest)
            .service(crate::api::index)
            .wrap(cors)
    })
        .bind_openssl((config.host, config.port), builder)?
        .run()
        .await.expect("TODO: panic message");

    println!("Shutting down..");
    Ok(())
}



