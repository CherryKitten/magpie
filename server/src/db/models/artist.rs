use super::*;

type BoxedArtistQuery<'a> = artists::BoxedQuery<'a, Sqlite, SqlType<Artist>>;

#[derive(
    Debug,
    PartialEq,
    Eq,
    Selectable,
    Queryable,
    QueryableByName,
    Identifiable,
    AsChangeset,
    Deserialize,
)]
#[diesel(table_name = artists)]
pub struct Artist {
    pub id: i32,
    pub name: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct ArtistResponse {
    pub id: i32,
    pub name: Option<String>,
    pub albums: Option<Vec<(i32, String)>>,
}

#[derive(Deserialize, Default, Clone)]
pub struct ArtistFilter {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub limit: Option<i64>,
    /// String content doesn't really matter, basically if this is Some(), we filter out all
    /// Artists that have no own albums
    pub with_albums: Option<String>,
}

impl ArtistResponse {
    fn from(value: &Artist, simple: bool) -> Self {
        let mut albums_vec: Vec<(i32, String)> = vec![];
        if !simple {
            if let Ok(albums) = Artist::all_albums(value.id) {
                for album in albums {
                    albums_vec.push((album.id, album.title.unwrap()))
                }
            };
        }
        let albums = match albums_vec.len() {
            0 => None,
            _ => Some(albums_vec)
        };
        ArtistResponse {
            id: value.id,
            name: value.name.clone(),
            albums,
        }
    }
}

impl Artist {
    pub fn all() -> BoxedArtistQuery<'static> {
        artists::table.select(Artist::as_select()).into_boxed()
    }
    pub fn get(
        filter: ArtistFilter,
        simple: bool,
    ) -> Result<ResponseContainerThingyHowTheFuckDoICallThis<ArtistResponse>> {
        let mut conn = establish_connection();
        let mut query = Self::all();

        if let Some(id) = filter.id {
            query = query.filter(artists::id.eq(id))
        }

        if let Some(name) = filter.name {
            query = query.filter(artists::name.like("%".to_string() + &name + "%"))
        }

        if let Some(limit) = filter.limit {
            query = query.limit(limit)
        };

        let result: Vec<Artist> = query.load(&mut conn)?;

        let mut response = vec![];
        result
            .iter()
            .for_each(|elem| response.push(ArtistResponse::from(elem, simple)));

        if filter.with_albums.is_some() {
            response.retain(|a| a.albums.is_some())
        }

        if response.len() == 1 {
            Ok(ResponseContainerThingyHowTheFuckDoICallThis::One(
                response.remove(0),
            ))
        } else {
            Ok(ResponseContainerThingyHowTheFuckDoICallThis::Many(response))
        }
    }

    pub fn all_albums(id: i32) -> Result<Vec<Album>> {
        let mut conn = establish_connection();

        let artist = artists::table.find(id).first::<Artist>(&mut conn)?;

        Ok(AlbumArtist::belonging_to(&artist)
            .inner_join(albums::table)
            .select(albums::all_columns)
            .load::<Album>(&mut conn)?)
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

    pub fn from(value: ArtistResponse) -> Self {
        let mut conn = establish_connection();

        artists::table.find(value.id).first(&mut conn).unwrap()
    }
}
