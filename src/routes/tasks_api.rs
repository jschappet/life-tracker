use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use crate::models::{NewTask, Task};
use crate::crud::{create_task, get_tasks, update_task, delete_task};
use crate::state::AppState;
use serde_json::json;

#[post("/api/tasks")]
pub async fn create_task_api<'hb>(
    data: web::Data<AppState<'hb>>,
    new_task: web::Json<NewTask>,
) -> impl Responder {
    let pool = &data.db_pool;
    let mut conn = pool.get().expect("Failed to get DB connection");
    match create_task(
        &mut conn,
        &new_task.title,
        new_task.description.as_deref(),
        new_task.due_date,
    ) {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create task"),
    }
}

#[get("/api/tasks")]
pub async fn get_tasks_api<'hb>(data: web::Data<AppState<'hb>>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match get_tasks(conn) {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve tasks"),
    }
}

#[get("/tasks")]
pub async fn get_tasks_page<'hb>(data: web::Data<AppState<'hb>>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    let hb = &data.hb;

    match get_tasks(conn) {
        Ok(tasks) => {
            let data = json!({ "tasks": tasks });
            match hb.render("partials/daily_tasks", &data) {
                Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
                Err(err) => {
                    log::error!("{}", err.reason());
                    HttpResponse::InternalServerError().body("Failed to render template")
                },
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve tasks"),
    }
}

#[put("/api/task/{task_id}")]
pub async fn update_task_api<'hb>(
    data: web::Data<AppState<'hb>>,
    task_id: web::Path<i32>,
    updated_task: web::Json<Task>,
) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match update_task(
        conn,
        task_id.into_inner(),
        &updated_task.title,
        updated_task.description.as_deref(),
        updated_task.due_date,
    ) {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update task"),
    }
}

#[delete("/api/task/{task_id}")]
pub async fn delete_task_api<'hb>(data: web::Data<AppState<'hb>>, task_id: web::Path<i32>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    let new_task_id: i32 = task_id.into_inner();
    match delete_task(conn, new_task_id) {
        Ok(_) => HttpResponse::Ok().body(format!("Task {:?} deleted", new_task_id)),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete task"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_task_api)
       .service(get_tasks_api)
       .service(update_task_api)
       .service(get_tasks_page)
       .service(delete_task_api);
}