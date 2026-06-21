use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::inventory)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Inventory {
    pub id: i32,
    pub item_name: String,
    pub current_stock: i32
}