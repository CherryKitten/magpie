use super::schema::*;
use diesel::dsl::AsSelect;
use diesel::dsl::SqlTypeOf;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};

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
    Serialize,
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

type SqlType<T> = SqlTypeOf<AsSelect<T, Sqlite>>;

type TrackQuery<'a> = tracks::BoxedQuery<'a, Sqlite, SqlType<Track>>;
type AlbumQuery<'a> = albums::BoxedQuery<'a, Sqlite, SqlType<Album>>;
type ArtistQuery<'a> = artists::BoxedQuery<'a, Sqlite, SqlType<Artist>>;

impl Track {
    pub fn all() -> TrackQuery<'static> {
        tracks::table.select(Track::as_select()).into_boxed()
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
    Serialize,
    Deserialize,
)]
#[diesel(table_name = albums)]
pub struct Album {
    id: i32,
    year: Option<i32>,
    pub title: Option<String>,
}

impl Album {
    pub fn all() -> AlbumQuery<'static> {
        albums::table.select(Album::as_select()).into_boxed()
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
    Serialize,
    Deserialize,
)]
#[diesel(table_name = artists)]
pub struct Artist {
    id: i32,
    name: Option<String>,
}

impl Artist {
    pub fn all() -> ArtistQuery<'static> {
        artists::table.select(Artist::as_select()).into_boxed()
    }
}

#[derive(Debug, PartialEq, Eq, Queryable, Associations, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = album_artists)]
#[diesel(belongs_to(Album))]
#[diesel(belongs_to(Artist))]
pub struct AlbumArtist {
    id: i32,
    album_id: Option<i32>,
    artist_id: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Track))]
#[diesel(belongs_to(Artist))]
#[diesel(table_name = track_artists)]
pub struct TrackArtist {
    id: i32,
    track_id: Option<i32>,
    artist_id: Option<i32>,
}
