use super::*;
use crate::db::models::album::AlbumFilter;
use crate::db::models::artist::{Artist, ArtistFilter};
type BoxedTrackQuery<'a> = tracks::BoxedQuery<'a, Sqlite, SqlType<Track>>;

#[derive(
    PartialEq, Eq, Selectable, Queryable, QueryableByName, Identifiable, Associations, AsChangeset,
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
    pub title: Option<String>,
    pub year: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct TrackResponse {
    pub id: i32,
    pub title: Option<String>,
    pub track_number: Option<i32>,
    pub disc_number: Option<i32>,
    pub year: Option<i32>,
    pub album: Option<(i32, String)>,
    pub artist: Option<Vec<(i32, String)>>,
    pub album_artist: Option<Vec<(i32, String)>>,
}

#[derive(Deserialize, Default, Clone)]
pub struct TrackFilter {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub album: Option<i32>,
    pub year: Option<i32>,
    pub limit: Option<i64>,
}

impl TrackResponse {
    fn from(value: &Track, simple: bool) -> Self {
        let mut artists: Vec<(i32, String)> = vec![];
        let mut album_artists: Vec<(i32, String)> = vec![];
        if !simple {
            if let Ok(artist) = value.get_artist() {
                for i in artist {
                    artists.push((i.id, i.name.unwrap()));
                }
            }

            if let Ok(artist) = value.get_album().unwrap().get_artist() {
                for i in artist {
                    album_artists.push((i.id, i.name.unwrap()))
                }
            }
        }

        TrackResponse {
            id: value.id,
            track_number: value.track_number,
            disc_number: value.disc_number,
            title: value.title.clone(),
            year: value.year,

            album: {
                match value.album_id {
                    None => None,
                    Some(id) => {
                        if let Ok(album) = Album::get(
                            AlbumFilter {
                                id: Some(id),
                                limit: Some(1),
                                ..AlbumFilter::default()
                            },
                            true
                        ) {
                            Some((id, album.value().title.unwrap()))
                        } else {
                            None
                        }
                    }
                }
            },
            artist: Option::from(artists),
            album_artist: Option::from(album_artists),
        }
    }
}

impl Track {
    fn all() -> BoxedTrackQuery<'static> {
        tracks::table.select(Track::as_select()).into_boxed()
    }

    pub fn get(
        filter: TrackFilter,
        simple: bool
    ) -> Result<ResponseContainerThingyHowTheFuckDoICallThis<TrackResponse>> {
        let mut conn = establish_connection();
        let mut query = Self::all();

        if let Some(id) = filter.id {
            query = query.filter(tracks::id.eq(id))
        }
        if let Some(year) = filter.year {
            query = query.filter(tracks::year.eq(year))
        };
        if let Some(title) = filter.title {
            query = query.filter(tracks::title.like("%".to_string() + &title + "%"))
        }
        if let Some(album) = filter.album {
            query = query.filter(tracks::album_id.eq(album))
        }
        if let Some(limit) = filter.limit {
            query = query.limit(limit)
        };

        query = query
            .order_by(tracks::disc_number)
            .then_order_by(tracks::track_number);

        let result: Vec<Track> = query.load(&mut conn)?;

        let mut response = vec![];
        result
            .iter()
            .for_each(|elem| response.push(TrackResponse::from(elem, simple)));

        if response.len() == 1 {
            Ok(ResponseContainerThingyHowTheFuckDoICallThis::One(
                response.remove(0),
            ))
        } else {
            Ok(ResponseContainerThingyHowTheFuckDoICallThis::Many(response))
        }
    }

    pub fn insert_or_update(tag: Tag, path: &Path) -> Result<Track> {
        trace!("Inserting or updating {:?}", path);
        let mut conn = establish_connection();
        let file_size = fs::metadata(path)?.len();

        let artists = vectorize_tags(tag.get_strings(&ItemKey::TrackArtist));
        let albumartists = vectorize_tags(tag.get_strings(&ItemKey::AlbumArtist));
        Artist::from_vec(&artists)?;
        Artist::from_vec(&albumartists)?;

        let picture = {
            if tag.picture_count() > 0 {
                Some(&tag.pictures()[0])
            } else {
                None
            }
        };

        let album = match tag.album() {
            Some(album) => Some(Album::new(
                album.to_string(),
                albumartists,
                tag.year().unwrap_or_default() as i32,
                picture,
            )?),
            None => None,
        };

        let insert = (
            tracks::title.eq(tag.title().map(|title| title.to_string())),
            tracks::track_number.eq(tag.track().map(|track| track as i32)),
            tracks::disc_number.eq(tag.disk().map(|track| track as i32)),
            tracks::path.eq(match path.to_str() {
                None => return Err(Error::msg("Could not get path")),
                Some(path) => path.to_string(),
            }),
            tracks::filesize.eq(file_size as i32),
            tracks::year.eq(tag.year().map(|year| year as i32)),
            tracks::album_id.eq(album.map(|album| album.id)),
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
                    track_artists::artist_id.eq(Artist::get(
                        ArtistFilter {
                            name: Some(artist),
                            limit: Some(1),
                            ..ArtistFilter::default()
                        }, true
                    )
                    .unwrap()
                    .value()
                    .id),
                ))
                .execute(&mut conn)?;
        }

        Ok(track)
    }

    pub fn get_album(&self) -> Result<Album> {
        let mut conn = establish_connection();

        if let Some(album_id) = self.album_id {
            let album = albums::table.find(album_id).first(&mut conn)?;
            Ok(album)
        } else {
            Err(Error::msg("Track has no album"))
        }
    }

    pub fn get_artist(&self) -> Result<Vec<Artist>> {
        let mut conn = establish_connection();

        Ok(TrackArtist::belonging_to(&self)
            .inner_join(artists::table)
            .select(artists::all_columns)
            .load::<Artist>(&mut conn)?)
    }

    pub fn check(path: &Path, file_size: i32) -> bool {
        let mut conn = establish_connection();

        if tracks::table
            .select(Track::as_select())
            .filter(tracks::path.eq(path.to_str().unwrap_or_default()))
            .filter(tracks::filesize.eq(file_size))
            .first(&mut conn)
            .is_ok()
        {
            return true;
        }
        false
    }

    pub fn from(value: TrackResponse) -> Self {
        let mut conn = establish_connection();

        tracks::table.find(value.id).first(&mut conn).unwrap()
    }
}
