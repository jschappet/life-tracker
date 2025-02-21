use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use crate::crud::{create_reward, get_rewards, get_reward, update_reward, delete_reward};
use crate::state::AppState;
use crate::models::{Reward, NewReward};

#[post("/api/rewards")]
pub async fn create_reward_api<'hb>(
    data: web::Data<AppState<'hb>>,
    new_reward: web::Json<NewReward>,
) -> impl Responder {
    let pool = &data.db_pool;
    let mut conn = pool.get().expect("Failed to get DB connection");
    match create_reward(
        &mut conn,
        new_reward.user_id,
        &new_reward.description,
        new_reward.points,
    ) {
        Ok(reward) => HttpResponse::Ok().json(reward),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create reward"),
    }
}

#[get("/api/rewards")]
pub async fn get_rewards_api<'hb>(data: web::Data<AppState<'hb>>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match get_rewards(conn) {
        Ok(rewards) => HttpResponse::Ok().json(rewards),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve rewards"),
    }
}

#[get("/api/reward/{reward_id}")]
pub async fn get_reward_api<'hb>(data: web::Data<AppState<'hb>>, reward_id: web::Path<i32>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match get_reward(conn, reward_id.into_inner()) {
        Ok(reward) => HttpResponse::Ok().json(reward),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve reward"),
    }
}

#[put("/api/reward/{reward_id}")]
pub async fn update_reward_api<'hb>(
    data: web::Data<AppState<'hb>>,
    reward_id: web::Path<i32>,
    updated_reward: web::Json<Reward>,
) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match update_reward(
        conn,
        reward_id.into_inner(),
        &updated_reward.description,
        updated_reward.points,
    ) {
        Ok(reward) => HttpResponse::Ok().json(reward),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update reward"),
    }
}

#[delete("/api/reward/{reward_id}")]
pub async fn delete_reward_api<'hb>(data: web::Data<AppState<'hb>>, reward_id: web::Path<i32>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    let id = reward_id.into_inner();
    match delete_reward(conn, id) {
        Ok(_) => HttpResponse::Ok().body(format!("Reward {:?} deleted", id)),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete reward"),
    }
}

#[get("/rewards")]
pub async fn get_rewards_page<'hb>(data: web::Data<AppState<'hb>>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match get_rewards(conn) {
        Ok(rewards) => {
            let html = format!(
                r#"{}"#,
                rewards
                    .into_iter()
                    .map(|reward| format!("<li>{}</li>",  reward.description))
                    .collect::<Vec<String>>()
                    .join("")
            );
            HttpResponse::Ok().content_type("text/html").body(html)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve rewards"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_reward_api)
       .service(get_rewards_api)
       .service(get_reward_api)
       .service(update_reward_api)
       .service(delete_reward_api)
       .service(get_rewards_page);
}
