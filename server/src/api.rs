use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Responder};

use std::io;

use crate::config::AppConfig;
use crate::db;
use actix_web::web::Json;
use diesel::SqliteConnection;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

pub(crate) struct AppState {
    pub(crate) app_name: String,
    pub(crate) conn: SqliteConnection,
}

#[get("/")]
async fn index() -> impl Responder {
    Json("todo")
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {} from {}!", &name, &app_name)
}

//#[get("/test/{path}")]
#[get("/test")]
async fn musictest() -> impl Responder {
    NamedFile::open_async("../../test_data/music/Bring Me The Horizon/Bring Me The Horizon - 2022 - sTraNgeRs/01. sTraNgeRs.flac").await
    //NamedFile::open_async(path.to_string()).await
}

pub(crate) async fn start_server(config: &AppConfig) -> Result<(), io::Error> {
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
                conn: (db::establish_connection()),
            }))
            .service(hello)
            .service(musictest)
            .service(index)
            .wrap(cors)
    })
    .bind_openssl((config.host.as_str(), config.port), builder)?
    .run()
    .await
    .expect("TODO: panic message");

    Ok(())
}
