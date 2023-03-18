use super::*;

use anyhow::{Error, Result};
use duplicate::duplicate;
use lofty::Picture;
use std::collections::HashMap;

#[derive(
    Debug, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable, Identifiable,
)]
#[diesel(table_name = albums)]
pub struct Album {
    pub id: i32,
    pub year: Option<i32>,
    pub title: Option<String>,
    pub art: Option<Vec<u8>>,
}

impl Album {
    pub fn new(
        title: String,
        albumartists: Vec<&str>,
        year: i32,
        picture: Option<&Picture>,
        conn: &mut SqliteConnection,
    ) -> Result<Self> {
        let insert = (
            albums::title.eq(title),
            albums::year.eq(year),
            albums::art.eq(picture.map(|picture| picture.data())),
        );
        let album: Album = diesel::insert_into(albums::table)
            .values(&insert)
            .get_result(conn)
            .unwrap();

        for artist in albumartists {
            Artist::get_by_title_or_new(artist, conn)?;

            diesel::insert_into(album_artists::table)
                .values((
                    album_artists::album_id.eq(album.id),
                    album_artists::artist_id.eq(Artist::get_by_title(artist, conn)?.id),
                ))
                .on_conflict_do_nothing()
                .execute(conn)?;
        }

        Ok(album)
    }

    pub fn get(
        mut filter: HashMap<String, String>,
        conn: &mut SqliteConnection,
    ) -> Result<Vec<Self>> {
        let mut select = albums::table.select(Album::as_select()).into_boxed();

        if !filter.is_empty() {
            duplicate! {
                [
                    key statement;
                    [ "title" ] [ albums::title.like(format!("%{item}%")) ];
                    [ "year" ]  [ albums::year.eq((item.parse::<i32>()?)) ];
                ]
                if let Some(item) = filter.remove(key) {
                select = select.filter(statement);
            }}
        }

        select = select.limit(filter.remove("limit").unwrap_or("50".to_string()).parse()?);
        select = select.offset(filter.remove("offset").unwrap_or("0".to_string()).parse()?);

        select = select
            .distinct()
            .order_by(albums::year)
            .then_order_by(albums::title);

        let result: Vec<Album> = select.load(conn)?;
        if !result.is_empty() {
            Ok(result)
        } else {
            Err(Error::msg("Did not find any tracks"))
        }
    }

    pub fn all(conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(albums::table
            .select(albums::all_columns)
            .get_results(conn)?)
    }

    pub fn get_by_id(id: i32, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(albums::table.find(id).first(conn)?)
    }

    pub fn get_by_title(title: &str, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(albums::table
            .select(albums::all_columns)
            .filter(albums::title.like(format!("%{title}%")))
            .get_result::<Album>(conn)?)
    }

    pub fn get_by_artist_id(id: i32, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        let artist: Artist = artists::table.find(id).first(conn)?;

        Ok(AlbumArtist::belonging_to(&artist)
            .inner_join(albums::table)
            .select(albums::all_columns)
            .get_results(conn)?)
    }

    pub fn get_by_artist_title(title: &str, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        let id = Artist::get_by_title(title, conn)?.id;

        Self::get_by_artist_id(id, conn)
    }

    pub fn into_map(self) -> crate::api::response_container::Map {
        let mut map = HashMap::new();

        map.insert(self.title.unwrap_or_default(), self.id);

        crate::api::response_container::Map::new(map).unwrap_or_default()
    }

    pub fn get_artist(&self, conn: &mut SqliteConnection) -> Result<Vec<Artist>> {
        Ok(AlbumArtist::belonging_to(self)
            .inner_join(artists::table)
            .select(artists::all_columns)
            .get_results(conn)?)
    }

    pub fn get_tracks(&self, conn: &mut SqliteConnection) -> Result<Vec<Track>> {
        Track::get_by_album_id(self.id, conn)
    }
}
