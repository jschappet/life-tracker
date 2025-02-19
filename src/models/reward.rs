use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::rewards)]
pub struct Reward {
    pub id: i32,
    pub user_id: i32,
    pub description: String,
    pub points: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable,  Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::rewards)]
pub struct NewReward {
    pub user_id: i32,
    pub description: String,
    pub points: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}
