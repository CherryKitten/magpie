mod responses;
mod routes;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use crate::config::AppConfig;
use actix_web::web::Json;
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use std::io;
use std::sync::Mutex;

use diesel::prelude::*;

use crate::db::establish_connection;

use crate::db::models::*;
use crate::metadata::{get_album_by_id, get_all_tracks, get_track_by_id};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

pub struct AppState {
    pub app_name: String,
    pub conn: Mutex<SqliteConnection>,
}



pub async fn start_server(config: &AppConfig) -> Result<(), io::Error> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("../test_data/key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("../test_data/cert.pem")
        .unwrap();

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
            .service(routes::play_track)
            .service(routes::get_albums)
            .service(routes::get_artists)
            .wrap(cors)
    })
    .bind_openssl((config.host.as_str(), config.port), builder)?
    .run()
    .await
    .expect("TODO: panic message");

    Ok(())
}
