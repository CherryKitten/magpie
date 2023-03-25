use super::*;

use anyhow::{Error, Result};
use duplicate::duplicate;
use std::collections::HashMap;

#[derive(
    Debug, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable, Identifiable,
)]
#[diesel(table_name = artists)]
pub struct Artist {
    pub id: i32,
    pub name: String,
}

impl Artist {
    pub fn new(name: &str, conn: &mut SqliteConnection) -> Result<Self> {
        diesel::insert_into(artists::table)
            .values(artists::name.eq(name))
            .execute(conn)?;

        Artist::get_by_title(name, conn)
    }

    pub fn get(
        mut filter: HashMap<String, String>,
        conn: &mut SqliteConnection,
    ) -> Result<Vec<Self>> {
        let mut select = artists::table.select(Artist::as_select()).into_boxed();

        if !filter.is_empty() {
            duplicate! {
                [
                    key statement;
                    [ "title" ] [ artists::name.like(format!("%{item}%")) ];
                    [ "name" ]  [ artists::name.like(format!("%{item}%")) ];
                ]
                if let Some(item) = filter.remove(key) {
                select = select.filter(statement);
            }}
        }

        select = select.limit(filter.remove("limit").unwrap_or("50".to_string()).parse()?);
        select = select.offset(filter.remove("offset").unwrap_or("0".to_string()).parse()?);

        select = select.distinct().order_by(artists::name);

        let result: Vec<Artist> = select.load(conn)?;
        if !result.is_empty() {
            Ok(result)
        } else {
            Err(Error::msg("Did not find any tracks"))
        }
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

    pub fn into_map(self) -> crate::api::response_container::Map {
        let mut map = HashMap::new();

        map.insert(self.name, self.id);

        crate::api::response_container::Map::new(map).unwrap_or_default()
    }
}
