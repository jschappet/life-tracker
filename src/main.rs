//mod routes;
mod middleware;

mod claims;

use actix_web::dev::ServiceRequest;
use actix_web::{web, App, Error, HttpServer, HttpResponse, middleware as mw};

use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::bearer::BearerAuth;

// Used for integration with `actix-web-httpauth`
use actix_web_grants::authorities::AttachAuthorities;

use actix_web_httpauth::middleware::HttpAuthentication;
use life_tracker::routes;
use life_tracker::state::AppState;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use std::env;
use dotenvy;
use actix_files as fs;
use env_logger::Env;
use handlebars::{DirectorySourceOptions, Handlebars};
use std::sync::Arc;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

async fn run_migrations(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // We just get permissions from JWT
    log::debug!("validator {:?}", req);
    let result = claims::decode_jwt(credentials.token());
    match result {
        Ok(claims) => {
            log::debug!("validator: {:?}", claims);
            req.attach(claims.permissions);
            Ok(req)
        }
        // required by `actix-web-httpauth` validator signature
        Err(e) => Err((e, req)),
    }
}

async fn basic_validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // Basic auth example - validate username/password
    if credentials.user_id() == "admin" && credentials.password().unwrap_or_default() == "secret" {
        req.attach(vec!["ADMIN".to_string()]);
        Ok(req)
    } else {
        Err((Error::from(actix_web::error::ErrorUnauthorized("Invalid credentials")), req))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enable logging system-wide
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    log::debug!("Logging initialized.");

    dotenvy::dotenv().ok();
    log::debug!("Environment variables loaded.");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::debug!("Database URL: {}", database_url);

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: r2d2::Pool<ConnectionManager<SqliteConnection>> = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    log::info!("Database connection pool created.");

    // Run migrations
    {
        let mut conn = pool.get().expect("Failed to get DB connection");
        if let Err(e) = run_migrations(&mut conn).await {
            log::error!("Failed to run migrations: {}", e);
            std::process::exit(1);
        }
    }

    let dso = DirectorySourceOptions::default();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory("./templates/", dso)
        .expect("Failed to register templates directory");

    log::debug!("Hb: {}", handlebars.get_templates().keys().count());

    // Log all registered templates
    for template_name in handlebars.get_templates().keys() {
        log::debug!("Registered template: {}", template_name);
    }
    let handlebars = Arc::new(handlebars);

    let app_state = AppState {
        db_pool: pool.clone(),
        omdb_api_key: "foo".to_string(),
        hb: handlebars.clone(),
    };

    log::info!("App state initialized.");
    //let basic_auth = HttpAuthentication::basic(basic_validator);
    let secret_key = actix_web::cookie::Key::from("SECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDFSECRETKAJSDKAJSDF".as_bytes());

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(move || {
        let bearer_auth = HttpAuthentication::bearer(validator);

        App::new()
            .wrap(mw::Logger::default())
            .wrap(
                actix_session::SessionMiddleware::builder(
                    actix_session::storage::CookieSessionStore::default(),
                    secret_key.clone(),
                )
                .cookie_http_only(true)
                .cookie_same_site(actix_web::cookie::SameSite::None)
                .cookie_secure(false)
                .build(),
            )
            .app_data(web::Data::new(app_state.clone())) // Provide the app state
            .service(
                web::scope("/tracker/app")
                    .configure(routes::config_navigation),
            )
            .service(
                fs::Files::new("/tracker/s", "static")
                    .index_file("index.html"), // Serve static files from /static
            )
            .service(
                web::scope("/tracker/api")
                    .wrap(bearer_auth)
                    .configure(routes::config_api),
            )
            // Create Route to redirect /tracker to /tracker/app/login
            .service(
                web::scope("/tracker")
                    .route("", web::get().to(|| async {
                        HttpResponse::Found()
                            .append_header(("LOCATION", "/tracker/app/login"))
                            .finish()
                    })),
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}