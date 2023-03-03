use super::*;

use crate::establish_connection;
use crate::metadata::vectorize_tags;
use anyhow::{Error, Result};
use lofty::{Accessor, ItemKey, Tag};
use log::trace;
use std::fs;
use std::path::Path;

#[derive(
    Debug, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable, Identifiable,
)]
#[diesel(belongs_to(Album))]
#[diesel(table_name = tracks)]
pub struct Track {
    pub id: i32,
    pub album_id: Option<i32>,
    pub path: Option<String>,
    pub filesize: i32,
    pub track_number: Option<i32>,
    pub disc_number: Option<i32>,
    pub title: Option<String>,
    pub year: Option<i32>,
}

impl Track {
    pub fn new(tag: Tag, path: &Path) -> Result<Self> {
        trace!("Inserting or updating {:?}", path);
        let mut conn = establish_connection()?;
        let file_size = fs::metadata(path)?.len();

        let artists = vectorize_tags(tag.get_strings(&ItemKey::TrackArtist));
        let albumartists = vectorize_tags(tag.get_strings(&ItemKey::AlbumArtist));
        let genres = vectorize_tags(tag.get_strings(&ItemKey::Genre));

        let picture = {
            if tag.picture_count() > 0 {
                Some(&tag.pictures()[0])
            } else {
                None
            }
        };

        let album = match tag.album() {
            Some(album) => {
                if let Ok(album) = Album::get_by_title(&album) {
                    Some(album)
                } else {
                    Some(Album::new(
                        album.to_string(),
                        albumartists,
                        tag.year().unwrap_or_default() as i32,
                        picture,
                    )?)
                }
            }
            None => None,
        };

        let insert = (
            tracks::title.eq(tag.title().map(|title| title.to_string())),
            tracks::track_number.eq(tag.track().map(|track| track as i32)),
            tracks::disc_number.eq(tag.disk().map(|track| track as i32)),
            tracks::path.eq(match path.to_str() {
                None => return Err(Error::msg("Could not get path")),
                Some(path) => path.to_string(),
            }),
            tracks::filesize.eq(file_size as i32),
            tracks::year.eq(tag.year().map(|year| year as i32)),
            tracks::album_id.eq(album.map(|album| album.id)),
        );

        let track: Track = diesel::insert_into(tracks::table)
            .values(&insert)
            .on_conflict(tracks::path)
            .do_update()
            .set(insert.clone())
            .get_result(&mut conn)?;

        for artist in artists {
            Artist::get_by_title_or_new(&artist)?;

            diesel::insert_into(track_artists::table)
                .values((
                    track_artists::track_id.eq(track.id),
                    track_artists::artist_id.eq(Artist::get_by_title(&artist)?.id),
                ))
                .on_conflict_do_nothing()
                .execute(&mut conn)?;
        }

        for genre in genres {
            let genre = Genre::get_or_new(genre.clone())?;

            diesel::insert_into(track_genres::table)
                .values((
                    track_genres::track_id.eq(track.id),
                    track_genres::genre_id.eq(genre.id),
                ))
                .on_conflict_do_nothing()
                .execute(&mut conn)?;
        }

        Ok(track)
    }

    pub fn check(path: &Path) -> bool {
        let mut conn = establish_connection().unwrap();
        let file_size = fs::metadata(path).unwrap().len() as i32;

        tracks::table
            .select(Track::as_select())
            .filter(tracks::path.eq(path.to_str().unwrap_or_default()))
            .filter(tracks::filesize.eq(file_size))
            .first(&mut conn)
            .is_ok()
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let mut conn = establish_connection()?;

        Ok(tracks::table
            .select(tracks::all_columns)
            .get_results(&mut conn)?)
    }

    pub fn get_by_id(id: i32) -> Result<Self> {
        let mut conn = establish_connection()?;

        Ok(tracks::table.find(id).first(&mut conn)?)
    }

    pub fn get_one_by_title(title: &str) -> Result<Self> {
        let mut conn = establish_connection()?;
        Ok(tracks::table
            .select(tracks::all_columns)
            .filter(tracks::title.eq(title))
            .first::<Track>(&mut conn)?)
    }

    pub fn get_by_title(title: &str) -> Result<Vec<Self>> {
        let mut conn = establish_connection()?;
        Ok(tracks::table
            .select(tracks::all_columns)
            .filter(tracks::title.eq(title))
            .get_results::<Track>(&mut conn)?)
    }

    pub fn get_by_album_id(id: i32) -> Result<Vec<Self>> {
        let mut conn = establish_connection()?;
        Ok(tracks::table
            .select(tracks::all_columns)
            .filter(tracks::album_id.eq(id))
            .get_results(&mut conn)?)
    }

    pub fn get_by_album_title(title: &str) -> Result<Vec<Self>> {

        let id = Album::get_by_title(title)?.id;

        Self::get_by_album_id(id)
    }

    pub fn get_by_artist_id(id: i32) -> Result<Vec<Self>> {
        let mut conn = establish_connection()?;

        let artist: Artist = artists::table.find(id).first(&mut conn)?;

        Ok(TrackArtist::belonging_to(&artist)
            .inner_join(tracks::table)
            .select(tracks::all_columns)
            .get_results(&mut conn)?)
    }

    pub fn get_by_artist_title(title: &str) -> Result<Vec<Self>> {
        let id = Artist::get_by_title(title)?.id;

        Self::get_by_artist_id(id)
    }

    pub fn get_by_genre_id(id: i32) -> Result<Vec<Self>> {
        let mut conn = establish_connection()?;

        let genre = Genre::get_by_id(id)?;

        Ok(TrackGenre::belonging_to(&genre)
            .inner_join(tracks::table)
            .select(tracks::all_columns)
            .get_results(&mut conn)?)
    }

    pub fn get_by_genre_title(title: &str) -> Result<Vec<Self>> {
        let id = Genre::get_by_title(title)?.id;

        Self::get_by_genre_id(id)
    }
}
