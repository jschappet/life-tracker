use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use crate::crud::{create_daily_todo, get_daily_todos_by_user, update_daily_todo, delete_daily_todo};
use crate::state::AppState;
use crate::models::{DailyTodo, NewDailyTodo};

#[post("/daily_todo")]
pub async fn create_daily_todo_api<'hb>(
    data: web::Data<AppState<'hb>>,
    new_daily_todo: web::Json<NewDailyTodo>,
) -> impl Responder {
    let pool = &data.db_pool;
    let mut conn = pool.get().expect("Failed to get DB connection");
    match create_daily_todo(
        &mut conn,
        new_daily_todo.user_id,
        new_daily_todo.task_id,
        new_daily_todo.date,
        new_daily_todo.completed,
    ) {
        Ok(daily_todo) => HttpResponse::Ok().json(daily_todo),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create daily todo"),
    }
}

#[get("/daily_todo/{user_id}")]
pub async fn get_daily_todo_api<'hb>(data: web::Data<AppState<'hb>>, user_id: web::Path<i32>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    log::info!("Getting daily todos for user {:?}", user_id);
    match get_daily_todos_by_user(conn, user_id.into_inner()) {
        Ok(daily_todos) => HttpResponse::Ok().json(daily_todos),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve daily todos"),
    }
}

#[put("/daily_todo/{daily_todo_id}")]
pub async fn update_daily_todo_api<'hb>(
    data: web::Data<AppState<'hb>>,
    daily_todo_id: web::Path<i32>,
    updated_daily_todo: web::Json<DailyTodo>,
) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match update_daily_todo(
        conn,
        daily_todo_id.into_inner(),
        updated_daily_todo.completed,
    ) {
        Ok(daily_todo) => HttpResponse::Ok().json(daily_todo),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update daily todo"),
    }
}

#[delete("/daily_todo/{daily_todo_id}")]
pub async fn delete_daily_todo_api<'hb>(data: web::Data<AppState<'hb>>, daily_todo_id: web::Path<i32>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    let id = daily_todo_id.into_inner();
    match delete_daily_todo(conn, id) {
        Ok(_) => HttpResponse::Ok().body(format!("Daily todo {:?} deleted", id)),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete daily todo"),
    }
}


#[get("/daily_todos/{user_id}")]
pub async fn get_daily_todos_page<'hb>(data: web::Data<AppState<'hb>>, user_id: web::Path<i32>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match get_daily_todos_by_user(conn, user_id.into_inner()) {
        Ok(daily_todos) => {
            // TODO Grab the associated task for this daily_toddo
            let html = format!(
                r#"{}"#,
                daily_todos
                    .into_iter()
                    .map(|todo| format!("<li>{}</li>", todo.date))
                    .collect::<Vec<String>>()
                    .join("")
            );    
            HttpResponse::Ok().content_type("text/html").body(html)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve daily todos"),
    }
}
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_daily_todo_api)
       .service(get_daily_todo_api)
       .service(update_daily_todo_api)
       .service(delete_daily_todo_api)
       .service(get_daily_todos_page)
       ;
}