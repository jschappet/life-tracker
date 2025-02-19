#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::prelude::*;
    use life_tracker::state::AppState;
    use life_tracker::models::{NewProject, Project};
    use life_tracker::routes::projects_api;
    use chrono::NaiveDateTime;

    fn get_app_state() -> AppState {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = Pool::builder().build(manager).expect("Failed to create pool");
        
        let mut conn = pool.get().expect("Failed to get DB connection");

        diesel::sql_query("PRAGMA foreign_keys = ON;").execute(&mut conn).unwrap();
        diesel::sql_query("DROP TABLE IF EXISTS projects;").execute(&mut conn).unwrap();
        diesel::sql_query("
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT  NOT NULL,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
        ").execute(&mut conn).unwrap();

        AppState {
            db_pool: pool,
            omdb_api_key: "foo".to_string(),
        }
    }

    #[actix_rt::test]
    async fn test_create_project_api() {
        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(projects_api::config)
        ).await;

        let project_json = r#"{
            "title": "New Project",
            "description": "API test project",
            "user_id": 0
        }"#;

        let req = test::TestRequest::post()
            .uri("/api/projects")
            .append_header(("Content-Type", "application/json"))
            .set_payload(project_json)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn test_get_projects_api() {
        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .configure(projects_api::config)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/projects")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
