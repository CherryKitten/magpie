use super::*;
use crate::establish_connection;
use anyhow::Result;

#[derive(
    Debug, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable, Identifiable,
)]
#[diesel(table_name = artists)]
pub struct Artist {
    pub id: i32,
    pub name: Option<String>,
}

impl Artist {
    pub fn new(name: &str) -> Result<Self> {
        let mut conn = establish_connection()?;

        diesel::insert_into(artists::table)
            .values(artists::name.eq(name))
            .execute(&mut conn)?;

        Artist::get_by_title(name)
    }

    pub fn all() -> Result<Vec<Self>> {
        let mut conn = establish_connection()?;

        Ok(artists::table
            .select(artists::all_columns)
            .get_results(&mut conn)?)
    }

    pub fn get_by_id(id: i32) -> Result<Self> {
        let mut conn = establish_connection()?;

        Ok(artists::table.find(id).first(&mut conn)?)
    }

    pub fn get_by_title(title: &str) -> Result<Self> {
        let mut conn = establish_connection()?;
        Ok(artists::table
            .select(artists::all_columns)
            .filter(artists::name.eq(title))
            .get_result::<Artist>(&mut conn)?)
    }

    pub fn get_by_title_or_new(title: &str) -> Result<Self> {
        let artist = Artist::get_by_title(title);
        match artist {
            Ok(artist) => Ok(artist),
            Err(_) => Artist::new(title),
        }
    }
}
