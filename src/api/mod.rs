use std::net::SocketAddr;

use anyhow::Context;
use axum::extract::State;
use axum::headers::authorization::Basic;
use axum::headers::Authorization;
use axum::http::{Method, Request};
use axum::middleware::{from_fn_with_state, Next};
use axum::response::Response;
use axum::routing::get;
use axum::{http, Router, TypedHeader};
use diesel::prelude::*;
use diesel::{Identifiable, QueryDsl, Queryable, Selectable, SqliteConnection};
use http::StatusCode;
use log::info;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::api::routes::*;
use crate::db::DbPool;
use crate::Result;

pub mod dto;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}

pub async fn run(pool: DbPool) -> Result<()> {
    let config = super::settings::get_config()?;
    let _dev = config.get_bool("dev")?;
    let state = AppState { pool };

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(tower_http::cors::Any);

    let app = Router::new()
        .route("/api/version", get(get_version))
        .route("/api/artists", get(get_artists))
        .route("/api/artists/:id", get(get_artist))
        .route("/api/albums", get(get_albums))
        .route("/api/albums/:id", get(get_album))
        .route("/api/tracks", get(get_tracks))
        .route("/api/tracks/:id", get(get_track))
        .route("/api/play/:id", get(play_track))
        .route("/api/art/:id", get(get_art))
        .route("/api/search/:query", get(unimplemented))
        .route_layer(ServiceBuilder::new().layer(from_fn_with_state(state.clone(), auth)))
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let (mut host, port) = (config.get_string("host")?, config.get_int("port")? as u16);
    if host == *"localhost" {
        host = "127.0.0.1".to_string()
    }
    info!("Starting API webserver on {}:{}", host, port);

    axum::Server::bind(&SocketAddr::new(host.parse()?, port))
        .serve(app.into_make_service())
        .await
        .context("Failed to bind webserver")?;

    Ok(())
}

async fn auth<B>(
    state: State<AppState>,
    auth: TypedHeader<Authorization<Basic>>,
    request: Request<B>,
    next: Next<B>,
) -> std::result::Result<Response, StatusCode>
where
    B: Send + 'static,
{
    let mut conn = state
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let None = User::check(auth.username(), auth.password(), &mut conn) {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        let response = next.run(request).await;

        Ok(response)
    }
}

#[derive(Selectable, Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = crate::db::schema::users)]
struct User {
    id: i32,
    username: String,
    password: String,
    email: Option<String>,
}

impl User {
    fn check(user: &str, pw: &str, conn: &mut SqliteConnection) -> Option<User> {
        use crate::db::schema::users::*;

        table
            .select(User::as_select())
            .filter(username.like(format!("%{user}%")))
            .filter(password.eq(pw))
            .first(conn)
            .ok()
    }
}
