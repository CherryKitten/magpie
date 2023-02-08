use crate::db::establish_connection;
use crate::db::models::{Album, Artist, Track};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::*;

pub mod scanner;

pub fn vectorize_tags<'a>(tags: impl Iterator<Item = &'a str> + Sized) -> Vec<String> {
    let mut temp_vec = vec![];
    for tag in tags {
        temp_vec.push(tag.to_string());
    }
    temp_vec
}

pub fn get_all_tracks() -> Vec<Track> {
    let mut conn = establish_connection();

    Track::all().load::<Track>(&mut conn).expect("")
}

pub fn get_all_albums() -> Vec<Album> {
    let mut conn = establish_connection();

    Album::all().load::<Album>(&mut conn).expect("")
}

pub fn get_all_artists() -> Vec<Artist> {
    let mut conn = establish_connection();
    Artist::all().load::<Artist>(&mut conn).expect("")
}

pub fn get_album_by_id(id: i32) -> Option<Album> {
    let mut conn = establish_connection();

    match albums::table.find(id).first(&mut conn) {
        Ok(album) => Some(album),
        Err(..) => None,
    }
}

pub fn get_artist_by_id(id: i32) -> Option<Artist> {
    let mut conn = establish_connection();

    match artists::table.find(id).first(&mut conn) {
        Ok(artist) => Some(artist),
        Err(..) => None,
    }
}

pub fn get_track_by_id(id: i32) -> Option<Track> {
    let mut conn = establish_connection();

    match tracks::table.find(id).first(&mut conn) {
        Ok(track) => Some(track),
        Err(..) => None,
    }
}
