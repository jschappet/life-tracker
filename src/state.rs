use crate::{db::DbPool, settings::Settings};
use handlebars::Handlebars;
use std::sync::Arc;

// state.rs
#[derive(Clone)]
pub struct AppState<'hb> {
    pub db_pool: DbPool,
    pub omdb_api_key: String,
    pub hb: Arc<Handlebars<'hb>>, 
    pub settings: Settings,
}
