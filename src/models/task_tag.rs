use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{models::{Tag, Task}, schema::task_tags};

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(Task)]
#[belongs_to(Tag)]
#[table_name = "task_tags"]
pub struct TaskTag {
    pub id: i32,
    pub task_id: i32,
    pub tag_id: i32,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::task_tags)]
pub struct NewTaskTag {
    pub task_id: i32,
    pub tag_id: i32,
}
