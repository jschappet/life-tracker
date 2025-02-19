#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::prelude::*;
    use life_tracker::state::AppState;
    use life_tracker::models::{User, NewUser};
    use life_tracker::routes::users_api;
    use chrono::NaiveDateTime;

    fn get_app_state() -> AppState {
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

        AppState {
            db_pool: pool,
            omdb_api_key: "foo".to_string(),
        }
    }

    #[actix_rt::test]
    async fn test_create_user_api() {
        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(users_api::config)
        ).await;

        let user_json = r#"{
            "username": "username",
            "email": "email@nobody.com",
            "password_hash": "asdfaf"
        }"#;

        let req = test::TestRequest::post()
            .uri("/api/users")
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
                .app_data(web::Data::new(app_state))
                .configure(users_api::config)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/users")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
