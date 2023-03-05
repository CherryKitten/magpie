use crate::api::response_container::{MetaDataContainer, ResponseContainer};
use actix_files::NamedFile;
use actix_web::web::Json;
use actix_web::{get, web, Either, HttpResponse, Responder};
use std::collections::HashMap;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_index)
            .service(get_tracks)
            .service(get_track)
            .service(play_track)
            .service(get_artists)
            .service(get_artist)
            .service(get_albums)
            .service(get_album)
            .service(get_album_art),
    );
}

#[get("/")]
async fn get_index() -> impl Responder {
    let mut map = HashMap::new();

    map.insert(
        "Magpie",
        "Selfhosted Music streaming service wirtten in Rust",
    );
    map.insert("Version", "0.2.0");

    Json(map)
}

#[get("/tracks")]
pub async fn get_tracks(filter: web::Query<HashMap<String, String>>) -> HttpResponse {
    if let Ok(tracks) = crate::db::Track::get(filter.into_inner()) {
        let mut metadata: Vec<MetaDataContainer> = vec![];
        for a in tracks {
            metadata.push(MetaDataContainer::from(a));
        }
        HttpResponse::Ok().json(ResponseContainer::new(metadata))
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/tracks/{id}")]
pub async fn get_track(id: web::Path<i32>) -> HttpResponse {
    if let Ok(track) = crate::db::Track::get_by_id(*id) {
        let metadata: Vec<MetaDataContainer> = vec![MetaDataContainer::from(track)];
        HttpResponse::Ok().json(ResponseContainer::new(metadata))
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/tracks/{id}/play")]
pub async fn play_track(id: web::Path<i32>) -> Either<HttpResponse, NamedFile> {
    if let Ok(track) = crate::db::Track::get_by_id(*id) {
        if let Some(path) = track.path {
            let file = NamedFile::open_async(path).await;
            if let Ok(file) = file {
                Either::Right(file)
            } else {
                Either::Left(HttpResponse::InternalServerError().body("Error getting file."))
            }
        } else {
            Either::Left(HttpResponse::InternalServerError().body("Error getting file."))
        }
    } else {
        Either::Left(HttpResponse::NotFound().finish())
    }
}

#[get("/artists")]
pub async fn get_artists(filter: web::Query<HashMap<String, String>>) -> HttpResponse {
    if let Ok(artist) = crate::db::Artist::get(filter.into_inner()) {
        let mut metadata: Vec<MetaDataContainer> = vec![];
        for a in artist {
            metadata.push(MetaDataContainer::from(a));
        }
        HttpResponse::Ok().json(ResponseContainer::new(metadata))
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/artists/{id}")]
pub async fn get_artist(id: web::Path<i32>) -> HttpResponse {
    if let Ok(artist) = crate::db::Artist::get_by_id(*id) {
        let metadata: Vec<MetaDataContainer> = vec![MetaDataContainer::from(artist)];
        HttpResponse::Ok().json(ResponseContainer::new(metadata))
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/albums")]
pub async fn get_albums(filter: web::Query<HashMap<String, String>>) -> HttpResponse {
    if let Ok(albums) = crate::db::Album::get(filter.into_inner()) {
        let mut metadata: Vec<MetaDataContainer> = vec![];
        for a in albums {
            metadata.push(MetaDataContainer::from(a));
        }
        HttpResponse::Ok().json(ResponseContainer::new(metadata))
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/albums/{id}")]
pub async fn get_album(id: web::Path<i32>) -> HttpResponse {
    if let Ok(album) = crate::db::Album::get_by_id(*id) {
        let metadata: Vec<MetaDataContainer> = vec![MetaDataContainer::from(album)];
        HttpResponse::Ok().json(ResponseContainer::new(metadata))
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/albums/{id}/art")]
pub async fn get_album_art(id: web::Path<i32>) -> HttpResponse {
    if let Ok(album) = crate::db::Album::get_by_id(*id) {
        if let Some(art) = album.art {
            HttpResponse::Ok().body(art)
        } else {
            HttpResponse::NotFound().finish()
        }
    } else {
        HttpResponse::NotFound().finish()
    }
}

pub async fn get_library() -> HttpResponse {
    HttpResponse::Ok().json("todo")
}
