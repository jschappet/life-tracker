use diesel::prelude::*;
use serde::{Deserialize, Serialize};
//use chrono::{NaiveDate, NaiveDateTime};

// User Model
#[derive(Debug, Queryable, Selectable, Insertable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Debug, Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,

}