use crate::db::schema::*;
use crate::metadata::*;
use diesel::prelude::*;

use super::dto::*;
use crate::api::AppState;
use crate::Result;
use anyhow::Context;
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use duplicate::duplicate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Filter {
    limit: Option<i64>,
    offset: Option<i64>,
    name: Option<String>,
    title: Option<String>,
    subtitle: Option<String>,
    album: Option<String>,
    year: Option<String>,
    bpm: Option<String>,
    language: Option<String>,
}

macro_rules! db_conn {
    ($state:ident) => {
        $state
            .pool
            .get()
            .context("Could not get database connection")?
    };
}

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/version", get(get_version))
        .route("/artists", get(get_artists))
        .route("/artists/:id", get(get_artist))
        .route("/albums", get(get_albums))
        .route("/albums/:id", get(get_album))
        .route("/tracks", get(get_tracks))
        .route("/tracks/:id", get(get_track))
        .route("/play/:id", get(unimplemented))
        .route("/art/:id", get(unimplemented))
        .route("/search/:query", get(unimplemented))
}

async fn unimplemented() -> Json<&'static str> {
    Json("Not Implemented yet, sorry.")
}

async fn get_version() -> Json<Version> {
    Json(Version::default())
}

async fn get_artists(
    Query(filter): Query<Filter>,
    State(state): State<AppState>,
) -> Result<Json<MagpieResponse>> {
    let mut conn = db_conn!(state);
    let mut select = artists::table.select(Artist::as_select()).into_boxed();

    let limit = filter.limit.unwrap_or(50);
    let offset = filter.offset.unwrap_or(0);

    duplicate! {
        [
            key statement;
            [ title ] [ artists::name.like(format!("%{item}%")) ];
            [ name ]  [ artists::name.like(format!("%{item}%")) ];
        ]
        if let Some(item) = filter.key {
        select = select.filter(statement);
    }}

    select = select
        .limit(limit)
        .offset(offset)
        .distinct()
        .order_by(artists::name);

    let result: Vec<Artist> = select.load(&mut conn)?;

    if result.is_empty() {
        return Ok(Json(MagpieResponse::new().error("No Artists found")));
    }

    let result = result
        .into_iter()
        .map(|v| MagpieArtist::new(v).unwrap())
        .collect();

    let result = MagpieData::Artists(result);

    let response = MagpieResponse::new()
        .add_data(result)
        .set_pagination(limit, offset);

    Ok(Json(response))
}

async fn get_artist(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<MagpieResponse>> {
    let mut conn = db_conn!(state);

    let result = artists::table
        .select(Artist::as_select())
        .find(id)
        .first::<Artist>(&mut conn);

    let mut response = match result {
        Ok(result) => {
            let result = MagpieArtist::new(result)?;
            MagpieResponse::new().add_data(MagpieData::Artist(result))
        }
        Err(e) => MagpieResponse::new().error(&e.to_string()),
    };

    if response.status == MagpieStatus::Ok {
        let children = Album::get_by_artist_id(id, &mut conn)?
            .into_iter()
            .map(|v| MagpieAlbum::new(v, &mut conn).unwrap())
            .collect();

        let children = MagpieData::Albums(children);

        response = response.add_children(children);
    }

    Ok(Json(response))
}

async fn get_albums(
    Query(filter): Query<Filter>,
    State(state): State<AppState>,
) -> Result<Json<MagpieResponse>> {
    let mut conn = db_conn!(state);
    let mut select = albums::table.select(Album::as_select()).into_boxed();

    let limit = filter.limit.unwrap_or(50);
    let offset = filter.offset.unwrap_or(0);

    duplicate! {
        [
            key statement;
            [ title ] [ albums::title.like(format!("%{item}%")) ];
            [ year ]  [ albums::year.eq((item.parse::<i32>()?)) ];
        ]
        if let Some(item) = filter.key {
        select = select.filter(statement);
    }}

    select = select
        .limit(limit)
        .offset(offset)
        .distinct()
        .order_by(albums::year);

    let result: Vec<Album> = select.load(&mut conn)?;

    if result.is_empty() {
        return Ok(Json(MagpieResponse::new().error("No Albums found")));
    }

    let result = result
        .into_iter()
        .map(|v| MagpieAlbum::new(v, &mut conn).unwrap())
        .collect();

    let result = MagpieData::Albums(result);

    let response = MagpieResponse::new()
        .add_data(result)
        .set_pagination(limit, offset);

    Ok(Json(response))
}

async fn get_album(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<MagpieResponse>> {
    let mut conn = db_conn!(state);

    let result = albums::table
        .select(Album::as_select())
        .find(id)
        .first::<Album>(&mut conn);

    let mut response = match result {
        Ok(result) => {
            let result = MagpieAlbum::new(result, &mut conn)?;
            MagpieResponse::new().add_data(MagpieData::Album(result))
        }
        Err(e) => MagpieResponse::new().error(&e.to_string()),
    };

    if response.status == MagpieStatus::Ok {
        let children = Track::get_by_album_id(id, &mut conn)?
            .into_iter()
            .map(|v| MagpieTrack::new(v, &mut conn).unwrap())
            .collect();

        let children = MagpieData::Tracks(children);

        response = response.add_children(children);
    }

    Ok(Json(response))
}

async fn get_tracks(
    Query(filter): Query<Filter>,
    State(state): State<AppState>,
) -> Result<Json<MagpieResponse>> {
    let mut conn = db_conn!(state);
    let mut select = tracks::table.select(Track::as_select()).into_boxed();

    let limit = filter.limit.unwrap_or(50);
    let offset = filter.offset.unwrap_or(0);

    duplicate! {
        [
            key statement;
            [ title ]         [ tracks::title.like(format!("%{item}%")) ];
            [ subtitle ]      [ tracks::subtitle.like(format!("%{item}%")) ];
            [ album ]         [ tracks::album_id.eq(Album::get_by_title(&item, &mut conn)?.id) ];
            [ year ]          [ tracks::year.eq((item.parse::<i32>()?)) ];
            [ bpm ]           [ tracks::bpm.eq(item) ];
            [ language ]      [ tracks::language.eq(item) ];
        ]
        if let Some(item) = filter.key {
        select = select.filter(statement);
    }}

    select = select
        .limit(limit)
        .offset(offset)
        .distinct()
        .order_by(tracks::disc_number)
        .then_order_by(tracks::track_number);

    let result: Vec<Track> = select.load(&mut conn)?;

    if result.is_empty() {
        return Ok(Json(MagpieResponse::new().error("No Tracks found")));
    }

    let result = result
        .into_iter()
        .map(|v| MagpieTrack::new(v, &mut conn).unwrap())
        .collect();

    let result = MagpieData::Tracks(result);

    let response = MagpieResponse::new()
        .add_data(result)
        .set_pagination(limit, offset);

    Ok(Json(response))
}

async fn get_track(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<MagpieResponse>> {
    let mut conn = db_conn!(state);

    let result = tracks::table
        .select(Track::as_select())
        .find(id)
        .first::<Track>(&mut conn);

    let response = match result {
        Ok(result) => {
            let result = MagpieTrack::new(result, &mut conn)?;
            MagpieResponse::new().add_data(MagpieData::Track(result))
        }
        Err(e) => MagpieResponse::new().error(&e.to_string()),
    };

    Ok(Json(response))
}
