pub mod bybit;
pub mod caching;
pub mod db;
pub mod env;
pub mod eth_prices;
pub mod execution_chain;
pub mod health;
pub mod key_value_store;
pub mod mockservices;
pub mod serve;
pub mod units;

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}
