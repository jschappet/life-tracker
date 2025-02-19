use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::daily_todos)]
pub struct DailyTodo {
    pub id: i32,
    pub user_id: i32,
    pub task_id: i32,
    pub date: NaiveDate,
    pub completed: Option<bool>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::daily_todos)]
pub struct NewDailyTodo {
    pub user_id: i32,
    pub task_id: i32,
    pub date: NaiveDate,
    pub completed: Option<bool>,
}