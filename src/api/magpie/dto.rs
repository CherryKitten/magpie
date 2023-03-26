use crate::metadata::*;
use anyhow::Result;
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

const MAJOR: u32 = 0;
const MINOR: u32 = 1;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Version {
    major: u32,
    minor: u32,
}

impl Default for Version {
    fn default() -> Self {
        Version {
            major: MAJOR,
            minor: MINOR,
        }
    }
}
#[skip_serializing_none]
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Default)]
pub struct MagpieResponse {
    pub status: MagpieStatus,
    pub count: Option<i32>,
    pub page: Option<i32>,
    pub data: MagpieData,
    pub children: Option<MagpieData>,
}

#[skip_serializing_none]
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Default)]
pub struct MagpieArtist {
    pub id: i32,
    pub name: String,
}

#[skip_serializing_none]
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Default)]
pub struct MagpieAlbum {
    pub id: i32,
    pub title: String,
    pub year: Option<i32>,
    pub artist: Option<Vec<String>>,
    pub art: Option<String>,
}

#[skip_serializing_none]
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Default)]
pub struct MagpieTrack {
    pub id: i32,
    pub album: Option<String>,
    pub album_id: Option<i32>,
    pub track_number: Option<i32>,
    pub disc_number: Option<i32>,
    pub disc_title: Option<String>,
    pub content_group: Option<String>,
    pub title: String,
    pub subtitle: Option<String>,
    pub year: Option<i32>,
    pub release_date: Option<String>,
    pub bpm: Option<String>,
    pub length: Option<i32>,
    pub initial_key: Option<String>,
    pub language: Option<String>,
    pub label_id: Option<i32>,
    pub original_title: Option<String>,
    pub added_at: Option<String>,
    pub art: Option<String>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Default)]
pub enum MagpieStatus {
    Ok,
    Error(String),
    #[default]
    Unimplemented,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Default)]
#[allow(clippy::large_enum_variant)]
pub enum MagpieData {
    Artists(Vec<MagpieArtist>),
    Artist(MagpieArtist),
    Albums(Vec<MagpieAlbum>),
    Album(MagpieAlbum),
    Tracks(Vec<MagpieTrack>),
    Track(MagpieTrack),
    #[default]
    None,
}

impl MagpieResponse {
    pub fn new() -> Self {
        MagpieResponse {
            ..Default::default()
        }
    }
    pub fn error(mut self, msg: &str) -> Self {
        self.status = MagpieStatus::Error(String::from(msg));
        self
    }

    pub fn add_data(mut self, data: MagpieData) -> Self {
        self.status = MagpieStatus::Ok;
        self.data = data;

        self
    }
    pub fn set_pagination(mut self, count: i64, page: i64) -> Self {
        self.count = Some(count as i32);
        self.page = Some(page as i32);

        self
    }

    pub fn add_children(mut self, data: MagpieData) -> Self {
        self.children = Some(data);

        self
    }
}

impl MagpieTrack {
    pub fn new(track: Track, conn: &mut SqliteConnection) -> Result<Self> {
        let data = MagpieTrack {
            id: track.id,
            album: Some(track.get_album(conn)?.title),
            album_id: track.album_id,
            track_number: track.track_number,
            disc_number: track.disc_number,
            disc_title: track.disc_title,
            content_group: track.content_group,
            title: track.title,
            subtitle: track.subtitle,
            year: track.year,
            release_date: track.release_date,
            bpm: track.bpm,
            length: track.length,
            initial_key: track.initial_key,
            language: track.language,
            label_id: track.label_id,
            original_title: track.original_title,
            added_at: track.added_at,
            art: None,
        };

        Ok(data)
    }
}

impl MagpieAlbum {
    pub fn new(album: Album, conn: &mut SqliteConnection) -> Result<Self> {
        let data = MagpieAlbum {
            id: album.id,
            title: album.title.clone(),
            year: album.year,
            artist: Option::from(
                album
                    .get_artist(conn)?
                    .into_iter()
                    .map(|v| v.name)
                    .collect::<Vec<String>>(),
            ),
            art: None,
        };

        Ok(data)
    }
}

impl MagpieArtist {
    pub(crate) fn new(artist: Artist) -> Result<Self> {
        let data = MagpieArtist {
            id: artist.id,
            name: artist.name,
        };

        Ok(data)
    }
}
