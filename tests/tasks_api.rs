#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_web::{test, web, App, middleware as mw};
    //use actix_web::{web, App, Error, HttpServer, HttpResponse, middleware as mw};

    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::prelude::*;
    use handlebars::{DirectorySourceOptions, Handlebars};
    use life_tracker::state::AppState;
   // use life_tracker::models::{NewTask, Task};
    use life_tracker::routes::tasks_api;
   // use chrono::NaiveDate;

    fn get_app_state<'hb>() -> AppState<'hb> {
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
            status TEXT CHECK(
                    status IN ('pending', 'suspended','in_progress', 'completed')
                ) 
                DEFAULT 'pending',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
            start_time TIMESTAMP,
            end_time TIMESTAMP
            );
        
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
            gpt_api_key: "demo".to_string(),
        }
    }

    #[actix_rt::test]
    async fn test_create_task_api() {
        let _ = env_logger::builder().is_test(true).try_init(); // Add this line

        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .wrap(mw::Logger::default())
                .app_data(web::Data::new(app_state))
                .service(
                    web::scope("/tracker/api")
                    .configure(tasks_api::config)
                )
        ).await;

        let task_json = r#"{
            "title": "New Task",
            "description": "API test task",
            "due_date": "2025-02-06",
            "user_id": 0
        }"#;

        let req = test::TestRequest::post()
            .uri("/tracker/api/tasks")
            .append_header(("Content-Type", "application/json"))
            .set_payload(task_json)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn test_get_tasks_api() {
        let _ = env_logger::builder().is_test(true).try_init(); // Add this line

        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .wrap(mw::Logger::default())
                .app_data(web::Data::new(app_state))
                .service(
                    web::scope("/tracker/api")
                    .configure(tasks_api::config)
                )
            ).await;

        let req = test::TestRequest::get()
            .uri("/tracker/api/tasks")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        log::error!("Headers: {:?}  ", resp.headers().clone());
        assert_eq!(resp.status(), 200);
    }
}
