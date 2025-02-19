use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use crate::crud::{create_goal, get_goal_by_id, get_goals_by_user, update_goal, delete_goal};
use crate::state::AppState;
use crate::models::{Goal, NewGoal};

#[post("/api/goals")]
pub async fn create_goal_api(
    data: web::Data<AppState>,
    new_goal: web::Json<NewGoal>,
) -> impl Responder {
    let pool = &data.db_pool;
    let mut conn = pool.get().expect("Failed to get DB connection");
    match create_goal(
        &mut conn,
        new_goal.user_id,
        &new_goal.title,
        new_goal.description.as_deref(),
        new_goal.due_date,
        new_goal.status.as_deref().unwrap(),
    ) {
        Ok(goal) => HttpResponse::Ok().json(goal),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create goal"),
    }
}

#[get("/api/goal/{goal_id}")]
pub async fn get_goal_by_id_api(data: web::Data<AppState>, goal_id: web::Path<i32>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match get_goal_by_id(conn, goal_id.into_inner()) {
        Ok(goal) => HttpResponse::Ok().json(goal),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve goal"),
    }
}

#[get("/api/goals/{user_id}")]
pub async fn get_goals_api(data: web::Data<AppState>, user_id: web::Path<i32>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match get_goals_by_user(conn, user_id.into_inner()) {
        Ok(goals) => HttpResponse::Ok().json(goals),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve goals"),
    }
}

#[put("/api/goal/{goal_id}")]
pub async fn update_goal_api(
    data: web::Data<AppState>,
    goal_id: web::Path<i32>,
    updated_goal: web::Json<Goal>,
) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match update_goal(
        conn,
        goal_id.into_inner(),
        Some(&updated_goal.title),
        updated_goal.description.as_deref(),
        updated_goal.due_date,
        updated_goal.status.as_deref(),
    ) {
        Ok(goal) => HttpResponse::Ok().json(goal),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update goal"),
    }
}

#[delete("/api/goal/{goal_id}")]
pub async fn delete_goal_api(data: web::Data<AppState>, goal_id: web::Path<i32>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    let id = goal_id.into_inner();
    match delete_goal(conn, id) {
        Ok(_) => HttpResponse::Ok().body(format!("Goal {:?} deleted", id)),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete goal"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_goal_api)
       .service(get_goals_api)
       .service(update_goal_api)
       .service(delete_goal_api);
}