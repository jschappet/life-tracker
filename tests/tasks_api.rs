#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::prelude::*;
    use life_tracker::state::AppState;
    use life_tracker::models::{NewTask, Task};
    use life_tracker::routes::tasks_api;
    use chrono::NaiveDate;

    fn get_app_state() -> AppState {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = Pool::builder().build(manager).expect("Failed to create pool");
        
        let mut conn = pool.get().expect("Failed to get DB connection");

        diesel::sql_query("PRAGMA foreign_keys = ON;").execute(&mut conn).unwrap();
        diesel::sql_query("DROP TABLE IF EXISTS tasks;").execute(&mut conn).unwrap();
        diesel::sql_query("
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL DEFAULT 0,
    project_id INTEGER,
    title TEXT NOT NULL,
    description TEXT,
    due_date DATE,
    status TEXT CHECK(status IN ('pending', 'in_progress', 'completed')) DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
        ").execute(&mut conn).unwrap();

        AppState {
            db_pool: pool,
            omdb_api_key: "foo".to_string(),
        }
    }

    #[actix_rt::test]
    async fn test_create_task_api() {
        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(tasks_api::config)
        ).await;

        let task_json = r#"{
            "title": "New Task",
            "description": "API test task",
            "due_date": "2025-02-06"
        }"#;

        let req = test::TestRequest::post()
            .uri("/api/tasks")
            .append_header(("Content-Type", "application/json"))
            .set_payload(task_json)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn test_get_tasks_api() {
        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(tasks_api::config)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/tasks")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
