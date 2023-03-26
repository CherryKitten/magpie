use std::fs;
use std::path::Path;

use anyhow::{Error, Result};

use lofty::{Accessor, FileProperties, ItemKey, Tag};
use log::debug;

use super::*;

#[skip_serializing_none]
#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    Selectable,
    Queryable,
    QueryableByName,
    Insertable,
    Identifiable,
    Serialize,
    Deserialize,
)]
#[diesel(belongs_to(Album))]
#[diesel(table_name = tracks)]
pub struct Track {
    pub id: i32,
    pub album_id: Option<i32>,
    #[serde(skip)]
    pub path: Option<String>,
    #[serde(skip)]
    pub filesize: i32,
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
    #[serde(skip)]
    pub art: Option<Vec<u8>>,
    pub fallback_artist_id: Option<i32>,
}

impl Track {
    pub fn new(
        tag: (Tag, FileProperties),
        path: &Path,
        conn: &mut SqliteConnection,
    ) -> Result<Self> {
        let (tag, properties) = tag;

        debug!("Inserting or updating {:?}", path);

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
                if let Ok(album) = Album::get_by_title(&album, conn) {
                    Some(album)
                } else {
                    Some(Album::new(
                        album.to_string(),
                        albumartists,
                        tag.year().unwrap_or_default() as i32,
                        picture,
                        conn,
                    )?)
                }
            }
            None => None,
        };

        let insert = (
            tracks::title.eq(tag
                .title()
                .map(|title| title.to_string())
                .ok_or(Error::msg("No title"))?),
            tracks::track_number.eq(tag.track().unwrap_or(1) as i32),
            tracks::disc_number.eq(tag.disk().unwrap_or(1) as i32),
            tracks::path.eq(path.to_str().ok_or(Error::msg("Could not get path"))?),
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
            tracks::art.eq(None::<Vec<u8>>),
            tracks::label_id.eq(None::<i32>),
            tracks::fallback_artist_id.eq(None::<i32>),
        );

        let track: Track = diesel::insert_into(tracks::table)
            .values(&insert)
            .on_conflict(tracks::path)
            .do_update()
            .set(insert.clone())
            .get_result(conn)?;

        for artist in artists {
            Artist::get_by_title_or_new(artist, conn)?;

            diesel::insert_into(track_artists::table)
                .values((
                    track_artists::track_id.eq(track.id),
                    track_artists::artist_id.eq(Artist::get_by_title(artist, conn)?.id),
                ))
                .on_conflict_do_nothing()
                .execute(conn)?;
        }

        for genre in genres {
            let genre = Genre::get_or_new(genre, conn)?;

            diesel::insert_into(track_genres::table)
                .values((
                    track_genres::track_id.eq(track.id),
                    track_genres::genre_id.eq(genre.id),
                ))
                .on_conflict_do_nothing()
                .execute(conn)?;
        }

        Ok(track)
    }

    pub fn check(path: &Path, conn: &mut SqliteConnection) -> bool {
        let file_size = fs::metadata(path).unwrap().len() as i32;

        tracks::table
            .select(Track::as_select())
            .filter(tracks::path.eq(path.to_str().unwrap_or_default()))
            .filter(tracks::filesize.eq(file_size))
            .first(conn)
            .is_ok()
    }

    pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(tracks::table.select(Track::as_select()).get_results(conn)?)
    }

    pub fn get_by_id(id: i32, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(tracks::table
            .select(Track::as_select())
            .find(id)
            .first(conn)?)
    }

    pub fn get_one_by_title(title: &str, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(tracks::table
            .select(Track::as_select())
            .filter(tracks::title.like(title))
            .first::<Track>(conn)?)
    }

    pub fn get_by_title(title: &str, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(tracks::table
            .select(Track::as_select())
            .filter(tracks::title.like(title))
            .get_results::<Track>(conn)?)
    }

    pub fn get_by_album_id(id: i32, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(tracks::table
            .select(Track::as_select())
            .filter(tracks::album_id.eq(id))
            .order_by(tracks::disc_number)
            .then_order_by(tracks::track_number)
            .get_results(conn)?)
    }

    pub fn get_by_album_title(title: &str, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        let id = Album::get_by_title(title, conn)?.id;

        Self::get_by_album_id(id, conn)
    }

    pub fn get_by_artist_id(id: i32, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        let artist: Artist = artists::table
            .select(Artist::as_select())
            .find(id)
            .first(conn)?;

        Ok(TrackArtist::belonging_to(&artist)
            .inner_join(tracks::table)
            .select(Track::as_select())
            .get_results(conn)?)
    }

    pub fn get_by_genre_id(id: i32, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        let genre = Genre::get_by_id(id, conn)?;

        Ok(TrackGenre::belonging_to(&genre)
            .inner_join(tracks::table)
            .select(Track::as_select())
            .get_results(conn)?)
    }

    pub fn get_by_genre_title(title: &str, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        let id = Genre::get_by_title(title, conn)?.id;

        Self::get_by_genre_id(id, conn)
    }

    pub fn get_album(&self, conn: &mut SqliteConnection) -> Result<Album> {
        Album::get_by_id(self.album_id.ok_or(Error::msg("No album found"))?, conn)
    }

    pub fn get_artist(&self, conn: &mut SqliteConnection) -> Result<Vec<Artist>> {
        Ok(TrackArtist::belonging_to(self)
            .inner_join(artists::table)
            .select(Artist::as_select())
            .get_results(conn)?)
    }
}
