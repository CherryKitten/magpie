#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};

static BASE_URL: &str = "http://localhost:8080/api/";

fn main() {
    dioxus_logger::init(log::LevelFilter::Trace).expect("failed to init logger");
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    render! {
        div { id: "app", Router::<Route> {} }
    }
}

pub fn Home(cx: Scope) -> Element {
    render! {"Hello World"}
}

pub fn Albums(cx: Scope) -> Element {
    let albums = use_future(cx, (), |_| get_albums());

    match albums.value() {
        Some(Ok(albums)) => {
            render! {
                ul {
                    for album in albums {
                        li {
                        "{album.artist[0]} - {album.title}"
                        }
                    }
                }
            }
        }
        Some(Err(err)) => {
            render! {"{err}"}
        }
        None => {
            render! {"Loading..."}
        }
    }
}

pub fn Navigation(cx: Scope) -> Element {
    let url = gloo::utils::document().url().ok()?;
    render! { div { margin_bottom: "2rem", id: "nav", "Here be navigation, the url is {url}" } }
}

pub fn Wrapper(cx: Scope) -> Element {
    render! {
        Navigation {}

        div { id: "content", margin: "2rem", Outlet::<Route> {} }

        Player {}
    }
}

pub fn Player(cx: Scope) -> Element {
    render! { div { id: "player", "Here will be player" } }
}
#[derive(Routable, Clone)]
enum Route {
    #[layout(Wrapper)]
    #[nest("/web")]
    #[route("/")]
    Home {},
    #[route("/albums")]
    Albums {},
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Album {
    id: usize,
    title: String,
    year: usize,
    artist: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Artist {
    name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
struct MagpieResponse {
    status: String,
    count: usize,
    page: usize,
    data: Vec<Album>,
}
pub async fn get_albums() -> Result<Vec<Album>, reqwest::Error> {
    let url = format!("{BASE_URL}albums");

    let response: MagpieResponse = reqwest::get(&url).await?.json().await?;
    let albums = response.data;

    Ok(albums)
}
