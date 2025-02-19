use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::models::{Project, ProjectGoal};
use crate::schema::{projects::dsl::*, project_goals::dsl::*};
use chrono::{NaiveDateTime, Utc};

pub fn create_project(
    conn: &mut SqliteConnection,
    project_title: &str,
    project_description: Option<&str>,
    //proj_created_at: NaiveDateTime,
    user_id_val: i32,
) -> QueryResult<Project> {
    let now: NaiveDateTime = Utc::now().naive_utc();
    diesel::insert_into(projects)
        .values((
            user_id.eq(user_id_val),
            title.eq(project_title),
            description.eq(project_description),
            created_at.eq(now),
        ))
        .execute(conn)?;
    
    projects.order(id.desc()).first::<Project>(conn)
}

pub fn get_projects(conn: &mut SqliteConnection) -> QueryResult<Vec<Project>> {
    projects.load::<Project>(conn)
}

pub fn update_project(
    conn: &mut SqliteConnection,
    project_id_val: i32,
    new_title: &str,
    new_description: Option<&str>
) -> QueryResult<Project> {
    diesel::update(projects.find(project_id_val))
        .set((title.eq(new_title), description.eq(new_description)))
        .execute(conn)?;
    
    projects.find(project_id_val).first::<Project>(conn)
}

pub fn delete_project(conn: &mut SqliteConnection, project_id_val: i32) -> QueryResult<usize> {
    diesel::delete(projects.find(project_id_val)).execute(conn)
}

pub fn create_project_goal(conn: &mut SqliteConnection, project_id_val: i32, goal_id_val: i32) -> QueryResult<ProjectGoal> {
    let new_project_goal = ProjectGoal { project_id: project_id_val, goal_id: goal_id_val };
    
    diesel::insert_into(project_goals)
        .values(&new_project_goal)
        .execute(conn)?;
    
    project_goals.filter(project_id.eq(project_id_val)).order(goal_id.desc()).first(conn)
}

pub fn get_project_goals(conn: &mut SqliteConnection, project_id_filter: i32) -> QueryResult<Vec<ProjectGoal>> {
    project_goals.filter(project_id.eq(project_id_filter)).load::<ProjectGoal>(conn)
}

pub fn delete_project_goal(conn: &mut SqliteConnection, project_id_filter: i32, goal_id_filter: i32) -> QueryResult<usize> {
    diesel::delete(
        project_goals.filter(project_id.eq(project_id_filter).and(goal_id.eq(goal_id_filter)))
    ).execute(conn)
}
