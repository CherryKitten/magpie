use crate::db::models::*;
use actix_files::NamedFile;
use actix_web::web::Json;
use actix_web::{error, get, web, Responder};
use log::info;

#[get("/")]
pub async fn index() -> impl Responder {
    Json("todo")
}

#[get("/tracks")]
pub async fn get_tracks() -> Result<impl Responder, error::Error> {
    info!("GET /tracks");
    if let Ok(tracks) = Track::all() {
        Ok(Json(tracks))
    } else {
        Err(error::ErrorInternalServerError("could not find any tracks"))
    }
}

#[get("/tracks/{id}")]
pub async fn get_track(id: web::Path<i32>) -> Result<impl Responder, error::Error> {
    info!("GET /tracks/{id}");
    if let Ok(track) = Track::by_id(*id) {
        let path = match track.path {
            None => "".to_string(),
            Some(path) => path,
        };

        Ok(NamedFile::open_async(path).await)
    } else {
        Err(error::ErrorNotFound("Track not found"))
    }
}

#[get("/albums")]
pub async fn get_albums() -> Result<impl Responder, error::Error> {
    info!("GET /albums");
    if let Ok(albums) = Album::all() {
        Ok(Json(albums))
    } else {
        Err(error::ErrorInternalServerError("could not find any albums"))
    }
}

#[get("/artists")]
pub async fn get_artists() -> Result<impl Responder, error::Error> {
    info!("GET /artists");
    if let Ok(artists) = Artist::all() {
        Ok(Json(artists))
    } else {
        Err(error::ErrorInternalServerError(
            "could not find any artists",
        ))
    }
}
