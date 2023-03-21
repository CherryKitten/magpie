use crate::api::response_container::{MetaDataContainer, ResponseContainer};
use crate::db;

use crate::api::AppState;
use anyhow::Error;
use axum::{
    body::StreamBody,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use std::collections::HashMap;
use tokio::task::spawn_blocking;
use tokio_util::io::ReaderStream;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_index))
        .route("/tracks", get(get_tracks))
        .route("/tracks/:id", get(get_track))
        .route("/tracks/:id/play", get(play_track))
        .route("/artists", get(get_artists))
        .route("/artists/:id", get(get_artist))
        .route("/albums", get(get_albums))
        .route("/albums/:id", get(get_album))
        .route("/albums/:id/art", get(get_album_art))
}

async fn get_index() -> Response {
    let mut map = HashMap::new();

    map.insert(
        "Magpie",
        "Selfhosted Music streaming service wirtten in Rust",
    );
    map.insert("Version", "0.2.0");

    Json(map).into_response()
}

pub async fn get_tracks(
    Query(filter): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Response {
    let pool = state.pool;
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = db::Track::get(filter, &mut conn);
    match result {
        Ok(tracks) => {
            let mut metadata: Vec<MetaDataContainer> = vec![];
            for a in tracks {
                metadata.push(MetaDataContainer::from(a));
            }
            Json(ResponseContainer::new(metadata)).into_response()
        }
        Err(error) => (StatusCode::NOT_FOUND, error.to_string()).into_response(),
    }
}

pub async fn get_track(Path(id): Path<i32>, State(state): State<AppState>) -> Response {
    let pool = state.pool;
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = spawn_blocking(move || db::Track::get_by_id(id, &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Tracks")));

    match result {
        Ok(track) => {
            let metadata: Vec<MetaDataContainer> = vec![MetaDataContainer::from(track)];
            Json(ResponseContainer::new(metadata)).into_response()
        }
        Err(error) => (StatusCode::NOT_FOUND, error.to_string()).into_response(),
    }
}

pub async fn play_track(Path(id): Path<i32>, State(state): State<AppState>) -> Response {
    let pool = state.pool;
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = spawn_blocking(move || db::Track::get_by_id(id, &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Tracks")));

    match result {
        Ok(track) => match tokio::fs::File::open(track.path.unwrap()).await {
            Ok(file) => {
                let stream = ReaderStream::new(file);
                let body = StreamBody::new(stream);
                body.into_response()
            }
            Err(error) => (StatusCode::NOT_FOUND, error.to_string()).into_response(),
        },
        Err(error) => (StatusCode::NOT_FOUND, error.to_string()).into_response(),
    }
}

pub async fn get_artists(
    Query(filter): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Response {
    let pool = state.pool;
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = spawn_blocking(move || db::Artist::get(filter, &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Artists")));

    if let Ok(artist) = result {
        let mut metadata: Vec<MetaDataContainer> = vec![];
        for a in artist {
            metadata.push(MetaDataContainer::from(a));
        }
        Json(ResponseContainer::new(metadata)).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn get_artist(Path(id): Path<i32>, State(state): State<AppState>) -> Response {
    let pool = state.pool;
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = spawn_blocking(move || db::Artist::get_by_id(id, &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Artists")));

    if let Ok(artist) = result {
        let metadata: Vec<MetaDataContainer> = vec![MetaDataContainer::from(artist)];
        Json(ResponseContainer::new(metadata)).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn get_albums(
    Query(filter): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Response {
    let pool = state.pool;
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = spawn_blocking(move || db::Album::get(filter, &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Albums")));

    if let Ok(albums) = result {
        let mut metadata: Vec<MetaDataContainer> = vec![];
        for a in albums {
            metadata.push(MetaDataContainer::from(a));
        }
        Json(ResponseContainer::new(metadata)).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn get_album(Path(id): Path<i32>, State(state): State<AppState>) -> Response {
    let pool = state.pool;
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = spawn_blocking(move || db::Album::get_by_id(id, &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Albums")));

    if let Ok(album) = result {
        let metadata: Vec<MetaDataContainer> = vec![MetaDataContainer::from(album)];
        Json(ResponseContainer::new(metadata)).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn get_album_art(Path(id): Path<i32>, State(state): State<AppState>) -> Response {
    let pool = state.pool;
    let mut conn = pool.get().expect("Failed to get database connection");

    let result = spawn_blocking(move || db::Album::get_by_id(id, &mut conn))
        .await
        .unwrap_or(Err(Error::msg("Failed to get Albums")));

    if let Ok(album) = result {
        if let Some(art) = album.art {
            art.into_response()
        } else {
            StatusCode::NOT_FOUND.into_response()
        }
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
