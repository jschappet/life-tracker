#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use actix_web::{test, web, App};
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::prelude::*;
    use handlebars::{DirectorySourceOptions, Handlebars};
    use life_tracker::state::AppState;
    use life_tracker::routes::rewards_api;
    use env_logger; // Add this line

    fn get_app_state<'hb>() -> AppState<'hb> {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = Pool::builder().build(manager).expect("Failed to create pool");
        
        let mut conn = pool.get().expect("Failed to get DB connection");

        diesel::sql_query("PRAGMA foreign_keys = ON;").execute(&mut conn).unwrap();
        diesel::sql_query("DROP TABLE IF EXISTS rewards;").execute(&mut conn).unwrap();
        diesel::sql_query("
CREATE TABLE rewards (
    id INTEGER PRIMARY KEY AUTOINCREMENT  NOT NULL,
    user_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    points INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP  NOT NULL
);
        ").execute(&mut conn).unwrap();

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
    async fn test_create_reward_api() {
        let _ = env_logger::builder().is_test(true).try_init(); // Add this line
        log::error!("DEBUG LOG");
        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .service(
                    web::scope("/tracker/api")
                    .configure(rewards_api::config)
                )

        ).await;

        let reward_json = r#"{
            "user_id": 1,
            "description": "Test reward",
            "points": 50,
            "created_at": "2025-01-15T14:30:00"
        }"#;

        let req = test::TestRequest::post()
            .uri("/tracker/api/rewards")
            .append_header(("Content-Type", "application/json"))
            .set_payload(reward_json)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        log::debug!("{:?}", resp.headers());
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn test_get_rewards_api() {
        let _ = env_logger::builder().is_test(true).try_init(); // Add this line

        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .service(
                    web::scope("/tracker/api")
                    .configure(rewards_api::config)
                )
        ).await;

        let req = test::TestRequest::get()
            .uri("/tracker/api/rewards")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
