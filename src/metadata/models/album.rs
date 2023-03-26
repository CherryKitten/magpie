use super::*;
use anyhow::Result;
use lofty::Picture;

#[derive(
    Debug, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable, Identifiable,
)]
#[diesel(table_name = albums)]
pub struct Album {
    pub id: i32,
    pub year: Option<i32>,
    pub title: String,
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

    pub fn all(conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(albums::table.select(Album::as_select()).get_results(conn)?)
    }

    pub fn get_by_id(id: i32, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(albums::table.find(id).first(conn)?)
    }

    pub fn get_by_title(title: &str, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(albums::table
            .select(Album::as_select())
            .filter(albums::title.like(format!("%{title}%")))
            .get_result::<Album>(conn)?)
    }

    pub fn get_by_artist_id(id: i32, conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        let artist = Artist::get_by_id(id, conn)?;

        Ok(AlbumArtist::belonging_to(&artist)
            .inner_join(albums::table)
            .select(Album::as_select())
            .get_results(conn)?)
    }

    pub fn get_artist(&self, conn: &mut SqliteConnection) -> Result<Vec<Artist>> {
        Ok(AlbumArtist::belonging_to(self)
            .inner_join(artists::table)
            .select(Artist::as_select())
            .get_results(conn)?)
    }

    pub fn get_tracks(&self, conn: &mut SqliteConnection) -> Result<Vec<Track>> {
        Track::get_by_album_id(self.id, conn)
    }
}
