use serde::{Deserialize, Serialize};
use crate::db::models::*;
use crate::metadata::get_album_by_id;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackResponse {
    id: i32,
    albumid: Option<i32>,
    album: Option<String>,
    track_number: Option<i32>,
    disc_number: Option<i32>,
    title: Option<String>,
    year: Option<i32>,
}

impl TrackResponse {
    pub fn from_track(track: Track) -> Self {
        TrackResponse {
            id: track.id,
            albumid: track.album_id,
            album: match get_album_by_id(track.album_id.unwrap_or(0)) {
                Some(a) => a.title,
                None => None,
            },
            track_number: track.track_number,
            disc_number: track.disc_number,
            title: track.title,
            year: track.year,
        }
    }
}