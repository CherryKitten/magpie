use crate::db::models::*;
use actix_files::NamedFile;
use actix_web::web::Json;
use actix_web::{error, get, web, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    Json("todo")
}

#[get("/tracks")]
pub async fn get_tracks() -> Result<impl Responder, error::Error> {
    if let Ok(tracks) = Track::get(TrackFilters::All) {
        Ok(Json(tracks))
    } else {
        Err(error::ErrorInternalServerError("could not find any tracks"))
    }
}

#[get("/tracks/{id}")]
pub async fn get_track(id: web::Path<i32>) -> Result<impl Responder, error::Error> {
    if let Ok(track) = Track::get(TrackFilters::Id(*id)) {
        Ok(Json(track))
    } else {
        Err(error::ErrorNotFound("Track not found"))
    }
}

#[get("/tracks/{id}/play")]
pub async fn play_track(id: web::Path<i32>) -> Result<impl Responder, error::Error> {
    if let Ok(mut track) = Track::get(TrackFilters::Id(*id)) {
        if let Some(path) = &track.remove(0).path {
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
pub async fn get_albums() -> Result<impl Responder, error::Error> {
    if let Ok(albums) = Album::get(AlbumFilters::All) {
        Ok(Json(albums))
    } else {
        Err(error::ErrorInternalServerError("could not find any albums"))
    }
}

#[get("/albums/{id}")]
pub async fn get_album(id: web::Path<i32>) -> Result<impl Responder, error::Error> {
    if let Ok(album) = Album::get(AlbumFilters::Id(*id)) {
        Ok(Json(album))
    } else {
        Err(error::ErrorNotFound("Album not found"))
    }
}

#[get("/artists")]
pub async fn get_artists() -> Result<impl Responder, error::Error> {
    if let Ok(artists) = Artist::get(ArtistFilters::All) {
        Ok(Json(artists))
    } else {
        Err(error::ErrorInternalServerError(
            "could not find any artists",
        ))
    }
}

#[get("/artists/{id}")]
pub async fn get_artist(id: web::Path<i32>) -> Result<impl Responder, error::Error> {
    if let Ok(mut artist) = Artist::get(ArtistFilters::Id(*id)) {
        Ok(Json(artist.remove(0)))
    } else {
        Err(error::ErrorNotFound("Album not found"))
    }
}
