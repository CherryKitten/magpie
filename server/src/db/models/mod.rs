use super::schema::*;
use anyhow::Result;
pub mod album;
pub mod artist;
pub mod genre;
pub mod track;

pub use album::Album;
pub use artist::Artist;
pub use genre::Genre;
pub use track::Track;

use diesel::prelude::*;

#[derive(Identifiable, Queryable, Associations, Eq, PartialEq, Debug)]
#[diesel(table_name = album_artists)]
#[diesel(belongs_to(Album))]
#[diesel(belongs_to(Artist))]
pub struct AlbumArtist {
    id: i32,
    album_id: Option<i32>,
    artist_id: Option<i32>,
}

#[derive(Identifiable, Queryable, Associations, Eq, PartialEq, Debug)]
#[diesel(belongs_to(Track))]
#[diesel(belongs_to(Artist))]
#[diesel(table_name = track_artists)]
pub struct TrackArtist {
    id: i32,
    track_id: Option<i32>,
    artist_id: Option<i32>,
}

#[derive(Identifiable, Queryable, Associations, Eq, PartialEq, Debug)]
#[diesel(belongs_to(Track))]
#[diesel(belongs_to(Genre))]
#[diesel(table_name = track_genres)]
pub struct TrackGenre {
    id: i32,
    track_id: Option<i32>,
    genre_id: Option<i32>,
}
