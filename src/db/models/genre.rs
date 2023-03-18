use super::*;

#[derive(
    Debug, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable, Identifiable,
)]
#[diesel(table_name = genres)]
pub struct Genre {
    pub(crate) id: i32,
    name: String,
}

impl Genre {
    pub fn get_or_new(name: &str, conn: &mut SqliteConnection) -> Result<Genre> {
        if let Ok(genre) = genres::table
            .select(Genre::as_select())
            .filter(genres::name.like(name))
            .first(conn)
        {
            Ok(genre)
        } else {
            Ok(diesel::insert_into(genres::table)
                .values(genres::name.eq(name))
                .get_result(conn)?)
        }
    }

    pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<Self>> {
        Ok(genres::table
            .select(genres::all_columns)
            .get_results(conn)?)
    }

    pub fn get_by_id(id: i32, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(genres::table.find(id).first(conn)?)
    }

    pub fn get_by_title(title: &str, conn: &mut SqliteConnection) -> Result<Self> {
        Ok(genres::table
            .select(genres::all_columns)
            .filter(genres::name.like(title))
            .first::<Genre>(conn)?)
    }
}
