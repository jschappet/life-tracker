pub mod task;
pub mod tags;
pub mod daily_todos;
pub mod project;
pub mod user;
pub mod goals;
pub mod reward;


pub use task::{create_task, get_tasks, get_task,
     update_task, delete_task, get_tasks_by_user, search_tasks_by_title,
     update_task_without_title};

pub use user::{create_user, get_users, get_user, update_user, 
    delete_user, get_user_by_username, authenticate_user};

pub use reward::{create_reward, get_rewards, get_reward, update_reward, delete_reward};

pub use daily_todos::{create_daily_todo, get_daily_todos_by_user, get_daily_todos, update_daily_todo, delete_daily_todo};

pub use goals::{create_goal, get_goal_by_id, get_goals_by_user, update_goal, delete_goal};
pub use tags::{create_tag, delete_tag, get_tag, get_tags};

pub use project::{
    create_project,  
    update_project, 
    delete_project, 
    create_project_goal, 
    get_project_goals, 
    delete_project_goal,
    get_projects
};

