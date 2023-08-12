use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use diesel::prelude::*;

pub use album::Album;
pub use artist::Artist;
pub use genre::Genre;
pub use track::Track;

use crate::db::schema::*;
use crate::Result;

pub mod album;
pub mod artist;
pub mod genre;
pub mod track;

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

#[derive(Selectable, Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = art)]
pub struct Art {
    pub id: i32,
    pub hash: f64,
    pub data: Vec<u8>,
}

impl Art {
    fn check_hash(hash: f64, conn: &mut SqliteConnection) -> bool {
        art::table
            .select(Art::as_select())
            .filter(art::hash.eq(hash))
            .first(conn)
            .is_ok()
    }
    fn new(data: lofty::Picture, conn: &mut SqliteConnection) -> Result<Self> {
        let data = data.data();
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        let hash = hasher.finish();
        if Self::check_hash(hash as f64, conn) {
            return Err(crate::Error::msg("Image already exists"));
        }

        let data = Vec::from(data);

        let result = diesel::insert_into(art::table)
            .values((art::hash.eq(hash as f64), art::data.eq(data)))
            .get_result::<Art>(conn)?;

        Ok(result)
    }
    pub fn get_by_id(id: i32, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(art::table.select(Art::as_select()).find(id).first(conn)?)
    }
}
