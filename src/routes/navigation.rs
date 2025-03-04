use actix_web::{get, post, web, HttpResponse, HttpRequest, Responder};
//use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use crate::claims::{Claims, create_jwt};
use crate::crud::search_tasks_by_title;
use crate::{crud, models};
use crate::state::AppState;
use serde_json::json;
use crate::auth::get_user_from_request;


#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/manual_hello")]
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


fn get_dashbard_data<'hb>(user: models::User, conn: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>) -> Option<serde_json::Value> {
    log::debug!("User: {:?}", user);

    // Filter out the tasks that are 'completed'
    let tasks = crud::get_tasks_by_user(conn, user.id).ok()?;
    let active_tasks: Vec<_> = tasks.into_iter().filter(|task| task.status.clone().unwrap() != "completed").collect();
    //let tasks: Vec<_> = tasks.into_iter().filter(|task| task.status.clone().unwrap() == "completed").collect();

    let data = json!({"user": user, 
            "tasks": active_tasks}
        );
    Some(data)
}

fn get_user_from_session<'hb>(session: &actix_session::Session, conn: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>) -> Option<models::User> {
    let username = match session.get::<String>(crate::types::USER_ID_KEY) {
        Ok(Some(username)) => username,
        _ => return None,
    };

    crud::get_user_by_username(conn, username).ok()
}


#[get("/dashboard")]
pub async fn dashboard(data: web::Data<AppState<'_>>, session: actix_session::Session) -> impl Responder {
    let username = match session.get::<String>(crate::types::USER_ID_KEY) {
        Ok(message_option) => {
            match message_option {
                Some(message) => message,
                None => return HttpResponse::Found()
                     .append_header(("Location", "./login"))
                    .finish(),
            }
        }
        Err(_) => return HttpResponse::InternalServerError().body("Error."),
    };

    let hb = &data.hb;
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    let user = match crud::get_user_by_username(conn, username) {
            Ok(user) => user,
            Err(_) => return HttpResponse::Unauthorized().body("Unauthorized"),
        };
    log::debug!("User: {:?}", user);
    let data = get_dashbard_data(user, conn);
    match hb.render("dashboard", &data) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(err) => {
            log::error!("{}", err.reason());
            HttpResponse::InternalServerError().body("Failed to render template")
        },
    }
}

#[get("/")]
async fn redirect_to_index() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/s/"))
        .finish()
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    message: String,
    token: Option<String>,
}

// Convert to template
#[get("/login")]
async fn login_form(data: web::Data<AppState<'_>>) -> HttpResponse {
    let hb = &data.hb;
    let data = json!({});
    match hb.render("login", &data) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(err) => {
            log::error!("{}", err.reason());
            HttpResponse::InternalServerError().body("Failed to render template")
        },
    }
}

#[post("/auth/login")]
async fn handle_login(data: web::Data<AppState<'_>>, form: web::Form<LoginForm>, session: actix_session::Session) -> HttpResponse {
    let conn = &mut data.db_pool.get().expect("Database connection failed");

    match crud::authenticate_user(conn, form.username.clone(), form.password.clone()) {
        Ok(user) => {
            log::debug!("User authenticated: {:?}", user);

            session.renew();
                session
                    .insert(crate::types::USER_ID_KEY, user.username)
                    .expect("`user_id` cannot be inserted into session");
                session
                    .insert(crate::types::USER_EMAIL_KEY, user.email)
                    .expect("`user_email` cannot be inserted into session");
            // Create JWT token with admin permissions
            let claims = Claims::new(
            form.username.clone(),
            vec!["ADMIN".to_string(), "OP_GET_SECURED_INFO".to_string()]
            );

            match create_jwt(claims) {
                Ok(token) => {
                    // let response = LoginResponse {
                    //     message: "Login successful!".to_string(),
                    //     token: Some(token),
                    // };
                    //HttpResponse::Ok().json(response)
                    session
                        .insert(crate::types::JWT_TOKEN, token.clone())
                        .expect("`Token` cannot be inserted into session");
                    HttpResponse::Found()
                        .append_header(("Location", "../dashboard"))
                        .cookie(
                            actix_web::cookie::Cookie::build("jwt_token", token)
                                .path("/")
                                .http_only(false)
                                .finish()
                        )
                        .finish()
                        }
                        Err(_) => {
                            let response = LoginResponse {
                                message: "Error creating token".to_string(),
                                token: None,
                            };
                            HttpResponse::InternalServerError().json(response)
                        }
                    }
        },
        Err(_) => {
            let response = LoginResponse {
                message: "Invalid credentials".to_string(),
                token: None,
            };
            HttpResponse::Unauthorized().json(response)
        }
    }
   
}

#[get("/time-tracker")]
async fn time_tracker(data: web::Data<AppState<'_>>, session: actix_session::Session) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
    // Verify user is logged in
    let username = match session.get::<String>(crate::types::USER_ID_KEY) {
        Ok(Some(username)) => username,
        _ => return HttpResponse::Found()
            .append_header(("Location", "./login"))
            .finish(),
    };

    let user = match crud::get_user_by_username(conn, username.clone()) {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().body("Unauthorized"),
    };

    let hb = &data.hb;


    match hb.render("time-tracker", &get_dashbard_data(user, conn)) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(err) => {
            log::error!("{}", err.reason());
            HttpResponse::InternalServerError().body("Failed to render template")
        },
    }
}

#[derive(Deserialize, Debug)]
struct SubmitTaskData {
    #[serde(rename = "taskInput")]
    title: String,
}

#[derive(Deserialize, Debug)]
struct UpdateTask {
    #[serde(rename = "taskInput")]
    title: String,
    task_id: i32,
    notes: Option<String>,
    status: Option<String>,
}


#[post("/start-task")]
async fn start_task(data: web::Data<AppState<'_>>, req: HttpRequest) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");
   
    if let Some(user) = get_user_from_request(&req, conn) {
        log::debug!("Start Task - Request: {:?}", req);

        // TODO: Implement task starting logic
        HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Task started",
            "user_id": user.id
        }))
    } else {
        HttpResponse::Unauthorized().finish()
    }
}


#[get("/footer")]
async fn footer(data: web::Data<AppState<'_>>) -> impl Responder {
        let hb = &data.hb;

        match hb.render("partials/footer", &json!({"suggestions": []}))
        {
            Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
            Err(err) => {
                log::error!("{}", err.reason());
                HttpResponse::InternalServerError().body("Failed to render template")
            },
        }
}


#[get("/autocomplete")]
async fn autocomplete(data: web::Data<AppState<'_>>, req: HttpRequest, session: actix_session::Session) -> impl Responder {
    //log::debug!("Autocomplete - Query String: {:?}", req);
    
    let conn = &mut data.db_pool.get().expect("Database connection failed");
   
    if let Some(user) = get_user_from_session(&session, conn) {
        let query: String = req.query_string().to_string();
        log::debug!("Query: {}", query);

        // TODO - parse queryString 
        let query = query.split("=").collect::<Vec<&str>>()[1].to_string();
        let query = query.replace("%20", " ");
        //  TODO Create a querytype struct for each autocomplete form
        // TODO match based on querytype
        let hb = &data.hb;

        let data = if query == "" {

             json!({"suggestions": []})
        } else {
            let tasks = search_tasks_by_title(conn, &query, user.id).unwrap_or_else(|_| vec![]);
            log::debug!("Task Count: {}", tasks.len());
            let suggestions: Vec<_> = tasks.into_iter().map(|task| task.title).collect();
            json!({"suggestions": suggestions})
            
        };
        log::debug!("Autocomplete - Suggestions: {:?}", data);
        match hb.render("partials/autocomplete", &data) {
            Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
            Err(err) => {
                log::error!("{}", err.reason());
                HttpResponse::InternalServerError().body("Failed to render template")
            },
        }
    } else {
        log::debug!("Unauthorized access to autocomplete");
        HttpResponse::Unauthorized().finish()
    }
}



#[post("/end-task")]
async fn end_task(data: web::Data<AppState<'_>>, req: HttpRequest) -> impl Responder {
    let conn = &mut data.db_pool.get().expect("Database connection failed");

    if let Some(user) = get_user_from_request(&req, conn) {
        log::debug!("End Task - Request: {:?}", req);
        // TODO: Implement task ending logic
        HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Task ended",
            "user_id": user.id
        }))
    } else {
        HttpResponse::Unauthorized().finish()
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
       .service(echo)
       .service(manual_hello)
       .service(redirect_to_index)
       .service(login_form)
       .service(dashboard)
       .service(handle_login)
       .service(time_tracker)
       .service(start_task)
       .service(end_task)
       .service(autocomplete)
       .service(footer)
       //.service(submit_task)
       //.service(tt_update_task)
       ;  // Ensure the tt_update_task route is added
}


