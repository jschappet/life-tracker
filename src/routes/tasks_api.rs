extern crate serde;

use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use crate::crud::tags::{add_tags_by_task, get_tags_by_task};
use crate::models::{NewTask, Task};
use crate::crud::{create_task, delete_task, get_tasks, update_task, update_task_without_title};
use crate::state::AppState;
use crate::types::TaskStatus;
use serde_json::json;
use crate::auth::get_user_from_request;
use serde::Deserialize;
use chrono::{Utc, Duration};

#[derive(Deserialize, Debug, Clone)]
struct SubmitTaskData {
    #[serde(rename = "taskInput")]
    title: String,
    status: String,
    tags: Vec<i32>,
    // Optional fields can be added later if needed
}

// Create struct for UpdateTask
#[derive(Deserialize, Debug)]
struct UpdateTask {
    #[serde(rename = "taskInput")]
    title: Option<String>,
    task_id: i32,
    notes: Option<String>,
    status: Option<String>,
}

#[post("/tt-update-task")]
async fn tt_update_task(
    data: web::Data<AppState<'_>>, 
    req: HttpRequest,    
    new_task: web::Json<UpdateTask>,
) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    
    if let Some(user) = get_user_from_request(&req, conn) {
        //log::debug!("Update Task - Request: {:?}", req);
        log::debug!("Update Task - Request: {:?}", new_task.title);
        log::debug!("Update Task - user: {:?}", user);
        let ts = TaskStatus::from_str(new_task.status.clone().unwrap().as_str());
        match update_task_without_title(
            conn,
            new_task.task_id,
            new_task.notes.as_deref(),
            None, // Assuming no due date update
            ts.unwrap(),
        ) {
            Ok(task) => HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Task updated",
                "task": task
            })),
            Err(_) => HttpResponse::InternalServerError().body("Failed to update task"),
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }   
}

#[post("/tasks")]
pub async fn create_task_api<'hb>(
    data: web::Data<AppState<'hb>>,
    new_task: web::Json<NewTask>,
) -> impl Responder {
    log::debug!("create task api: started");
    let pool = &data.db_pool;
    let mut conn = pool.get().expect("Failed to get DB connection");
    let ts = TaskStatus::from_str(new_task.status.as_ref().unwrap().as_str());

    match create_task(
        &mut conn,
        &new_task.title,
        new_task.description.as_deref(),
        new_task.due_date,
        new_task.user_id,
        ts,
    ) {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create task"),
    }
}

#[post("/submit-task")]
async fn submit_task(
    data: web::Data<AppState<'_>>, 
    req: HttpRequest,
    task_data: web::Json<SubmitTaskData>
) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");

    if let Some(user) = get_user_from_request(&req, conn) {
        log::debug!("Submit Task - Request data: {:?}", task_data.clone());
        
        let task_due_date = Utc::now().naive_utc().date() + Duration::days(1);
        let ts = TaskStatus::from_str(&task_data.status);
        // TODO: fix compile error
        let tags = task_data.clone().tags;
       
        match create_task(conn, &task_data.title, Some(""), Some(task_due_date), user.id, ts) {
            Ok(task) => {
                log::debug!("Adding Tags...");
                match add_tags_by_task(conn, task.id, tags) {
                    Ok(tags) => HttpResponse::Ok().json(json!({
                        "status": "success",
                        "message": "Task submitted",
                        "task": {
                            "title": task_data.title,
                            "user_id": user.id,
                            "task_id": task.id,
                            "description": task.description,
                            "tags": tags,
                        }
                    })),
                    Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve tags"),
                }
            },
            Err(_) => HttpResponse::InternalServerError().body("Failed to create task"),
        }
    } else {
        log::debug!("Unauthorized access attempt: {:?}", req);
        HttpResponse::Unauthorized().finish()
    }
}

#[get("/tasks")]
pub async fn get_tasks_api<'hb>(data: web::Data<AppState<'hb>>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match get_tasks(conn) {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve tasks"),
    }
}

#[get("/page/tasks")]
pub async fn get_tasks_page<'hb>(data: web::Data<AppState<'hb>>, req: HttpRequest) -> impl Responder {
    let conn= &mut data.db_pool.get().expect("Database connection failed");
    let hb = &data.hb;

    let user = match get_user_from_request(&req, conn) {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().body("Unauthorized"),
    };
    log::debug!("User: {:?}", user);

    match get_tasks(conn) {
        Ok(tasks) => {
            let data = json!({ "tasks": tasks, "user": user });
            match hb.render("partials/tasks", &data) {
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

#[put("/task/{task_id}")]
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
        TaskStatus::Pending
    ) {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update task"),
    }
}

#[delete("/task/{task_id}")]
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
       .service(delete_task_api)
       .service(submit_task)
       .service(tt_update_task);  // Ensure the tt_update_task route is added
}