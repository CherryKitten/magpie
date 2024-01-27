use super::*;

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
