use diesel::prelude::*;
use crate::models::Goal;
use chrono::NaiveDate;
use crate::schema::goals::dsl::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

// Create a new goal
pub fn create_goal(conn: &mut SqliteConnection, goal_user_id: i32, goal_title: &str, goal_description: Option<&str>, goal_due_date: Option<NaiveDate>, goal_status: &str) -> Result<Goal, DbError> {
    let new_goal = Goal {
        id: 0, // Diesel will auto-assign ID
        user_id: goal_user_id,
        title: goal_title.to_string(),
        description: goal_description.map(|d| d.to_string()),
        due_date: goal_due_date,
        status: Some(goal_status.to_string()),
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(goals)
        .values(&new_goal)
        .execute(conn)?;

    Ok(new_goal)
}

pub fn get_goals_by_user(conn: &mut SqliteConnection, user_id_filter: i32) -> QueryResult<Vec<Goal>> {
    goals.filter(user_id.eq(user_id_filter)).load::<Goal>(conn)
}


pub fn get_goal_by_id(conn: &mut SqliteConnection, goal_id: i32) -> Result<Goal, DbError>  {
    goals
        .filter(id.eq(goal_id))
        .first::<Goal>(conn)
        .map_err(|e| e.into())
}

// Update a goal
pub fn update_goal(
    conn: &mut SqliteConnection,
    goal_id: i32,
    new_title: Option<&str>,
    new_description: Option<&str>,
    new_due_date: Option<NaiveDate>,
    new_status: Option<&str>,
) -> Result<Goal, DbError> {
    use crate::schema::goals::dsl::*;

    diesel::update(goals.filter(id.eq(goal_id)))
        .set((
            new_title.map(|t| title.eq(t)),
            new_description.map(|d| description.eq(d)),
            new_due_date.map(|d| due_date.eq(d)),
            new_status.map(|s| status.eq(s)),
        ))
        .execute(conn)?;

    get_goal_by_id(conn, goal_id)
}

// Delete a goal
pub fn delete_goal(conn: &mut SqliteConnection, goal_id: i32) -> Result<usize, DbError> {
    use crate::schema::goals::dsl::*;
    diesel::delete(goals.filter(id.eq(goal_id))).execute(conn).map_err(|e| e.into())
}
