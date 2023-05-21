use crate::schema::books;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub price: f64,
    pub in_stock: bool,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = books)]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub price: f64,
    pub in_stock: bool,
}
