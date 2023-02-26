use super::*;
use crate::establish_connection;
use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable)]
#[diesel(table_name = artists)]
pub struct Artist {
    pub id: i32,
    pub name: Option<String>,
}

impl Artist {
    pub fn new(name: String) -> Result<Self> {
        let mut conn = establish_connection()?;

        diesel::insert_into(artists::table)
            .values(artists::name.eq(name.clone()))
            .execute(&mut conn)?;

        Artist::get_by_name(name)
    }

    pub fn get_by_name(name: String) -> Result<Self> {
        let mut conn = establish_connection().unwrap();

        if let Ok(artist) = artists::table
            .select(Artist::as_select())
            .filter(artists::name.like(name))
            .first(&mut conn)
        {
            Ok(artist)
        } else {
            Err(anyhow::Error::msg("Could not get Artist"))
        }
    }

    pub fn get_by_name_or_new(name: String) -> Result<Self> {
        let artist = Artist::get_by_name(name.clone());
        match artist {
            Ok(artist) => Ok(artist),
            Err(_) => Artist::new(name),
        }
    }
}
