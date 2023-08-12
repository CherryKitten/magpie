use std::fs;
use std::path::Path;

use anyhow::{Error, Result};
use lofty::{Accessor, FileProperties, ItemKey, Tag};
use log::debug;

use super::*;

#[derive(
    Debug, Default, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable, Identifiable,
)]
#[diesel(belongs_to(Album))]
#[diesel(belongs_to(Art))]
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
    pub art_id: Option<i32>,
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
                if let Ok(album) = Album::by_title_exact(&album, conn) {
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
            tracks::bpm.eq(tag.get_string(&ItemKey::Bpm)),
            tracks::initial_key.eq(tag.get_string(&ItemKey::InitialKey)),
            tracks::language.eq(tag.get_string(&ItemKey::Language)),
            tracks::art_id.eq(None::<i32>),
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
            Artist::by_title_or_new(artist, conn)?;

            diesel::insert_into(track_artists::table)
                .values((
                    track_artists::track_id.eq(track.id),
                    track_artists::artist_id.eq(Artist::by_title(artist, conn)?.id),
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

    pub fn all(conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(tracks::table.select(Track::as_select()).get_results(conn)?)
    }

    pub fn by_id(id: i32, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(tracks::table
            .select(Track::as_select())
            .find(id)
            .first(conn)?)
    }

    pub fn by_title_one(title: &str, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(tracks::table
            .select(Track::as_select())
            .filter(tracks::title.like(title))
            .first::<Track>(conn)?)
    }

    pub fn by_title_all(title: &str, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(tracks::table
            .select(Track::as_select())
            .filter(tracks::title.like(title))
            .get_results::<Track>(conn)?)
    }

    pub fn by_album_id(id: i32, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(tracks::table
            .select(Track::as_select())
            .filter(tracks::album_id.eq(id))
            .order_by(tracks::disc_number)
            .then_order_by(tracks::track_number)
            .get_results(conn)?)
    }

    pub fn by_album_title(title: &str, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        let id = Album::by_title(title, conn)?.id;

        Self::by_album_id(id, conn)
    }

    pub fn by_artist_id(id: i32, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        let artist: Artist = artists::table
            .select(Artist::as_select())
            .find(id)
            .first(conn)?;

        Ok(TrackArtist::belonging_to(&artist)
            .inner_join(tracks::table)
            .select(Track::as_select())
            .get_results(conn)?)
    }

    pub fn by_genre_id(id: i32, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        let genre = Genre::get_by_id(id, conn)?;

        Ok(TrackGenre::belonging_to(&genre)
            .inner_join(tracks::table)
            .select(Track::as_select())
            .get_results(conn)?)
    }

    pub fn by_genre_title(title: &str, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        let id = Genre::get_by_title(title, conn)?.id;

        Self::by_genre_id(id, conn)
    }

    pub fn album(&self, conn: &mut SqliteConnection) -> Result<Album> {
        Album::by_id(self.album_id.ok_or(Error::msg("No album found"))?, conn)
    }

    pub fn artist(&self, conn: &mut SqliteConnection) -> Result<Vec<Artist>> {
        Ok(TrackArtist::belonging_to(self)
            .inner_join(artists::table)
            .select(Artist::as_select())
            .get_results(conn)?)
    }
}
