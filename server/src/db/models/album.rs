use super::*;
use crate::establish_connection;
use anyhow::Result;
use lofty::Picture;

#[derive(Debug, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable)]
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
        albumartists: Vec<String>,
        year: i32,
        picture: Option<&Picture>,
    ) -> Result<Self> {
        let mut conn = establish_connection()?;

        let insert = (
            albums::title.eq(title),
            albums::year.eq(year),
            albums::art.eq(picture.map(|picture| picture.data())),
        );
        let album: Album = diesel::insert_into(albums::table)
            .values(&insert)
            .get_result(&mut conn)
            .unwrap();

        for artist in albumartists {
            Artist::get_by_name_or_new(artist.clone())?;

            diesel::insert_into(album_artists::table)
                .values((
                    album_artists::album_id.eq(album.id),
                    album_artists::artist_id.eq(Artist::get_by_name(artist)?.id),
                ))
                .on_conflict_do_nothing()
                .execute(&mut conn)?;
        }

        Ok(album)
    }

    pub fn get_by_title(title: String) -> Result<Self> {
        let mut conn = establish_connection()?;

        if let Ok(album) = albums::table
            .select(Album::as_select())
            .filter(albums::title.like(title))
            .first(&mut conn)
        {
            Ok(album)
        } else {
            Err(anyhow::Error::msg("Failed to get album"))
        }
    }
}
