#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::prelude::*;
    use life_tracker::state::AppState;
    use life_tracker::models::{NewReward, Reward};
    use life_tracker::routes::rewards_api;
    use chrono::NaiveDateTime;

    fn get_app_state() -> AppState {
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

        AppState {
            db_pool: pool,
            omdb_api_key: "foo".to_string(),
        }
    }

    #[actix_rt::test]
    async fn test_create_reward_api() {
        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(rewards_api::config)
        ).await;

        let reward_json = r#"{
            "user_id": 1,
            "description": "Test reward",
            "points": 50
        }"#;

        let req = test::TestRequest::post()
            .uri("/api/rewards")
            .append_header(("Content-Type", "application/json"))
            .set_payload(reward_json)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn test_get_rewards_api() {
        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(rewards_api::config)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/rewards")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
