
use diesel::{r2d2::{ConnectionManager, Pool}, SqliteConnection};

//pub type DbPool = Pool<Sqlite>;
pub type DbPool = Pool<ConnectionManager<SqliteConnection>> ;