pub mod navigation;
pub mod tasks_api;
pub mod daily_todo_api;
pub mod goals_api;
pub mod projects_api;
pub mod rewards_api;
pub mod users_api;

pub mod tags_api;
pub mod gpt_api;

use actix_web::web;

pub fn config_api(cfg: &mut web::ServiceConfig) {
    cfg.configure(tasks_api::config)
        .configure(goals_api::config)
        .configure(projects_api::config)
        .configure(users_api::config)
        .configure(rewards_api::config)
        .configure(tags_api::config)
        .configure(gpt_api::config)

        .configure(daily_todo_api::config);
}

pub fn config_navigation(cfg: &mut web::ServiceConfig) {
    cfg.configure(navigation::config);
}
