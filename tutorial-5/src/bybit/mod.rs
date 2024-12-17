mod bybit;

use crate::key_value_store;
use crate::key_value_store::KeyValueStore;
use crate::key_value_store::KeyValueStorePostgres;
use crate::{
    caching::{self, CacheKey},
    db,
};
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use tracing::{debug, info};

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

fn calc_h24_change(current_price: &EthPrice, price_h24_ago: &EthPrice) -> f64 {
    (current_price.usd - price_h24_ago.usd) / price_h24_ago.usd
}

async fn update_eth_price_with_most_recent(
    client: &reqwest::Client,
    db_pool: &PgPool,
    key_value_store: &impl KeyValueStore,
    last_price: &mut EthPrice,
) -> Result<()> {
    let most_recent_price = bybit::get_eth_price(client).await?;

    if last_price == &most_recent_price {
        debug!(
            price = last_price.usd,
            minute = last_price.timestamp.to_string(),
            "most recent eth price is equal to last stored price, skipping"
        );
    } else {
        if last_price.timestamp == most_recent_price.timestamp {
            debug!(
                minute = last_price.timestamp.to_string(),
                last_price = last_price.usd,
                most_recent_price = most_recent_price.usd,
                "found more recent price for existing minute",
            );
        } else {
            debug!(
                timestamp = most_recent_price.timestamp.to_string(),
                price = most_recent_price.usd,
                "new most recent price",
            )
        }
    }

    *last_price = most_recent_price;

    let eth_price_stats = EthPriceStats {
        timestamp: last_price.timestamp,
        usd: last_price.usd,
        h24_change: calc_h24_change(last_price, &last_price),
    };

    // update latest eth price status to the correspoinding pg-db table: key_value_store
    // which key is string, value is json(converted via serde_json::to_value)
    key_value_store
        .set_value(
            CacheKey::EthPrice.to_db_key(),
            &serde_json::to_value(&eth_price_stats).unwrap(),
        )
        .await;

    // use pg-inner function to broadcast update status as aduit info
    caching::publish_cache_update(db_pool, &CacheKey::EthPrice).await;
    Ok(())
}
