use super::*;

#[derive(Debug, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable)]
#[diesel(table_name = genres)]
pub struct Genre {
    pub(crate) id: i32,
    name: String,
}

impl Genre {
    pub(crate) fn get_or_new(name: String) -> Result<Genre> {
        let mut conn = crate::establish_connection()?;

        if let Ok(genre) = genres::table
            .select(Genre::as_select())
            .filter(genres::name.like(name.clone()))
            .first(&mut conn)
        {
            Ok(genre)
        } else {
            Ok(diesel::insert_into(genres::table)
                .values(genres::name.eq(name))
                .get_result(&mut conn)?)
        }
    }
}
