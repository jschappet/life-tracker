use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use crate::crud::{create_user, get_users, get_user, update_user, delete_user};
use crate::state::AppState;
use crate::models::{User, NewUser};

#[post("/users")]
pub async fn create_user_api<'hb>(
    data: web::Data<AppState<'hb>>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    let pool = &data.db_pool;
    let mut conn = pool.get().expect("Failed to get DB connection");
    match create_user(
        &mut conn,
        &new_user.username,
        Some(&new_user.email),
        &new_user.password_hash,
    ) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }
}


#[get("/page/users")]
pub async fn get_users_page<'hb>(data: web::Data<AppState<'hb>>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match get_users(conn) {
        Ok(users) => {
            let html = format!(
                r#"{}"#,
                users
                    .into_iter()
                    .map(|user| format!("<li>{}: {}</li>",
                         user.username, user.email))
                    .collect::<Vec<String>>()
                    .join("")
            );
            HttpResponse::Ok().content_type("text/html").body(html)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve tasks"),
    }
}

#[get("/users")]
pub async fn get_users_api<'hb>(data: web::Data<AppState<'hb>>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match get_users(conn) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve users"),
    }
}

#[get("/user/{user_id}")]
pub async fn get_user_api<'hb>(data: web::Data<AppState<'hb>>, user_id: web::Path<i32>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match get_user(conn, user_id.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve user"),
    }
}

#[put("/user/{user_id}")]
pub async fn update_user_api<'hb>(
    data: web::Data<AppState<'hb>>,
    user_id: web::Path<i32>,
    updated_user: web::Json<User>,
) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match update_user(
        conn,
        user_id.into_inner(),
        &updated_user.password_hash,
        Some(&updated_user.email),
    ) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update user"),
    }
}

#[delete("/user/{user_id}")]
pub async fn delete_user_api<'hb>(data: web::Data<AppState<'hb>>, user_id: web::Path<i32>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    let id = user_id.into_inner();
    match delete_user(conn, id) {
        Ok(_) => HttpResponse::Ok().body(format!("User {:?} deleted", id)),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete user"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user_api)
       .service(get_users_api)
       .service(get_user_api)
       .service(update_user_api)
       .service(delete_user_api)
       .service(get_users_page)
       ;
}