#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_web::{test, web, App, middleware as mw};
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::prelude::*;
    use handlebars::{DirectorySourceOptions, Handlebars};
    use life_tracker::state::AppState;
  //  use life_tracker::models::{User, NewUser};
    use life_tracker::routes::users_api;
   // use chrono::NaiveDateTime;

    fn get_app_state<'hb>() -> AppState<'hb> {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = Pool::builder().build(manager).expect("Failed to create pool");
        
        let mut conn = pool.get().expect("Failed to get DB connection");

        diesel::sql_query("PRAGMA foreign_keys = ON;").execute(&mut conn).unwrap();
        diesel::sql_query("DROP TABLE IF EXISTS users;").execute(&mut conn).unwrap();
        diesel::sql_query(r#"
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT  NOT NULL,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP  NOT NULL 
);
        "#).execute(&mut conn).unwrap();

        diesel::sql_query(r#"
INSERT INTO users (id, username, email, password_hash) 
VALUES (0, "Not Assigned", "nobody@nowhere.com", "");
        "#).execute(&mut conn).unwrap();

        let dso = DirectorySourceOptions::default();

        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory("./templates/", dso)
            .expect("Failed to register templates directory");
    
        let handlebars = Arc::new(handlebars);

        AppState {
            db_pool: pool,
            omdb_api_key: "foo".to_string(),
            hb: handlebars,
        }
    }

    #[actix_rt::test]
    async fn test_create_user_api() {
        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .wrap(mw::Logger::default())
                .app_data(web::Data::new(app_state))
                .service(
                    web::scope("/tracker/api")
                    .configure(users_api::config)
                )
        ).await;

        let user_json = r#"{
            "username": "username",
            "email": "email@nobody.com",
            "password_hash": "asdfaf"
        }"#;

        let req = test::TestRequest::post()
            .uri("/tracker/api/users")
            .append_header(("Content-Type", "application/json"))
            .set_payload(user_json)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn test_get_users_api() {
        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .wrap(mw::Logger::default())
                .app_data(web::Data::new(app_state))
                .service(
                    web::scope("/tracker/api")
                    .configure(users_api::config)
                )
        ).await;

        let req = test::TestRequest::get()
            .uri("/tracker/api/users")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
