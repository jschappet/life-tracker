use serde::{Deserialize, Serialize};
use diesel::prelude::*;
//use diesel::sql_types::{Integer, Text, Nullable};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tags)]
pub struct Tag {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub icon_url: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tags)]
pub struct NewTag {
    pub name: String,
    pub user_id: i32,
    pub icon_url: Option<String>,
}
