use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name =  crate::schema::tasks)]
pub struct Task {
    pub id: i32,    
    pub user_id: i32,
    pub project_id: Option<i32>, // Nullable<Integer> -> Option<i32>
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub created_at: NaiveDateTime,
}


#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<NaiveDate>,
}
