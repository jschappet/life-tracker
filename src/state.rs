use crate::db::DbPool;

// state.rs
#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub omdb_api_key: String,
}
