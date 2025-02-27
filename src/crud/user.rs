use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::models::User;
use crate::schema::users::dsl::*;
use chrono::{NaiveDateTime, Utc};
use bcrypt::{hash, verify, DEFAULT_COST};

// Use bcrypt for password hashing
pub fn create_user(
    conn: &mut SqliteConnection,
    new_username: &str,
    new_email: Option<&str>,
    new_password: &str
) -> QueryResult<User> {
    let now: NaiveDateTime = Utc::now().naive_utc();
    let hashed_password = hash(new_password, DEFAULT_COST).expect("Failed to hash password");
    diesel::insert_into(users)
        .values((
            username.eq(new_username),
            email.eq(new_email.as_deref().unwrap_or("")),
            password_hash.eq(hashed_password),
            created_at.eq(now),
        ))
        .execute(conn)?;
    
    users.order(id.desc()).first::<User>(conn)
}

// Use bcrypt for password hashing
pub fn authenticate_user(
    conn: &mut SqliteConnection,
    usrname: String,
    password: String
) -> QueryResult<User> {
    let user = users.filter(username.eq(usrname)).first::<User>(conn)?;
    if verify(password, &user.password_hash).expect("Failed to verify password") {
        Ok(user)
    } else {
        Err(diesel::result::Error::NotFound)
    }
}

pub fn get_users(conn: &mut SqliteConnection) -> QueryResult<Vec<User>> {
    users.load::<User>(conn)
}

pub fn get_user(conn: &mut SqliteConnection, user_id_val: i32) -> QueryResult<User> {
    users.find(user_id_val).first::<User>(conn)
}

pub fn get_user_by_username(conn: &mut SqliteConnection, usr_name: String) -> QueryResult<User> {
    users.filter(username.eq(usr_name)).first::<User>(conn)
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
