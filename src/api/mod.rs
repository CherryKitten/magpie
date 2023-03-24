use crate::db::DbPool;
use anyhow::{Context, Result};
use axum::routing::get;
use axum::{Json, Router};
use log::info;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use crate::api::response_container::ResponseContainer;
use serde::{Serialize, Deserialize};

pub mod response_container;
pub mod subsonic;
pub mod magpie;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub enum Response {
    MagpieResponse(ResponseContainer),
    SubsonicResponse(subsonic::SubsonicResponse)
}

pub async fn run(pool: DbPool) -> Result<()> {
    let config = super::settings::get_config()?;
    let _dev = config.get_bool("dev")?;
    let state = AppState { pool };

    let api = magpie::api_routes();
    let subsonic = subsonic::subsonic_compat_routes();
    let app = Router::new()
        .nest("/api", api)
        .nest("/rest", subsonic)
        .route("/", get(|| async { Json("Hello World") }))
        .with_state(state)
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
