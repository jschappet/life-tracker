use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::models::{Task, NewTask};
use crate::schema::tasks::dsl::*;
use log::info;

use chrono::NaiveDate;


pub fn create_task(
    conn: &mut SqliteConnection, 
    task_title: &str, 
    task_description: Option<&str>, 
    task_due_date: Option<NaiveDate>
) -> QueryResult<Task> {
    info!("Creating Task ...");

    use crate::schema::tasks::dsl::*;

    let new_task = NewTask {
        title: task_title.to_string(),
        description: task_description.map(|s| s.to_string()),
        due_date: task_due_date,
    };


    diesel::insert_into(tasks)
        .values(&new_task)
        .execute(conn)?;

    tasks.order(id.desc()).first(conn)
}

pub fn get_tasks(conn: &mut SqliteConnection) -> QueryResult<Vec<Task>> {
    tasks.load::<Task>(conn)
}

pub fn update_task(conn: &mut SqliteConnection, other_task_id: i32, new_title: &str, new_description: Option<&str>, new_due_date: Option<NaiveDate>) -> QueryResult<Task> {
    diesel::update(tasks.find(other_task_id))
        .set((title.eq(new_title), description.eq(new_description), due_date.eq(new_due_date)))
        .execute(conn)?;

    tasks.find(other_task_id).first(conn)
}

pub fn delete_task(conn: &mut SqliteConnection, other_task_id: i32) -> QueryResult<usize> {
    diesel::delete(tasks.find(other_task_id)).execute(conn)
}


