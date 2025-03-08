// @generated automatically by Diesel CLI.

diesel::table! {
    daily_todos (id) {
        id -> Integer,
        user_id -> Integer,
        task_id -> Integer,
        date -> Date,
        completed -> Nullable<Bool>,
    }
}

diesel::table! {
    goals (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        due_date -> Nullable<Date>,
        status -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    project_goals (project_id, goal_id) {
        project_id -> Integer,
        goal_id -> Integer,
    }
}

diesel::table! {
    projects (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    rewards (id) {
        id -> Integer,
        user_id -> Integer,
        description -> Text,
        points -> Nullable<Integer>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Text,
        icon_url -> Nullable<Text>,
    }
}

diesel::table! {
    task_tags (task_id, tag_id) {
        task_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    tasks (id) {
        id -> Integer,
        user_id -> Integer,
        project_id -> Nullable<Integer>,
        title -> Text,
        description -> Nullable<Text>,
        due_date -> Nullable<Date>,
        status -> Nullable<Text>,
        created_at -> Timestamp,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(daily_todos -> tasks (task_id));
diesel::joinable!(daily_todos -> users (user_id));
diesel::joinable!(goals -> users (user_id));
diesel::joinable!(project_goals -> goals (goal_id));
diesel::joinable!(project_goals -> projects (project_id));
diesel::joinable!(projects -> users (user_id));
diesel::joinable!(rewards -> users (user_id));
diesel::joinable!(task_tags -> tags (tag_id));
diesel::joinable!(task_tags -> tasks (task_id));
diesel::joinable!(tasks -> projects (project_id));
diesel::joinable!(tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    daily_todos,
    goals,
    project_goals,
    projects,
    rewards,
    tags,
    task_tags,
    tasks,
    users,
);
