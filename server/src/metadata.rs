use diesel::prelude::*;
use lofty::{Accessor, ItemKey, Tag};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::db::schema::*;

#[derive(Debug, PartialEq, Eq, Queryable, QueryableByName, Serialize, Deserialize)]
#[diesel(table_name = tracks)]
pub struct Track {
    id: i32,
    album: Album,
    path: String,
    track_number: Option<i32>,
    disc_number: Option<i32>,
    title: Option<String>,
    year: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Queryable, QueryableByName, Serialize, Deserialize)]
#[diesel(table_name = albums)]
pub struct Album {
    id: i32,
    year: Option<i32>,
    title: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Queryable, QueryableByName, Serialize, Deserialize)]
#[diesel(table_name = artists)]
pub struct Artist {
    id: i32,
    name: String,
}

fn vectorize_tags<'a>(tags: impl Iterator<Item = &'a str> + Sized) -> Vec<String> {
    let mut temp_vec = vec![];
    for tag in tags {
        temp_vec.push(tag.to_string());
    }
    temp_vec
}
