use super::*;
use crate::establish_connection;
use anyhow::Result;
use std::collections::HashMap;

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

    pub fn into_map(self) -> crate::api::response_container::Map {
        let mut map = HashMap::new();

        map.insert(self.name.unwrap_or_default(), self.id);

        crate::api::response_container::Map::new(map).unwrap_or_default()
    }
}
