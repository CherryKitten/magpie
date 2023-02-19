use super::*;
use crate::db::models::artist::Artist;

type BoxedAlbumQuery<'a> = albums::BoxedQuery<'a, Sqlite, SqlType<Album>>;

#[derive(
    Debug, PartialEq, Eq, Queryable, QueryableByName, Identifiable, AsChangeset, Selectable,
)]
#[diesel(table_name = albums)]
pub struct Album {
    pub id: i32,
    pub year: Option<i32>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AlbumResponse {
    pub id: i32,
    pub year: Option<i32>,
    pub title: Option<String>,
    pub artist: Option<Vec<(i32, String)>>,
    pub tracks: Option<Vec<(i32, String)>>,
}

impl AlbumResponse {
    fn from(value: &Album, simple: bool) -> Self {
        let mut artists_vec: Vec<(i32, String)> = vec![];
        let mut tracks_vec: Vec<(i32, String)> = vec![];
        if !simple {
            if let Ok(tracks) = Track::get(None, None, Some(value.id), None, None, true) {
                match tracks {
                    ResponseContainerThingyHowTheFuckDoICallThis::One(track) => {
                        tracks_vec.push((track.id, track.title.unwrap()))
                    }
                    ResponseContainerThingyHowTheFuckDoICallThis::Many(tracks) => {
                        for track in tracks {
                            tracks_vec.push((track.id, track.title.unwrap()))
                        }
                    }
                }
            }
            if let Ok(artists) = value.get_artist() {
                for artist in artists {
                    artists_vec.push((artist.id, artist.name.unwrap()))
                }
            };
        }
        AlbumResponse {
            id: value.id,
            year: value.year,
            title: value.title.clone(),
            artist: Option::from(artists_vec),
            tracks: Option::from(tracks_vec),
        }
    }
}

impl Album {
    fn all() -> BoxedAlbumQuery<'static> {
        albums::table.select(Album::as_select()).into_boxed()
    }
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
                    album_artists::artist_id
                        .eq(Artist::get(None, Some(artist), Some(1), true)?.value().id),
                ))
                .on_conflict_do_nothing()
                .execute(&mut conn)?;
        }

        Ok(album)
    }
    pub fn get(
        id: Option<i32>,
        title: Option<String>,
        year: Option<i32>,
        limit: Option<i64>,
        simple: bool,
    ) -> Result<ResponseContainerThingyHowTheFuckDoICallThis<AlbumResponse>> {
        let mut conn = establish_connection();
        let mut query = Self::all();

        if let Some(id) = id {
            query = query.filter(albums::id.eq(id))
        }
        if let Some(year) = year {
            query = query.filter(albums::year.eq(year))
        };
        if let Some(title) = title {
            query = query.filter(albums::title.like("%".to_string() + &title + "%"))
        }

        if let Some(limit) = limit {
            query = query.limit(limit)
        };

        let result: Vec<Album> = query.load(&mut conn)?;

        let mut response = vec![];
        result
            .iter()
            .for_each(|elem| response.push(AlbumResponse::from(elem, simple)));

        if response.len() == 1 {
            Ok(ResponseContainerThingyHowTheFuckDoICallThis::One(
                response.remove(0),
            ))
        } else {
            Ok(ResponseContainerThingyHowTheFuckDoICallThis::Many(response))
        }
    }

    pub fn get_artist(&self) -> Result<Vec<Artist>> {
        let mut conn = establish_connection();

        Ok(AlbumArtist::belonging_to(self)
            .inner_join(artists::table)
            .select(artists::all_columns)
            .load::<Artist>(&mut conn)?)
    }

    pub fn from(value: AlbumResponse) -> Self {
        let mut conn = establish_connection();

        albums::table.find(value.id).first(&mut conn).unwrap()
    }
}
