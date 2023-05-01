use anyhow::Result;
use lofty::Picture;

use super::*;

#[derive(
    Debug, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable, Identifiable,
)]
#[diesel(belongs_to(Art))]
#[diesel(table_name = albums)]
pub struct Album {
    pub id: i32,
    pub year: Option<i32>,
    pub title: String,
    pub art_id: Option<i32>,
}

impl Album {
    pub fn new(
        title: String,
        albumartists: Vec<&str>,
        year: i32,
        picture: Option<&Picture>,
        conn: &mut SqliteConnection,
    ) -> Result<Self> {
        log::debug!("Creating new Album {title}");

        let picture = if let Some(picture) = picture {
            Art::new(picture.to_owned(), conn).ok()
        } else {
            None
        };

        let insert = (
            albums::title.eq(title),
            albums::year.eq(year),
            albums::art_id.eq(picture.map(|picture| picture.id)),
        );
        let album: Album = diesel::insert_into(albums::table)
            .values(&insert)
            .get_result(conn)
            .unwrap();

        for artist in albumartists {
            Artist::by_title_or_new(artist, conn)?;

            diesel::insert_into(album_artists::table)
                .values((
                    album_artists::album_id.eq(album.id),
                    album_artists::artist_id.eq(Artist::by_title(artist, conn)?.id),
                ))
                .on_conflict_do_nothing()
                .execute(conn)?;
        }

        Ok(album)
    }

    pub fn all(conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(albums::table.select(Album::as_select()).get_results(conn)?)
    }

    pub fn by_id(id: i32, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(albums::table.find(id).first(conn)?)
    }

    pub fn by_title(title: &str, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(albums::table
            .select(Album::as_select())
            .filter(albums::title.like(format!("%{title}%")))
            .get_result::<Album>(conn)?)
    }

    pub fn by_title_exact(title: &str, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(albums::table
            .select(Album::as_select())
            .filter(albums::title.like(title.to_string()))
            .get_result::<Album>(conn)?)
    }

    pub fn by_artist_id(id: i32, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        let artist = Artist::by_id(id, conn)?;

        Ok(AlbumArtist::belonging_to(&artist)
            .inner_join(albums::table)
            .select(Album::as_select())
            .get_results(conn)?)
    }

    pub fn artist(&self, conn: &mut SqliteConnection) -> Result<Vec<Artist>> {
        Ok(AlbumArtist::belonging_to(self)
            .inner_join(artists::table)
            .select(Artist::as_select())
            .get_results(conn)?)
    }

    pub fn tracks(&self, conn: &mut SqliteConnection) -> Result<Vec<Track>> {
        Track::by_album_id(self.id, conn)
    }
}
