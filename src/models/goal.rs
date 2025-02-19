use diesel::prelude::*;
use serde::{Deserialize, Serialize};
//use chrono::{NaiveDate, NaiveDateTime};
use crate::models::User;

// Goal Model
#[derive(Queryable, Selectable, Insertable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::goals)]
pub struct Goal {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<chrono::NaiveDate>,
    pub status: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::goals)]
pub struct NewGoal {
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<chrono::NaiveDate>,
    pub status: Option<String>,
}