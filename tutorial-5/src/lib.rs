pub mod caching;
pub mod db;
pub mod env;
pub mod health;
pub mod key_value_store;
pub mod mockservices;
pub mod serve;

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}
