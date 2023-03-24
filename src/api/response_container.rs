use crate::metadata::{Album, Artist, Track};
use serde::{
    ser::{Serialize as SerializeT, SerializeMap},
    Deserialize, Serialize, Serializer,
};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct ResponseContainer {
    size: i32,
    level: ContentType,
    metadata: Vec<MetaDataContainer>,
}

impl ResponseContainer {
    pub(crate) fn new(v: Vec<MetaDataContainer>) -> Self {
        ResponseContainer {
            size: v.len() as i32,
            level: v.get(0).unwrap().kind,
            metadata: v,
        }
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Default)]
enum ContentType {
    Artist,
    Album,
    #[default]
    Track,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Map(HashMap<String, i32>);

impl Map {
    pub fn new(map: HashMap<String, i32>) -> Option<Self> {
        Some(Self(map))
    }
    pub fn from_artists(v: Vec<Artist>) -> Option<Self> {
        let mut map = HashMap::new();
        for i in v {
            map.insert(i.name.unwrap_or_default(), i.id);
        }
        Map::new(map)
    }
    pub fn from_tracks(v: Vec<Track>) -> Option<Self> {
        let mut map = HashMap::new();
        for i in v {
            map.insert(i.title.unwrap_or_default(), i.id);
        }
        Map::new(map)
    }
}

#[skip_serializing_none]
#[derive(Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct MetaDataContainer {
    kind: ContentType,
    id: i32,
    index: Option<Order>,
    disc_title: Option<String>,
    content_group: Option<String>,
    title: Option<String>,
    subtitle: Option<String>,
    year: Option<i32>,
    release_date: Option<String>,
    art: Option<bool>,
    genre: Option<Vec<String>>,
    children: Option<Map>,
    tracks: Option<Vec<TrackSummary>>,
    album: Option<Map>,
    artist: Option<Map>,
    album_artist: Option<Map>,
    grandparents: Option<Map>,
    bpm: Option<String>,
    length: Option<i32>,
    initial_key: Option<String>,
    language: Option<String>,
    added_at: Option<String>,
    media: Option<MediaContainer>,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackSummary {
    title: String,
    id: i32,
    disc_number: i32,
    track_number: i32,
}

impl From<Track> for TrackSummary {
    fn from(v: Track) -> Self {
        Self {
            title: v.title.unwrap_or_default(),
            id: v.id,
            disc_number: v.disc_number.unwrap_or(1),
            track_number: v.track_number.unwrap_or(1),
        }
    }
}

#[derive(Eq, PartialEq, Deserialize, Default)]
pub struct Order(Option<i32>, Option<i32>);
impl SerializeT for Order {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry("disc_number", &self.0.unwrap_or(1))?;
        s.serialize_entry("track_number", &self.1.unwrap_or(1))?;

        s.end()
    }
}

impl Order {
    fn new(disc: Option<i32>, track: Option<i32>) -> Option<Self> {
        Option::from(Order(disc, track))
    }
}

impl From<Artist> for MetaDataContainer {
    fn from(v: Artist) -> Self {
        MetaDataContainer {
            kind: ContentType::Artist,
            id: v.id,
            title: v.name,
            ..MetaDataContainer::default()
        }
    }
}
impl From<Track> for MetaDataContainer {
    fn from(v: Track) -> Self {
        let album = v.get_album(&mut crate::db::establish_connection().unwrap());
        let artist = v.get_artist(&mut crate::db::establish_connection().unwrap());

        MetaDataContainer {
            kind: ContentType::Track,
            id: v.id,
            index: Order::new(v.disc_number, v.track_number),
            title: v.title.to_owned(),
            year: v.year,
            disc_title: v.disc_title,
            content_group: v.content_group,
            subtitle: v.subtitle,
            release_date: v.release_date,
            bpm: v.bpm,
            length: v.length,
            initial_key: v.initial_key,
            language: v.language,
            added_at: v.added_at,
            artist: {
                if let Ok(artist) = artist {
                    Map::from_artists(artist)
                } else {
                    None
                }
            },
            album: {
                if let Ok(album) = album {
                    Some(album.into_map())
                } else {
                    None
                }
            },
            ..MetaDataContainer::default()
        }
    }
}

impl From<Album> for MetaDataContainer {
    fn from(v: Album) -> Self {
        let artist = v.get_artist(&mut crate::db::establish_connection().unwrap());
        let tracks = v.get_tracks(&mut crate::db::establish_connection().unwrap());
        MetaDataContainer {
            kind: ContentType::Album,
            id: v.id,
            title: v.title,
            year: v.year,
            art: Option::from(v.art.is_some()),
            album_artist: {
                if let Ok(artist) = artist {
                    Map::from_artists(artist)
                } else {
                    None
                }
            },
            tracks: {
                Some(
                    tracks
                        .unwrap()
                        .into_iter()
                        .map(TrackSummary::from)
                        .collect(),
                )
            },
            ..MetaDataContainer::default()
        }
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaContainer {
    duration: Option<i32>,
    file: String,
    size: i32,
    format: String,
}
