use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Responder};

use crate::config::AppConfig;
use actix_web::web::Json;
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use std::io;
use std::sync::Mutex;

use crate::db::establish_connection;

use crate::metadata::{get_album_by_id, get_all_tracks, get_track_by_id, Track};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

pub(crate) struct AppState {
    pub(crate) app_name: String,
    pub(crate) conn: Mutex<SqliteConnection>,
}

#[get("/")]
async fn index() -> impl Responder {
    Json("todo")
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TrackResponse {
    id: i32,
    albumid: Option<i32>,
    album: Option<String>,
    track_number: Option<i32>,
    disc_number: Option<i32>,
    title: Option<String>,
    year: Option<i32>,
}

impl TrackResponse {
    fn from_track(track: Track) -> Self {
        TrackResponse {
            id: track.id,
            albumid: track.album,
            album: match get_album_by_id(track.album.unwrap_or(0)) {
                Some(a) => a.title,
                None => None,
            },
            track_number: track.track_number,
            disc_number: track.disc_number,
            title: track.title,
            year: track.year,
        }
    }
}

#[get("/tracks")]
async fn get_tracks(_data: web::Data<AppState>) -> impl Responder {
    let mut response = vec![];

    for track in get_all_tracks() {
        response.push(TrackResponse::from_track(track));
    }
    Json(response)
}

#[get("/tracks/{id}")]
async fn play_track(id: web::Path<i32>) -> impl Responder {
    let track = get_track_by_id(id.into_inner()).unwrap();
    NamedFile::open_async(track.path.unwrap()).await
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
                conn: (establish_connection().into()),
            }))
            .service(hello)
            .service(musictest)
            .service(index)
            .service(get_tracks)
            .service(play_track)
            .wrap(cors)
    })
    .bind_openssl((config.host.as_str(), config.port), builder)?
    .run()
    .await
    .expect("TODO: panic message");

    Ok(())
}
