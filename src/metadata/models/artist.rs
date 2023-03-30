use super::*;

use anyhow::Result;

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
        diesel::insert_into(artists::table)
            .values(artists::name.eq(name))
            .execute(conn)?;

        Artist::get_by_title(name, conn)
    }

    pub fn all(conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(artists::table
            .select(Artist::as_select())
            .get_results(conn)?)
    }

    pub fn get_by_id(id: i32, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(artists::table
            .select(Artist::as_select())
            .find(id)
            .first(conn)?)
    }

    pub fn get_by_title(title: &str, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(artists::table
            .select(Artist::as_select())
            .filter(artists::name.like(format!("%{title}%")))
            .get_result::<Artist>(conn)?)
    }

    pub fn get_by_title_or_new(title: &str, conn: &mut SqliteConnection) -> Result<Self> {
        let artist = Artist::get_by_title(title, conn);
        match artist {
            Ok(artist) => Ok(artist),
            Err(_) => Artist::new(title, conn),
        }
    }
}
