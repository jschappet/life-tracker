use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::models::{DailyTodo, NewDailyTodo};
use crate::schema::daily_todos::dsl::*;
//type DbError = Box<dyn std::error::Error + Send + Sync>;

use chrono::NaiveDate;


// Create
pub fn create_daily_todo(
    conn: &mut SqliteConnection,
    user_id_val: i32,
    task_id_val: i32,
    date_val: NaiveDate,
    completed_val: Option<bool>,
) -> QueryResult<DailyTodo> {

    let new_daily_todo = NewDailyTodo {
        user_id: user_id_val,
        task_id: task_id_val,
        date: date_val,
        completed: completed_val,
    };


    diesel::insert_into(daily_todos)
    .values(&new_daily_todo)
    .execute(conn)?;

    daily_todos.order(id.desc()).first::<DailyTodo>(conn)

}

// Read
pub fn get_daily_todos(conn: &mut SqliteConnection) -> QueryResult<Vec<DailyTodo>> {
    daily_todos.load::<DailyTodo>(conn)
}

// Read by Useer
pub fn get_daily_todos_by_user(conn: &mut SqliteConnection, user_id_filter: i32) -> QueryResult<Vec<DailyTodo>> {
    daily_todos.filter(user_id.eq(user_id_filter)).load::<DailyTodo>(conn)
}

// Update
pub fn update_daily_todo(
    conn: &mut SqliteConnection,
    todo_id: i32,
    new_completed: Option<bool>,
) -> QueryResult<DailyTodo> {
    diesel::update(daily_todos.find(todo_id))
        .set(completed.eq(new_completed))
        .execute(conn)?;
    daily_todos.order(id.desc()).first::<DailyTodo>(conn)
}

// Delete
pub fn delete_daily_todo(conn: &mut SqliteConnection, todo_id: i32) -> QueryResult<()> {
    diesel::delete(daily_todos.find(todo_id)).execute(conn)?;
    Ok(())
}