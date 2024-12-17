mod bybit;

use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow)]
struct EthPriceTimestamp {
    timestamp: DateTime<Utc>,
}

#[derive(Clone, Debug, FromRow, PartialEq)]
struct EthPrice {
    pub timestamp: DateTime<Utc>,
    #[sqlx(rename = "ethusd")]
    pub usd: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EthPriceStats {
    timestamp: DateTime<Utc>,
    usd: f64,
    h24_change: f64,
}
