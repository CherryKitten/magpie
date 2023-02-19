use super::schema::*;
use crate::db::establish_connection;
use anyhow::{Error, Result};
use diesel::helper_types::{AsSelect, SqlTypeOf};
use std::fs;

use std::path::Path;

use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use serde::{Deserialize, Serialize};

use crate::metadata::vectorize_tags;
use album::Album;
use artist::Artist;
use lofty::{Accessor, ItemKey, Tag};
use log::trace;
use track::Track;

pub mod album;
pub mod artist;
pub mod track;

type SqlType<M> = SqlTypeOf<AsSelect<M, Sqlite>>;

#[derive(Serialize)]
#[serde(untagged)]
pub enum ResponseContainerThingyHowTheFuckDoICallThis<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> ResponseContainerThingyHowTheFuckDoICallThis<T> {
    pub fn value(self) -> T {
        match self {
            ResponseContainerThingyHowTheFuckDoICallThis::One(value) => value,
            ResponseContainerThingyHowTheFuckDoICallThis::Many(mut value) => value.remove(0),
        }
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
