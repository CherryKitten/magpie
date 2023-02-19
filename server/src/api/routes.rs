use crate::db::models::album::Album;
use crate::db::models::artist::Artist;
use crate::db::models::track::Track;
use actix_files::NamedFile;
use actix_web::web::Json;
use actix_web::{error, get, web, Responder};
use serde::Deserialize;
use strum_macros::EnumString;

#[derive(EnumString)]
enum ResponseType {
    Artist,
    Album,
    Track,
}

#[derive(Deserialize)]
pub struct Filters {
    limit: Option<i64>,
    title: Option<String>,
    name: Option<String>,
    year: Option<i32>,
    album: Option<i32>,
}

#[get("/")]
pub async fn index() -> impl Responder {
    Json("nya!")
}

#[get("/tracks")]
pub async fn get_tracks(filters: web::Query<Filters>) -> Result<impl Responder, error::Error> {
    let limit = filters.limit;
    if let Ok(tracks) = Track::get(
        None,
        filters.title.clone(),
        filters.album,
        filters.year,
        Some(limit.unwrap_or(50)),
        false,
    ) {
        Ok(Json(tracks))
    } else {
        Err(error::ErrorInternalServerError("could not find any tracks"))
    }
}

#[get("/tracks/{id}")]
pub async fn get_track(id: web::Path<i32>) -> Result<impl Responder, error::Error> {
    if let Ok(tracks) = Track::get(Some(*id), None, None, None, Some(1), false) {
        Ok(Json(tracks))
    } else {
        Err(error::ErrorInternalServerError("could not find track"))
    }
}

#[get("/tracks/{id}/play")]
pub async fn play_track(id: web::Path<i32>) -> Result<impl Responder, error::Error> {
    if let Ok(track) = Track::get(Some(*id), None, None, None, Some(1), true) {
        let track = Track::from(track.value());
        if let Some(path) = &track.path {
            Ok(NamedFile::open_async(path).await)
        } else {
            Err(error::ErrorInternalServerError(
                "Problem getting track file",
            ))
        }
    } else {
        Err(error::ErrorNotFound("Track not found"))
    }
}

#[get("/albums")]
pub async fn get_albums(filters: web::Query<Filters>) -> Result<impl Responder, error::Error> {
    if let Ok(albums) = Album::get(
        None,
        filters.title.clone(),
        filters.year,
        filters.limit,
        false,
    ) {
        Ok(Json(albums))
    } else {
        Err(error::ErrorInternalServerError("could not find any albums"))
    }
}

#[get("/albums/{id}")]
pub async fn get_album(id: web::Path<i32>) -> Result<impl Responder, error::Error> {
    if let Ok(albums) = Album::get(Some(*id), None, None, Some(1), false) {
        Ok(Json(albums))
    } else {
        Err(error::ErrorInternalServerError("could not find album"))
    }
}

#[get("/artists")]
pub async fn get_artists(filters: web::Query<Filters>) -> Result<impl Responder, error::Error> {
    if let Ok(artists) = Artist::get(None, filters.name.clone(), filters.limit, false) {
        Ok(Json(artists))
    } else {
        Err(error::ErrorInternalServerError(
            "could not find any artists",
        ))
    }
}

#[get("/artists/{id}")]
pub async fn get_artist(id: web::Path<i32>) -> Result<impl Responder, error::Error> {
    if let Ok(artist) = Artist::get(Some(*id), None, Some(1), false) {
        Ok(Json(artist))
    } else {
        Err(error::ErrorNotFound("Album not found"))
    }
}
