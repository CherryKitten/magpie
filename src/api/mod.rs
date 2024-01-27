use crate::settings::Settings;
use axum::{extract::State, response::IntoResponse, routing::get, Router};

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub(crate) config: Settings,
}

async fn hello_world(State(state): State<AppState>) -> impl IntoResponse {
    dbg!(state.config);
    "Hello, World!"
}

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/", get(hello_world))
}
