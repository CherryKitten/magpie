use crate::Result;

use super::*;

#[derive(
    Debug, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable, Identifiable,
)]
#[diesel(belongs_to(Art))]
#[diesel(table_name = artists)]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub art_id: Option<i32>,
}

impl Artist {
    pub fn new(name: &str, conn: &mut SqliteConnection) -> Result<Self> {
        log::debug!("Creating new Artist {name}");

        let artist = diesel::insert_into(artists::table)
            .values(artists::name.eq(name))
            .get_result(conn)?;

        Ok(artist)
    }

    pub fn all(conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        let artist = artists::table
            .select(Artist::as_select())
            .get_results(conn)?;

        Ok(artist)
    }

    pub fn by_id(id: i32, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(artists::table
            .select(Artist::as_select())
            .find(id)
            .first(conn)?)
    }

    pub fn by_title(title: &str, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(artists::table
            .select(Artist::as_select())
            .filter(artists::name.like(format!("%{title}%")))
            .get_result::<Artist>(conn)?)
    }

    pub fn by_title_exact(title: &str, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(artists::table
            .select(Artist::as_select())
            .filter(artists::name.like(title.to_string()))
            .get_result::<Artist>(conn)?)
    }

    pub fn by_title_or_new(title: &str, conn: &mut SqliteConnection) -> Result<Self> {
        let artist = Artist::by_title_exact(title, conn);
        match artist {
            Ok(artist) => Ok(artist),
            Err(_) => Artist::new(title, conn),
        }
    }
}
