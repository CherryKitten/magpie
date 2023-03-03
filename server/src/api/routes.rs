use crate::api::response_container::{MetaDataContainer, ResponseContainer};
use actix_web::web::Json;
use actix_web::{get, web, HttpResponse, Responder};
use std::collections::HashMap;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_index)
            .service(get_track)
            .service(get_artists),
    );
}

#[get("/")]
async fn get_index() -> impl Responder {
    let mut map = HashMap::new();

    map.insert("test", "auch test");

    Json(map)
}

#[get("/tracks/{id}")]
pub async fn get_track(id: web::Path<i32>) -> HttpResponse {
    if let Ok(track) = crate::db::Track::get_by_id(*id) {
        HttpResponse::Ok().json(MetaDataContainer::from(track))
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/artists")]
pub async fn get_artists() -> HttpResponse {
    if let Ok(artist) = crate::db::Artist::all() {
        let mut metadata: Vec<MetaDataContainer> = vec![];
        for a in artist {
            metadata.push(MetaDataContainer::from(a));
        }
        HttpResponse::Ok().json(ResponseContainer::new(metadata))
    } else {
        HttpResponse::NotFound().finish()
    }
}

pub async fn get_library() -> HttpResponse {
    HttpResponse::Ok().json("todo")
}
