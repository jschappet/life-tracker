#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_web::{test, web, App};
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::prelude::*;
    use handlebars::{DirectorySourceOptions, Handlebars};
    use life_tracker::state::AppState;
  //  use life_tracker::models::{NewDailyTodo, DailyTodo};
    use life_tracker::routes::daily_todo_api;
   // use chrono::NaiveDate;

    fn get_app_state<'hb>() -> AppState<'hb> {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = Pool::builder().build(manager).expect("Failed to create pool");
        
        let mut conn = pool.get().expect("Failed to get DB connection");

        diesel::sql_query("PRAGMA foreign_keys = ON;").execute(&mut conn).unwrap();
        diesel::sql_query("DROP TABLE IF EXISTS daily_todos;").execute(&mut conn).unwrap();
        diesel::sql_query("
CREATE TABLE daily_todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT  NOT NULL,
    user_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    date DATE NOT NULL DEFAULT (DATE('now')),
    completed BOOLEAN DEFAULT FALSE 
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
            //gpt_api_key: "demo".to_string(),
        }
    }

    #[actix_rt::test]
    async fn test_create_daily_todo_api() {
        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .service(
                    web::scope("/tracker/api")
                    .configure(daily_todo_api::config)
                )
        ).await;

        let daily_todo_json = r#"{
            "user_id": 0,
            "task_id": 1,
            "date": "2025-01-01",
            "completed": false
        }"#;

        let req = test::TestRequest::post()
            .uri("/tracker/api/daily_todo")
            .append_header(("Content-Type", "application/json"))
            .set_payload(daily_todo_json)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn test_get_daily_todos_api() {
        let app_state = get_app_state();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .service(
                    web::scope("/tracker/api")
                    .configure(daily_todo_api::config)
                )
        ).await;

        let req = test::TestRequest::get()
            .uri("/tracker/api/daily_todo/0")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
