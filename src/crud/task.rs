use diesel::prelude::*;
use diesel::row::NamedRow;
use diesel::sqlite::SqliteConnection;
use crate::models::{task, NewTask, Task};
use crate::schema::tasks::dsl::*;
use log::info;

use chrono::{NaiveDate, NaiveDateTime, Utc};




pub fn create_task(
    conn: &mut SqliteConnection, 
    task_title: &str, 
    task_description: Option<&str>, 
    task_due_date: Option<NaiveDate>,
    task_user_id: i32,
    new_status: Option<String>,
) -> QueryResult<Task> {
    info!("Creating Task ...");

    use crate::schema::tasks::dsl::*;
    

    // TODO -  fix this
    let start_time_val = match new_status.as_deref() {
        Some("in_progress") => Some(Utc::now().naive_utc()),
        _ => None,
    };


    let new_task = NewTask {
        title: task_title.to_string(),
        user_id: task_user_id,
        description: task_description.map(|s| s.to_string()),
        due_date: task_due_date,
        status: new_status, 
        project_id: None,
        start_time: start_time_val,
        end_time: None,
    };


    diesel::insert_into(tasks)
        .values(&new_task)
        .execute(conn)?;

    tasks.order(id.desc()).first(conn)
}

pub fn get_tasks(conn: &mut SqliteConnection) -> QueryResult<Vec<Task>> {
    tasks.load::<Task>(conn)
}

pub fn get_tasks_by_user(conn: &mut SqliteConnection, other_user_id: i32) -> QueryResult<Vec<Task>> {
    tasks.filter(user_id.eq(other_user_id)).load::<Task>(conn)
}

pub fn get_task(conn: &mut SqliteConnection, other_task_id: i32) -> QueryResult<Task> {
    tasks.filter(id.eq(other_task_id)).first::<Task>(conn)
}

pub fn update_task_without_title(conn: &mut SqliteConnection, other_task_id: i32, new_description: Option<&str>, new_due_date: Option<NaiveDate>, new_status: Option<&str>) -> QueryResult<Task> {
    // Get the current date and time
    let date_now: NaiveDateTime = Utc::now().naive_utc();

    match new_status {
        Some("completed") => {
            
            diesel::update(tasks.find(other_task_id))
            .set(
                (
                    description.eq(new_description),
                    due_date.eq(new_due_date),
                    status.eq(new_status),
                    end_time.eq(date_now),
            )
            )
            .execute(conn)?;
        },

        Some("in_progress") => {
            log::debug!("Taask Update: {} {:?}",    other_task_id, new_status);
            diesel::update(tasks.find(other_task_id))
            .set(
                (
                    description.eq(new_description),
                    due_date.eq(new_due_date),
                    status.eq(new_status),
                    start_time.eq(date_now),
            )
            )
            .execute(conn)?;
        },
        _ => {
            log::debug!("Taask Update: {} {:?}",    other_task_id, new_status);
            diesel::update(tasks.find(other_task_id))
            .set(
                (
                    description.eq(new_description),
                    due_date.eq(new_due_date),
                    status.eq(new_status),
            )
            )
            .execute(conn)?;
        }
        
    };

    tasks.find(other_task_id).first(conn)
}

pub fn update_task(conn: &mut SqliteConnection, other_task_id: i32, new_title: &str, new_description: Option<&str>, new_due_date: Option<NaiveDate>, new_status: Option<&str>) -> QueryResult<Task> {
    diesel::update(tasks.find(other_task_id))
        .set(
            (
                title.eq(new_title), 
                description.eq(new_description),
                due_date.eq(new_due_date),
                status.eq(new_status),
        )
        )
        .execute(conn)?;

    tasks.find(other_task_id).first(conn)
}

pub fn delete_task(conn: &mut SqliteConnection, other_task_id: i32) -> QueryResult<usize> {
    diesel::delete(tasks.find(other_task_id)).execute(conn)
}

pub fn search_tasks_by_title(
    conn: &mut SqliteConnection,
    query_string: &str,
    usr_id: i32,
) -> QueryResult<Vec<Task>> {
    log::debug!("Searching for tasks with title containing: {}", query_string);
    tasks
        .filter(title.not_like(""))
        .filter(title.like(format!("%{}%", query_string)))
        .filter(user_id.eq(usr_id))
        .order_by(title.asc())
        .load::<Task>(conn)
}
// SELECT * FROM tasks WHERE title LIKE '%asdf%' AND user_id = 4;