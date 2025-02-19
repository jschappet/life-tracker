use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::models::Reward;
use crate::schema::rewards::dsl::*;
use chrono::{NaiveDateTime, Utc};

pub fn create_reward(
    conn: &mut SqliteConnection,
    user_id_val: i32,
    description_val: &str,
    points_val: Option<i32>
) -> QueryResult<Reward> {
    let now: NaiveDateTime = Utc::now().naive_utc();
    diesel::insert_into(rewards)
        .values((
            user_id.eq(user_id_val),
            description.eq(description_val),
            points.eq(points_val),
            created_at.eq(now),
        ))
        .execute(conn)?;
    
    rewards.order(id.desc()).first::<Reward>(conn)
}

pub fn get_rewards(conn: &mut SqliteConnection) -> QueryResult<Vec<Reward>> {
    rewards.load::<Reward>(conn)
}

pub fn get_reward(conn: &mut SqliteConnection, reward_id_val: i32) -> QueryResult<Reward> {
    rewards.find(reward_id_val).first::<Reward>(conn)
}

pub fn update_reward(
    conn: &mut SqliteConnection,
    reward_id_val: i32,
    new_description: &str,
    new_points: Option<i32>
) -> QueryResult<Reward> {
    diesel::update(rewards.find(reward_id_val))
        .set((description.eq(new_description), points.eq(new_points)))
        .execute(conn)?;
    
    rewards.find(reward_id_val).first::<Reward>(conn)
}

pub fn delete_reward(conn: &mut SqliteConnection, reward_id_val: i32) -> QueryResult<usize> {
    diesel::delete(rewards.find(reward_id_val)).execute(conn)
}
