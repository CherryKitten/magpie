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
    pub title: String,
    pub year: Option<i32>,
    pub release_date: Option<String>,
    pub length: Option<i32>,
    pub language: Option<String>,
    pub added_at: Option<String>,
    pub art_id: Option<i32>,
}
