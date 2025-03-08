use actix_web::{ get, post,  web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::crud::{create_tag, get_tags};
use crate::state::AppState;
use crate::auth::get_user_from_request;
use serde_json::json;



#[derive(Serialize, Deserialize)]
struct Tag {
    id: Option<i32>,
    name: String,
}

#[get("/tags")]
async fn get_tags_endpoint(data: web::Data<AppState<'_>>, req: HttpRequest) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    let user = match get_user_from_request(&req, conn) {
        Some(user) => user,
        None => {
            log::error!("Failed to get user");
            return HttpResponse::Unauthorized().body("Unauthorized")
        },
    };
    match get_tags(conn, user.id) {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(err) => {
            log::error!("Error fetching tags: {}", err);
            HttpResponse::InternalServerError().body("Failed to fetch tags")
        }
    }
}


#[get("/page/tags")]
pub async fn get_tags_page<'hb>(data: web::Data<AppState<'hb>>, req: HttpRequest) -> impl Responder {
    let conn= &mut data.db_pool.get().expect("Database connection failed");
    let hb = &data.hb;
    log::debug!("Starting Tags...");
    let user = match get_user_from_request(&req, conn) {
        Some(user) => user,
        None => {
            log::error!("Failed to get user");
            return HttpResponse::Unauthorized().body("Unauthorized")
        },
    };
    log::debug!("User: {:?}", user);

    match get_tags(conn, user.id) {
        Ok(tags) => {
            let data = json!({ "tags": tags, "user": user });
            match hb.render("partials/tags", &data) {
                Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
                Err(err) => {
                    log::error!("{}", err.reason());
                    HttpResponse::InternalServerError().body("Failed to render template")
                },
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve tags"),
    }
}

#[post("/tags")]
async fn create_tag_endpoint(data: web::Data<AppState<'_>>, tag: web::Json<Tag>, req: HttpRequest) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");

    let user = match get_user_from_request(&req, conn) {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().body("Unauthorized"),
    };

    match create_tag(conn, &tag.name, user.id) {
        Ok(new_tag) => HttpResponse::Created().json(new_tag),
        Err(err) => {
            log::error!("Error creating tag: {}", err);
            HttpResponse::InternalServerError().body("Failed to create tag")
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_tags_endpoint);
    cfg.service(create_tag_endpoint);
    cfg.service(get_tags_page);
}
