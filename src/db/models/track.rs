use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Error, Result};
use duplicate::duplicate;
use lofty::{Accessor, FileProperties, ItemKey, Tag};
use log::trace;

use crate::establish_connection;

use super::*;

#[derive(
    Debug, Default, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable, Identifiable,
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
    pub disc_title: Option<String>,
    pub content_group: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub year: Option<i32>,
    pub release_date: Option<String>,
    pub bpm: Option<String>,
    pub length: Option<i32>,
    pub initial_key: Option<String>,
    pub language: Option<String>,
    // TODO: pub label_id: Option<i32>,
    pub original_title: Option<String>,
    pub added_at: Option<String>,
}

impl Track {
    pub fn new(tag: (Tag, FileProperties), path: &Path) -> Result<Self> {
        let (tag, properties) = tag;

        trace!("Inserting or updating {:?}", path);
        let mut conn = establish_connection()?;
        let file_size = fs::metadata(path)?.len();

        let artists: Vec<&str> = tag.get_strings(&ItemKey::TrackArtist).collect();
        let albumartists: Vec<&str> = tag.get_strings(&ItemKey::AlbumArtist).collect();
        let genres: Vec<&str> = tag.get_strings(&ItemKey::Genre).collect();

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
            tracks::track_number.eq(tag.track().unwrap_or(1) as i32),
            tracks::disc_number.eq(tag.disk().unwrap_or(1) as i32),
            tracks::path.eq(match path.to_str() {
                None => return Err(Error::msg("Could not get path")),
                Some(path) => path.to_string(),
            }),
            tracks::filesize.eq(file_size as i32),
            tracks::year.eq(tag.year().map(|year| year as i32)),
            tracks::release_date.eq(tag.get_string(&ItemKey::OriginalReleaseDate)),
            tracks::album_id.eq(album.map(|album| album.id)),
            tracks::length.eq(properties.duration().as_secs() as i32),
            tracks::disc_title.eq(tag.get_string(&ItemKey::SetSubtitle)),
            tracks::content_group.eq(tag.get_string(&ItemKey::ContentGroup)),
            tracks::subtitle.eq(tag.get_string(&ItemKey::TrackSubtitle)),
            tracks::bpm.eq(tag.get_string(&ItemKey::BPM)),
            tracks::initial_key.eq(tag.get_string(&ItemKey::InitialKey)),
            tracks::language.eq(tag.get_string(&ItemKey::Language)),
        );

        let track: Track = diesel::insert_into(tracks::table)
            .values(&insert)
            .on_conflict(tracks::path)
            .do_update()
            .set(insert.clone())
            .get_result(&mut conn)?;

        for artist in artists {
            Artist::get_by_title_or_new(artist)?;

            diesel::insert_into(track_artists::table)
                .values((
                    track_artists::track_id.eq(track.id),
                    track_artists::artist_id.eq(Artist::get_by_title(artist)?.id),
                ))
                .on_conflict_do_nothing()
                .execute(&mut conn)?;
        }

        for genre in genres {
            let genre = Genre::get_or_new(genre)?;

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

    pub fn get(mut filter: HashMap<String, String>) -> Result<Vec<Self>> {
        let mut conn = establish_connection()?;

        let mut select = tracks::table.select(Track::as_select()).into_boxed();

        if !filter.is_empty() {
            duplicate! {
                [
                    key statement;
                    [ "title" ]         [ tracks::title.like(format!("%{item}%")) ];
                    [ "subtitle" ]      [ tracks::subtitle.like(format!("%{item}%")) ];
                    [ "album" ]         [ tracks::album_id.eq(Album::get_by_title(&item)?.id) ];
                    [ "year" ]          [ tracks::year.eq((item.parse::<i32>()?)) ];
                    [ "bpm" ]           [ tracks::bpm.eq(item) ];
                    [ "language" ]      [ tracks::language.eq(item) ];
                ]
                if let Some(item) = filter.remove(key) {
                select = select.filter(statement);
            }}
        }

        select = select.limit(
            filter
                .remove("limit")
                .unwrap_or("50".to_string())
                .parse()
                .unwrap(),
        );
        select = select.offset(
            filter
                .remove("offset")
                .unwrap_or("0".to_string())
                .parse()
                .unwrap(),
        );

        select = select
            .distinct()
            .order_by(tracks::disc_number)
            .then_order_by(tracks::track_number);

        let result: Vec<Track> = select.load(&mut conn)?;
        if !result.is_empty() {
            Ok(result)
        } else {
            Err(Error::msg("Did not find any tracks"))
        }
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
            .filter(tracks::title.like(title))
            .first::<Track>(&mut conn)?)
    }

    pub fn get_by_title(title: &str) -> Result<Vec<Self>> {
        let mut conn = establish_connection()?;
        Ok(tracks::table
            .select(tracks::all_columns)
            .filter(tracks::title.like(title))
            .get_results::<Track>(&mut conn)?)
    }

    pub fn get_by_album_id(id: i32) -> Result<Vec<Self>> {
        let mut conn = establish_connection()?;
        Ok(tracks::table
            .select(tracks::all_columns)
            .filter(tracks::album_id.eq(id))
            .order_by(tracks::disc_number)
            .then_order_by(tracks::track_number)
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

    pub fn get_album(&self) -> Result<Album> {
        Album::get_by_id(self.album_id.ok_or(Error::msg("No album found"))?)
    }

    pub fn get_artist(&self) -> Result<Vec<Artist>> {
        let mut conn = establish_connection()?;

        Ok(TrackArtist::belonging_to(self)
            .inner_join(artists::table)
            .select(artists::all_columns)
            .get_results(&mut conn)?)
    }
}
