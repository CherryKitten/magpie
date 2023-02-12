use super::schema::*;
use crate::db::establish_connection;
use anyhow::{Error, Result};
use diesel::helper_types::{AsSelect, SqlTypeOf};

use std::path::Path;

use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use serde::Serialize as SerializeDerive;

use crate::metadata::vectorize_tags;
use lofty::{Accessor, ItemKey, Tag};
use log::trace;

type SqlType<M> = SqlTypeOf<AsSelect<M, Sqlite>>;
type BoxedTrackQuery<'a> = tracks::BoxedQuery<'a, Sqlite, SqlType<Track>>;

#[derive(
    Debug,
    PartialEq,
    Eq,
    Selectable,
    Queryable,
    QueryableByName,
    Identifiable,
    Associations,
    AsChangeset,
    Deserialize,
)]
#[diesel(belongs_to(Album))]
#[diesel(table_name = tracks)]
pub struct Track {
    pub id: i32,
    pub album_id: Option<i32>,
    pub path: Option<String>,
    pub track_number: Option<i32>,
    pub disc_number: Option<i32>,
    pub title: Option<String>,
    pub year: Option<i32>,
}

impl Serialize for Track {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Track", 7)?;

        state.serialize_field("id", &self.id)?;
        state.serialize_field("album_id", &self.album_id)?;
        match self.album_id {
            None => {}
            Some(id) => match Album::by_id(id) {
                Ok(album) => {
                    state.serialize_field("album", &album.title.unwrap_or("".to_string()))?
                }
                Err(_) => {}
            },
        }
        state.serialize_field("track_number", &self.track_number)?;
        state.serialize_field("disc_number", &self.disc_number)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("year", &self.year)?;
        state.end()
    }
}

pub enum TrackFilters {
    All,
    Id(i32),
    Path(String),
    AlbumId(i32),
    //AlbumTitle(String),
    Title(String),
    Year(i32),
}

impl Track {
    pub fn insert_or_update(tag: Tag, path: &Path) -> Result<Track> {
        trace!("Inserting or updating {:?}", path);
        let mut conn = establish_connection();

        let artists = vectorize_tags(tag.get_strings(&ItemKey::TrackArtist));
        let albumartists = vectorize_tags(tag.get_strings(&ItemKey::AlbumArtist));
        Artist::from_vec(&artists)?;
        Artist::from_vec(&albumartists)?;

        let album = match tag.album() {
            Some(album) => Some(Album::new(
                album.to_string(),
                albumartists,
                tag.year().unwrap_or_default() as i32,
            )?),
            None => None,
        };

        let insert = (
            tracks::title.eq(match tag.title() {
                Some(title) => Some(title.to_string()),
                None => None,
            }),
            tracks::track_number.eq(match tag.track() {
                Some(track) => Some(track as i32),
                None => None,
            }),
            tracks::disc_number.eq(match tag.disk() {
                Some(track) => Some(track as i32),
                None => None,
            }),
            tracks::path.eq(match path.to_str() {
                None => return Err(Error::msg("Could not get path")),
                Some(path) => path.to_string(),
            }),
            tracks::year.eq(match tag.year() {
                Some(year) => Some(year as i32),
                None => None,
            }),
            tracks::album_id.eq(match album {
                Some(album) => Some(album.id),
                None => None,
            }),
        );

        let track: Track = diesel::insert_into(tracks::table)
            .values(&insert)
            .on_conflict(tracks::path)
            .do_update()
            .set(insert.clone())
            .get_result(&mut conn)?;

        for artist in artists {
            diesel::insert_or_ignore_into(track_artists::table)
                .values((
                    track_artists::track_id.eq(track.id),
                    track_artists::artist_id.eq(Artist::by_name(&artist)?.id),
                ))
                .execute(&mut conn)?;
        }

        Ok(track)
    }

    pub fn get(filter: TrackFilters) -> Result<Vec<Track>> {
        let mut conn = establish_connection();

        let select = tracks::table.select(Track::as_select());

        let query: BoxedTrackQuery = match filter {
            TrackFilters::All => select
                .order((
                    tracks::album_id.asc(),
                    tracks::disc_number.asc(),
                    tracks::track_number.asc(),
                ))
                .into_boxed(),
            TrackFilters::Id(id) => select.find(id).into_boxed(),
            TrackFilters::Path(path) => select.filter(tracks::path.eq(path)).into_boxed(),
            TrackFilters::AlbumId(id) => select
                .filter(tracks::album_id.eq(id))
                .order((tracks::disc_number.asc(), tracks::track_number.asc()))
                .into_boxed(),
            //TrackFilters::AlbumTitle(title) => select.filter(tracks::album_id.eq()).order((tracks::disc_number.asc(), tracks::track_number.asc())).into_boxed(),
            TrackFilters::Title(title) => select.filter(tracks::title.eq(title)).into_boxed(),
            TrackFilters::Year(year) => select.filter(tracks::year.eq(year)).into_boxed(),
        };

        let result = query.load(&mut conn)?;

        Ok(result)
    }

    pub fn get_album(&self) -> Result<Album> {
        if let Some(album_id) = self.album_id {
            Ok(Album::by_id(album_id)?)
        } else {
            Err(Error::msg("Track has no album"))
        }
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Queryable,
    QueryableByName,
    Identifiable,
    AsChangeset,
    Selectable,
    Deserialize,
)]
#[diesel(table_name = albums)]

pub struct Album {
    id: i32,
    year: Option<i32>,
    pub title: Option<String>,
}

impl Serialize for Album {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Album", 3)?;

        let tracks = match Track::get(TrackFilters::AlbumId(self.id)) {
            Ok(tracks) => tracks,
            Err(_) => vec![],
        };

        state.serialize_field("id", &self.id)?;
        state.serialize_field("year", &self.year)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("tracks", &tracks)?;
        state.end()
    }
}

impl Album {
    pub fn new(title: String, albumartists: Vec<String>, year: i32) -> Result<Album> {
        let mut conn = establish_connection();

        let insert = (albums::title.eq(title), albums::year.eq(year));
        let album: Album = diesel::insert_into(albums::table)
            .values(&insert)
            .on_conflict((albums::title, albums::year))
            .do_update()
            .set(insert.clone())
            .get_result(&mut conn)?;

        for artist in albumartists {
            diesel::insert_into(album_artists::table)
                .values((
                    album_artists::album_id.eq(album.id),
                    album_artists::artist_id.eq(Artist::by_name(&artist)?.id),
                ))
                .on_conflict_do_nothing()
                .execute(&mut conn)?;
        }

        Ok(album)
    }
    pub fn all() -> Result<Vec<Album>> {
        let mut conn = establish_connection();

        Ok(albums::table.load::<Album>(&mut conn)?)
    }
    pub fn by_id(id: i32) -> Result<Album> {
        let mut conn = establish_connection();

        Ok(albums::table
            .select(Album::as_select())
            .find(id)
            .first::<Album>(&mut conn)?)
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Selectable,
    Queryable,
    QueryableByName,
    Identifiable,
    AsChangeset,
    SerializeDerive,
    Deserialize,
)]
#[diesel(table_name = artists)]
pub struct Artist {
    id: i32,
    name: Option<String>,
}

impl Artist {
    pub fn all() -> Result<Vec<Artist>> {
        let mut conn = establish_connection();

        Ok(artists::table.load::<Artist>(&mut conn)?)
    }
    pub fn by_id(id: i32) -> Result<Artist> {
        let mut conn = establish_connection();

        Ok(artists::table.find(id).first(&mut conn)?)
    }
    pub fn by_name(name: &str) -> Result<Artist> {
        let mut conn = establish_connection();

        Ok((artists::table.filter(artists::name.eq(name.to_string()))).first(&mut conn)?)
    }

    pub fn from_vec(artists: &Vec<String>) -> Result<()> {
        let mut conn = establish_connection();

        let mut temp = vec![];
        for artist in artists {
            temp.push(artists::name.eq(artist))
        }

        diesel::insert_or_ignore_into(artists::table)
            .values(temp)
            .execute(&mut conn)?;

        Ok(())
    }
}

#[derive(
    Debug, PartialEq, Eq, Queryable, Associations, Identifiable, SerializeDerive, Deserialize,
)]
#[diesel(table_name = album_artists)]
#[diesel(belongs_to(Album))]
#[diesel(belongs_to(Artist))]
pub struct AlbumArtist {
    id: i32,
    album_id: Option<i32>,
    artist_id: Option<i32>,
}

#[derive(
    Debug, PartialEq, Eq, Queryable, Identifiable, Associations, SerializeDerive, Deserialize,
)]
#[diesel(belongs_to(Track))]
#[diesel(belongs_to(Artist))]
#[diesel(table_name = track_artists)]
pub struct TrackArtist {
    id: i32,
    track_id: Option<i32>,
    artist_id: Option<i32>,
}
