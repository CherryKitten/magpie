use actix_files::NamedFile;
use actix_web::{get, Responder, web};
use actix_web::web::Json;
use crate::db::models::*;
use diesel::prelude::*;
use crate::api::AppState;

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    Json("todo")
}

#[get("/tracks")]
pub(crate) async fn get_tracks(data: web::Data<AppState>) -> impl Responder {
    let mut conn = data.conn.lock().unwrap();

    Json(Track::all().load::<Track>(&mut *conn).expect(""))
}

#[get("/tracks/{id}")]
pub(crate) async fn play_track(id: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let mut conn = data.conn.lock().unwrap();

    NamedFile::open_async(Track::by_id(*id).first(&mut *conn).expect("").path.unwrap()).await
}

#[get("/albums")]
pub(crate) async fn get_albums(data: web::Data<AppState>) -> impl Responder {
    let mut conn = data.conn.lock().unwrap();

    Json(Album::all().load::<Album>(&mut *conn).expect(""))
}

#[get("/artists")]
pub(crate) async fn get_artists(data: web::Data<AppState>) -> impl Responder {
    let mut conn = data.conn.lock().unwrap();

    Json(Artist::all().load::<Artist>(&mut *conn).expect(""))
}
