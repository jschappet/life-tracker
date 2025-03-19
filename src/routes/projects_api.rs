use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use crate::crud::{create_project, delete_project, get_project, get_projects, update_project};
use crate::state::AppState;
use crate::models::{Project, NewProject};

#[post("/projects")]
pub async fn create_project_api<'hb>(
    data: web::Data<AppState<'hb>>,
    new_project: web::Json<NewProject>,
) -> impl Responder {
    let pool = &data.db_pool;
    let mut conn = pool.get().expect("Failed to get DB connection");
    match create_project(
        &mut conn,
        &new_project.title,
        new_project.description.as_deref(),
        new_project.user_id,
    ) {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create project"),
    }
}

#[get("/projects")]
pub async fn get_projects_api<'hb>(data: web::Data<AppState<'hb>>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match get_projects(conn) {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve projects"),
    }
}

#[put("/project/{project_id}")]
pub async fn update_project_api<'hb>(
    data: web::Data<AppState<'hb>>,
    project_id: web::Path<i32>,
    updated_project: web::Json<Project>,
) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match update_project(
        conn,
        project_id.into_inner(),
        &updated_project.title,
        updated_project.description.as_deref(),
    ) {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update project"),
    }
}

#[delete("/project/{project_id}")]
pub async fn delete_project_api<'hb>(data: web::Data<AppState<'hb>>, project_id: web::Path<i32>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    let id = project_id.into_inner();
    match delete_project(conn, id) {
        Ok(_) => HttpResponse::Ok().body(format!("Project {:?} deleted", id)),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete project"),
    }
}

#[get("/page/projects")]
pub async fn get_projects_page<'hb>(data: web::Data<AppState<'hb>>) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    match get_projects(conn) {
        Ok(projects) => {
            let html = format!(
                r#"{}"#,
                projects
                    .into_iter()
                    .map(|project| format!("<li>{}: {}</li>", project.title, project.description.unwrap_or("None".to_string())))
                    .collect::<Vec<String>>()
                    .join("")
            );
            HttpResponse::Ok().content_type("text/html").body(html)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve projects"),
    }
}


#[get("/page/project/{project_id}")]
pub async fn get_project_page<'hb>(
    data: web::Data<AppState<'hb>>, 
    project_id: web::Path<i32>,
) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    log::debug!("Gettting Project: {}", project_id.clone());
    match get_project(conn, project_id.into_inner()) {
        Ok(project) => {
            let json_data = json!({"project": project});
            
            match data.hb.render("partials/project", &json_data) {
                Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
                Err(err) => {
                    log::error!("{}", err.reason());
                    HttpResponse::InternalServerError().body("Failed to render template")
                },
            }
        }
        Err(e) => {
            log::debug!("failed to retrieve projects: {}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve projects")
        },
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_project_api)
       .service(get_projects_api)
       .service(update_project_api)
       .service(delete_project_api)
       .service(get_projects_page)
       .service(get_project_page);
}