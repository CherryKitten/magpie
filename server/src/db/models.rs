use super::schema::*;
use crate::db::establish_connection;
use anyhow::Result;

use diesel::prelude::*;

use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use serde::Serialize as SerializeDerive;

#[derive(
    Debug,
    PartialEq,
    Eq,
    Selectable,
    Queryable,
    QueryableByName,
    Identifiable,
    Associations,
    AsChangeset,
    Deserialize,
)]
#[diesel(belongs_to(Album))]
#[diesel(table_name = tracks)]
pub struct Track {
    pub id: i32,
    pub album_id: Option<i32>,
    pub path: Option<String>,
    pub track_number: Option<i32>,
    pub disc_number: Option<i32>,
    pub title: Option<String>,
    pub year: Option<i32>,
}

impl Serialize for Track {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Track", 7)?;

        let album = self.get_album().unwrap();

        state.serialize_field("id", &self.id)?;
        state.serialize_field("album_id", &self.album_id)?;
        state.serialize_field("album", &album.title.unwrap_or("".to_string()))?;
        state.serialize_field("track_number", &self.track_number)?;
        state.serialize_field("disc_number", &self.disc_number)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("year", &self.year)?;
        state.end()
    }
}

impl Track {
    pub fn all() -> Result<Vec<Track>> {
        let mut conn = establish_connection();

        Ok(tracks::table.load::<Track>(&mut conn)?)
    }
    pub fn by_id(id: i32) -> Result<Track> {
        let mut conn = establish_connection();

        Ok(tracks::table.find(id).first(&mut conn)?)
    }
    pub fn get_album(&self) -> Result<Album> {
        let album_id = self.album_id.unwrap_or_default();

        Ok(Album::by_id(album_id)?)
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Queryable,
    QueryableByName,
    Identifiable,
    AsChangeset,
    Selectable,
    SerializeDerive,
    Deserialize,
)]
#[diesel(table_name = albums)]
pub struct Album {
    id: i32,
    year: Option<i32>,
    pub title: Option<String>,
}

impl Album {
    pub fn all() -> Result<Vec<Album>> {
        let mut conn = establish_connection();

        Ok(albums::table.load::<Album>(&mut conn)?)
    }
    pub fn by_id(id: i32) -> Result<Album> {
        let mut conn = establish_connection();

        Ok(albums::table
            .select(Album::as_select())
            .find(id)
            .first::<Album>(&mut conn)?)
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Selectable,
    Queryable,
    QueryableByName,
    Identifiable,
    AsChangeset,
    SerializeDerive,
    Deserialize,
)]
#[diesel(table_name = artists)]
pub struct Artist {
    id: i32,
    name: Option<String>,
}

impl Artist {
    pub fn all() -> Result<Vec<Artist>> {
        let mut conn = establish_connection();

        Ok(artists::table.load::<Artist>(&mut conn)?)
    }
    pub fn by_id(id: i32) -> Result<Artist> {
        let mut conn = establish_connection();

        Ok(artists::table.find(id).first(&mut conn)?)
    }
}

#[derive(
    Debug, PartialEq, Eq, Queryable, Associations, Identifiable, SerializeDerive, Deserialize,
)]
#[diesel(table_name = album_artists)]
#[diesel(belongs_to(Album))]
#[diesel(belongs_to(Artist))]
pub struct AlbumArtist {
    id: i32,
    album_id: Option<i32>,
    artist_id: Option<i32>,
}

#[derive(
    Debug, PartialEq, Eq, Queryable, Identifiable, Associations, SerializeDerive, Deserialize,
)]
#[diesel(belongs_to(Track))]
#[diesel(belongs_to(Artist))]
#[diesel(table_name = track_artists)]
pub struct TrackArtist {
    id: i32,
    track_id: Option<i32>,
    artist_id: Option<i32>,
}
