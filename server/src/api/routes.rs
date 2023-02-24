use crate::db::models::album::{Album, AlbumFilter};
use crate::db::models::artist::{Artist, ArtistFilter};
use crate::db::models::track::{Track, TrackFilter};
use actix_files::NamedFile;
use actix_web::http::header::ContentType;
use actix_web::web::Json;
use actix_web::{error, get, web, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    Json("nya!")
}

#[get("/tracks")]
pub async fn get_tracks(filter: web::Query<TrackFilter>) -> Result<impl Responder, error::Error> {
    if let Ok(tracks) = Track::get(filter.into_inner(), false) {
        Ok(Json(tracks))
    } else {
        Err(error::ErrorInternalServerError("could not find any tracks"))
    }
}

#[get("/tracks/{id}")]
pub async fn get_track(id: web::Path<i32>) -> Result<impl Responder, error::Error> {
    if let Ok(tracks) = Track::get(
        TrackFilter {
            id: Some(*id),
            limit: Some(1),
            ..TrackFilter::default()
        },
        false,
    ) {
        Ok(Json(tracks))
    } else {
        Err(error::ErrorInternalServerError("could not find track"))
    }
}

#[get("/tracks/{id}/play")]
pub async fn play_track(id: web::Path<i32>) -> Result<impl Responder, error::Error> {
    if let Ok(track) = Track::get(
        TrackFilter {
            id: Some(*id),
            limit: Some(1),
            ..TrackFilter::default()
        },
        true,
    ) {
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
pub async fn get_albums(filter: web::Query<AlbumFilter>) -> Result<impl Responder, error::Error> {
    if let Ok(albums) = Album::get(filter.into_inner(), false) {
        Ok(Json(albums))
    } else {
        Err(error::ErrorInternalServerError("could not find any albums"))
    }
}

#[get("/albums/{id}")]
pub async fn get_album(id: web::Path<i32>) -> Result<impl Responder, error::Error> {
    if let Ok(albums) = Album::get(
        AlbumFilter {
            id: Some(*id),
            limit: Some(1),
            ..AlbumFilter::default()
        },
        false,
    ) {
        Ok(Json(albums))
    } else {
        Err(error::ErrorInternalServerError("could not find album"))
    }
}

#[get("/albums/{id}/art")]
pub async fn get_album_art(id: web::Path<i32>) -> Result<impl Responder, error::Error> {
    if let Ok(album) = Album::get(
        AlbumFilter {
            id: Some(*id),
            limit: Some(1),
            ..AlbumFilter::default()
        },
        true,
    ) {
        let album = Album::from(album.value());
        if let Some(picture) = album.art {
            Ok(HttpResponse::Ok()
                .content_type(ContentType::jpeg())
                .body(picture))
        } else {
            Err(error::ErrorInternalServerError(
                "Problem getting track file",
            ))
        }
    } else {
        Err(error::ErrorNotFound("Track not found"))
    }
}

#[get("/artists")]
pub async fn get_artists(filter: web::Query<ArtistFilter>) -> Result<impl Responder, error::Error> {
    if let Ok(artists) = Artist::get(filter.into_inner(), false) {
        Ok(Json(artists))
    } else {
        Err(error::ErrorInternalServerError(
            "could not find any artists",
        ))
    }
}

#[get("/artists/{id}")]
pub async fn get_artist(id: web::Path<i32>) -> Result<impl Responder, error::Error> {
    if let Ok(artist) = Artist::get(
        ArtistFilter {
            id: Some(*id),
            limit: Some(1),
            ..ArtistFilter::default()
        },
        false,
    ) {
        Ok(Json(artist))
    } else {
        Err(error::ErrorNotFound("Album not found"))
    }
}
