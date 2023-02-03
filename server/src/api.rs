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

struct AppState {
    app_name: String,
    conn: SqliteConnection,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
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

#[actix_web::main]
pub(crate) async fn main() -> std::io::Result<()> {
    let host = "localhost";
    let port = 8000;

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

    println!("Starting Webserver on {host}:{port}");
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
    .bind_openssl((host, port), builder)?
    .run()
    .await
}
