use super::*;

#[derive(
    Debug, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable, Identifiable,
)]
#[diesel(table_name = genres)]
pub struct Genre {
    pub(crate) id: i32,
    name: String,
}
