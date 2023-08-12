use std::net::SocketAddr;

use anyhow::Context;
use axum::http::Method;
use axum::routing::get;
use axum::{Json, Router};
use log::info;
use tower_http::trace::TraceLayer;

use crate::api::routes::api_routes;
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

    let api = api_routes();
    let app = Router::new()
        .nest("/api", api)
        .route("/", get(|| async { Json("Hello World") }))
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
