use serde::{Deserialize, Serialize};
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

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct Map(HashMap<String, i32>);

#[skip_serializing_none]
#[derive(Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct MetaDataContainer {
    kind: ContentType,
    id: i32,
    index: Option<Index>,
    title: Option<String>,
    year: Option<i32>,
    art: Option<String>,
    genre: Option<Vec<String>>,
    children: Option<Map>,
    parents: Option<Map>,
    grandparents: Option<Map>,
    media: Option<MediaContainer>,
}

#[skip_serializing_none]
#[derive(Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Index(Option<i32>, Option<i32>);

impl Index {
    fn new(disc: Option<i32>, track: Option<i32>) -> Option<Self> {
        Option::from(Index(disc, track))
    }
}

impl From<crate::db::Artist> for MetaDataContainer {
    fn from(v: crate::db::Artist) -> Self {
        MetaDataContainer {
            kind: ContentType::Artist,
            id: v.id,
            title: v.name,
            ..MetaDataContainer::default()
        }
    }
}
impl From<crate::db::Track> for MetaDataContainer {
    fn from(v: crate::db::Track) -> Self {
        MetaDataContainer {
            kind: ContentType::Track,
            id: v.id,
            index: Index::new(v.disc_number, v.track_number),
            title: v.title,
            year: v.year,
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
