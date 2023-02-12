mod routes;

use actix_cors::Cors;

use actix_web::{web, App, HttpServer, middleware::Logger};

use crate::config::AppConfig;
use anyhow::Result;
use diesel::SqliteConnection;


use std::sync::Mutex;

use crate::db::establish_connection;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

pub struct AppState {
    pub app_name: String,
    pub conn: Mutex<SqliteConnection>,
}

pub async fn start_server(config: &AppConfig) -> Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file("../test_data/key.pem", SslFiletype::PEM)?;
    builder.set_certificate_chain_file("../test_data/cert.pem")?;

    HttpServer::new(move || {
        let cors = Cors::permissive()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: "Magpie".to_string(),
                conn: (establish_connection().into()),
            }))
            .service(routes::index)
            .service(routes::get_tracks)
            .service(routes::get_track)
            .service(routes::get_albums)
            .service(routes::get_artists)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind_openssl((config.host.as_str(), config.port), builder)?
    .run()
    .await
    .expect("TODO: panic message");

    Ok(())
}
