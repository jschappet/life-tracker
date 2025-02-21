//mod routes;
mod middleware;

use actix_web::{web, App, HttpServer, middleware as mw};
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

    let dso = DirectorySourceOptions::default();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory("./templates/", dso)
        .expect("Failed to register templates directory");

        log::debug!("Hb: {}",handlebars.get_templates().keys().count());
            
    // Log all registered templates
    for template_name in handlebars.get_templates().keys() {
        log::debug!("Registered template: {}", template_name);
    }
    let handlebars  = Arc::new(handlebars);

    let app_state = AppState {
        db_pool: pool.clone(),
        omdb_api_key: "foo".to_string(),
        hb: handlebars.clone(),
    };

    log::info!("App state initialized.");

   
    HttpServer::new(move || {
        App::new()
            .wrap(mw::Logger::default()) 
            .wrap(middleware::log_routes::LogRoutes) // Add the custom middleware
            .app_data(web::Data::new(app_state.clone())) // Provide the app state
            .service(fs::Files::new("/s", "static")
                .index_file("index.html")) // Serve static files from /static
            .configure(routes::config) // Use the combined config in routes/mod.rs
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}