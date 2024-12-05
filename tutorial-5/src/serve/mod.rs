use axum::Extension;
use health::ServeHealth;
use sqlx::PgPool;
use std::sync::Arc;

pub mod caching;
pub mod health;
pub mod server;

use caching::Cache;
pub struct State {
    pub cache: Cache,
    pub db_pool: PgPool,
    pub health: ServeHealth,
}

pub type StateExtension = Extension<Arc<State>>;
