use actix_cors::Cors;
use std::path::Path;

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Responder};

use crate::db;
use crate::metadata::Track;
use crate::scanner::traverse_dir;
use actix_web::web::Json;
use diesel::SqliteConnection;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

pub(crate) struct AppState {
    pub(crate) app_name: String,
    pub(crate) conn: SqliteConnection,
}

#[get("/")]
pub(crate) async fn index(data: web::Data<AppState>) -> impl Responder {
    Json("todo")
}

#[get("/hello/{name}")]
pub(crate) async fn hello(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {} from {}!", &name, &app_name)
}

//#[get("/test/{path}")]
#[get("/test")]
pub(crate) async fn musictest() -> impl Responder {
    NamedFile::open_async("../../test_data/music/Bring Me The Horizon/Bring Me The Horizon - 2022 - sTraNgeRs/01. sTraNgeRs.flac").await
    //NamedFile::open_async(path.to_string()).await
}
