//mod routes;
mod middleware;

mod claims;

use actix_web::{web, App, HttpServer, HttpResponse, middleware as mw};

use actix_web_httpauth::middleware::HttpAuthentication;
use life_tracker::auth::validator;
use life_tracker::{routes, settings};
use life_tracker::state::AppState;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use crate::settings::Settings;
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



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new()
        .expect("Config failed to load");
    
    // Enable logging system-wide
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    log::debug!("Logging initialized.");

    dotenvy::dotenv().ok();
    log::debug!("Environment variables loaded.");

    let database_url = settings.database.url.clone();
    log::debug!("Database URL: {}", database_url);

    //let api_key: String = settings.api_key.clone();

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
        .register_templates_directory(settings.templates.clone(), dso)
        .expect("Failed to register templates directory");

    log::debug!("Hb: {}", handlebars.get_templates().keys().count());

    // Log all registered templates
    // for template_name in handlebars.get_templates().keys() {
    //     log::debug!("Registered template: {}", template_name);
    // }
    let handlebars = Arc::new(handlebars);

    let app_state = AppState {
        db_pool: pool.clone(),
        omdb_api_key: "foo".to_string(),
        hb: handlebars.clone(),
        settings: settings.clone(),
    };

    log::info!("App state initialized.");
    let secret_key = actix_web::cookie::Key::from(settings.web_config.cookie_key.as_bytes());
    let web_context = settings.web_config.context_path.clone(); 
    
    let settings_clone = settings.clone();

    HttpServer::new(move || {
        let bearer_auth = HttpAuthentication::bearer(validator);
        let settings = settings_clone.clone();
        
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
                web::scope(format!("{}/app", web_context).as_str())
                    .configure(routes::config_navigation),
            )
            .service(
                fs::Files::new(format!("{}/s", web_context).as_str(), "static")
                    .index_file("index.html"), // Serve static files from /static
            )
            .service(
                web::scope(format!("{}/api", web_context).as_str())
                    .wrap(bearer_auth)
                    .configure(routes::config_api),
            )
            // Create Route to redirect /tracker to /tracker/app/login
            .service(
                web::scope(&web_context)
                    .route("", web::get().to(move || {
                        let login_url = settings.web_config.login_url.clone();
                        async move {
                            HttpResponse::Found()
                                .append_header(("LOCATION", login_url))
                                .finish()
                        }
                    })),
            )
    })
    .bind(settings.get_bind())?
    .run()
    .await
}