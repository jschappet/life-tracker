use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::models::User;

use crate::models::Goal;

// Project Model
#[derive(Queryable, Selectable, Insertable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::projects)]
pub struct Project {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

// Project Model
#[derive(Queryable, Selectable, Insertable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::projects)]
pub struct NewProject {
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

// Project Goals (Join Table)
#[derive(Queryable, Insertable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Project))]
#[diesel(belongs_to(Goal))]
#[diesel(table_name = crate::schema::project_goals)]
pub struct ProjectGoal {
    pub project_id: i32,
    pub goal_id: i32,
}
/* 
use diesel::deserialize::Queryable;
use diesel::sql_types::{Integer, Text, Nullable, Timestamp};


use std::error::Error;

impl Queryable<(Integer, Integer, Text, Nullable<Text>, Timestamp), diesel::sqlite::Sqlite> for Project {
    type Row = (i32, i32, String, Option<String>, NaiveDateTime);

    fn build(row: Self::Row) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Project {
            id: row.0,
            user_id: row.1,
            title: row.2,
            description: row.3,
            created_at: row.4,
        })
    }
}

    */
