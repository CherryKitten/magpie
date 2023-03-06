use super::*;
use crate::establish_connection;

#[derive(
    Debug, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable, Identifiable,
)]
#[diesel(table_name = genres)]
pub struct Genre {
    pub(crate) id: i32,
    name: String,
}

impl Genre {
    pub fn get_or_new(name: &str) -> Result<Genre> {
        let mut conn = establish_connection()?;

        if let Ok(genre) = genres::table
            .select(Genre::as_select())
            .filter(genres::name.like(name))
            .first(&mut conn)
        {
            Ok(genre)
        } else {
            Ok(diesel::insert_into(genres::table)
                .values(genres::name.eq(name))
                .get_result(&mut conn)?)
        }
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let mut conn = establish_connection()?;

        Ok(genres::table
            .select(genres::all_columns)
            .get_results(&mut conn)?)
    }

    pub fn get_by_id(id: i32) -> Result<Self> {
        let mut conn = establish_connection()?;

        Ok(genres::table.find(id).first(&mut conn)?)
    }

    pub fn get_by_title(title: &str) -> Result<Self> {
        let mut conn = establish_connection()?;
        Ok(genres::table
            .select(genres::all_columns)
            .filter(genres::name.like(title))
            .first::<Genre>(&mut conn)?)
    }
}
