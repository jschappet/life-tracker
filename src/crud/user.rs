use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::models::User;
use crate::schema::users::dsl::*;
use chrono::{NaiveDateTime, Utc};


pub fn create_user(
    conn: &mut SqliteConnection,
    new_username: &str,
    new_email: Option<&str>,
    //proj_created_at: NaiveDateTime,
    new_password_hash: &str
) -> QueryResult<User> {
    let now: NaiveDateTime = Utc::now().naive_utc();
    diesel::insert_into(users)
        .values((
            username.eq(new_username),
            email.eq(new_email.as_deref().unwrap_or("")),
            password_hash.eq(new_password_hash),
            created_at.eq(now),
        ))
        .execute(conn)?;
    
    users.order(id.desc()).first::<User>(conn)
}

pub fn get_users(conn: &mut SqliteConnection) -> QueryResult<Vec<User>> {
    users.load::<User>(conn)
}

pub fn get_user(conn: &mut SqliteConnection, user_id_val: i32) -> QueryResult<User> {
    users.find(user_id_val).first::<User>(conn)
}

pub fn update_user(
    conn: &mut SqliteConnection,
    user_id_val: i32,
    new_password_hash: &str,
    new_email: Option<&str>
) -> QueryResult<User> {
    diesel::update(users.find(user_id_val))
        .set((password_hash.eq(new_password_hash),
         email.eq(new_email.as_deref().unwrap_or(""))))
        .execute(conn)?;
    
    users.find(user_id_val).first::<User>(conn)
}

pub fn delete_user(conn: &mut SqliteConnection, user_id_val: i32) -> QueryResult<usize> {
    diesel::delete(users.find(user_id_val)).execute(conn)
}
