use crate::db::establish_connection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::*;

pub mod scanner;

pub(crate) fn vectorize_tags<'a>(tags: impl Iterator<Item = &'a str> + Sized) -> Vec<String> {
    let mut temp_vec = vec![];
    for tag in tags {
        temp_vec.push(tag.to_string());
    }
    temp_vec
}

pub fn get_all_tracks() -> Vec<Track> {
    let mut conn = establish_connection();

    tracks::table
        .select(tracks::all_columns)
        .load::<Track>(&mut conn)
        .expect("")
}

pub fn get_all_albums() -> Vec<Album> {
    let mut conn = establish_connection();

    albums::table
        .select(albums::all_columns)
        .load::<Album>(&mut conn)
        .expect("")
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

#[derive(Debug, PartialEq, Eq, Queryable, QueryableByName, Serialize, Deserialize)]
#[diesel(table_name = tracks)]
pub struct Track {
    pub(crate) id: i32,
    pub(crate) album: Option<i32>,
    pub(crate) path: Option<String>,
    pub(crate) track_number: Option<i32>,
    pub(crate) disc_number: Option<i32>,
    pub(crate) title: Option<String>,
    pub(crate) year: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Queryable, QueryableByName, Serialize, Deserialize)]
#[diesel(table_name = albums)]
pub struct Album {
    id: i32,
    year: Option<i32>,
    pub(crate) title: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Queryable, QueryableByName, Serialize, Deserialize)]
#[diesel(table_name = artists)]
pub struct Artist {
    id: i32,
    name: Option<String>,
}
