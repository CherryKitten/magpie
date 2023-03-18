use crate::api::response_container::{MetaDataContainer, ResponseContainer};
use crate::db;
use crate::db::DbPool;
use actix_files::NamedFile;
use actix_web::web::Json;
use actix_web::{get, web, Either, HttpResponse, Responder};
use anyhow::Error;
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
pub async fn get_tracks(
    filter: web::Query<HashMap<String, String>>,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = web::block(move || crate::db::Track::get(filter.into_inner(), &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Tracks")));
    match result {
        Ok(tracks) => {
            let mut metadata: Vec<MetaDataContainer> = vec![];
            for a in tracks {
                metadata.push(MetaDataContainer::from(a));
            }
            HttpResponse::Ok().json(ResponseContainer::new(metadata))
        }
        Err(error) => HttpResponse::NotFound().json(error.to_string()),
    }
}

#[get("/tracks/{id}")]
pub async fn get_track(id: web::Path<i32>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = web::block(move || crate::db::Track::get_by_id(*id, &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Tracks")));

    match result {
        Ok(track) => {
            let metadata: Vec<MetaDataContainer> = vec![MetaDataContainer::from(track)];
            HttpResponse::Ok().json(ResponseContainer::new(metadata))
        }
        Err(error) => HttpResponse::NotFound().json(error.to_string()),
    }
}

#[get("/tracks/{id}/play")]
pub async fn play_track(
    id: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Either<HttpResponse, NamedFile> {
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = web::block(move || db::Track::get_by_id(*id, &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Tracks")));

    match result {
        Ok(track) => match NamedFile::open_async(track.path.unwrap()).await {
            Ok(file) => Either::Right(file),
            Err(error) => Either::Left(HttpResponse::NotFound().json(error.to_string())),
        },
        Err(error) => Either::Left(HttpResponse::NotFound().json(error.to_string())),
    }
}

#[get("/artists")]
pub async fn get_artists(
    filter: web::Query<HashMap<String, String>>,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = web::block(move || crate::db::Artist::get(filter.into_inner(), &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Artists")));

    if let Ok(artist) = result {
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
pub async fn get_artist(id: web::Path<i32>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = web::block(move || crate::db::Artist::get_by_id(*id, &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Artists")));

    if let Ok(artist) = result {
        let metadata: Vec<MetaDataContainer> = vec![MetaDataContainer::from(artist)];
        HttpResponse::Ok().json(ResponseContainer::new(metadata))
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/albums")]
pub async fn get_albums(
    filter: web::Query<HashMap<String, String>>,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = web::block(move || crate::db::Album::get(filter.into_inner(), &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Albums")));

    if let Ok(albums) = result {
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
pub async fn get_album(id: web::Path<i32>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = web::block(move || crate::db::Album::get_by_id(*id, &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Albums")));

    if let Ok(album) = result {
        let metadata: Vec<MetaDataContainer> = vec![MetaDataContainer::from(album)];
        HttpResponse::Ok().json(ResponseContainer::new(metadata))
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/albums/{id}/art")]
pub async fn get_album_art(id: web::Path<i32>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = web::block(move || crate::db::Album::get_by_id(*id, &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Albums")));

    if let Ok(album) = result {
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
