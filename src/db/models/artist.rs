use super::*;
use crate::establish_connection;
use anyhow::{Error, Result};
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

     pub fn get(mut filter: HashMap<String, String>) -> Result<Vec<Self>> {
        let mut conn = establish_connection()?;

        let mut select = artists::table.select(Artist::as_select()).into_boxed();

        if let Some(title) = filter.remove("title") {
            select = select.filter(artists::name.like(format!("%{title}%")));
        }

        select = select.limit(filter.remove("limit").unwrap_or("50".to_string()).parse()?);
         select = select.offset(filter.remove("offset").unwrap_or("0".to_string()).parse()?);

        select = select
            .distinct()
            .order_by(artists::name);

        let result: Vec<Artist> = select.load(&mut conn)?;
        if !result.is_empty() {
            Ok(result)
        } else {
            Err(Error::msg("Did not find any tracks"))
        }
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
            .filter(artists::name.like(title))
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
