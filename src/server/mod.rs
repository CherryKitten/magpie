use axum::{extract::State, response::IntoResponse, routing::get, Router};
use color_eyre::Result;
#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub(crate) config: super::config::Config,
    pub(crate) database_pool: super::database::DbPool,
}

pub(crate) async fn run(config: crate::config::Config) -> Result<()> {
    let database_pool = crate::database::get_connection_pool(config.database_url.clone())?;
    let state = AppState {
        config,
        database_pool,
    };
    let app = Router::new().with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
