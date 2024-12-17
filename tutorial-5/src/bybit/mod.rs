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
use serde::Deserialize;
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use tokio::time::sleep;
use tracing::{debug, info};

#[derive(Debug, FromRow)]
pub struct EthPriceTimestamp {
    timestamp: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, PartialEq)]
pub struct EthPrice {
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

// `record_eth_price` serves as the entry point for fetching, comparing, and storing price and timestamp values
// from the Bybit API to the local cache and database every 5 seconds.
//
// This synchronization task is a long-running process that starts after initializaiton.
// There is no need to worry about database table creation or migration issues,
// as this task is executed only after the project's main entry process (`server::serve()`),
// where all database initialization and migraiton operations are completed.
// Subsequent sub-tasks, such as this one, are triggered in sequenc after main process is setup completely.
pub async fn record_eth_price() -> Result<()> {
    info!("recording eth prices from bybit to local db table");
    let client = reqwest::Client::new();
    // name the db pool with record-eth-price which executes db connections only for record eth price
    let db_pool = db::get_db_pool("record-eth-price", 3).await;
    let key_value_store = KeyValueStorePostgres::new(db_pool.clone());
    // todo()! let eth_price_store

    // this last_price should be fetch from the eth_price_store which is the cache only for storing eth-price
    let mut last_price = &mut EthPrice {
        timestamp: Utc::now(),
        usd: 0.0,
    };

    // this is a long-running process for fetch & record eth price from bybit to local db

    loop {
        update_eth_price_with_most_recent(
            &client,
            &db_pool,
            &key_value_store,
            last_price,
        )
        .await?;
        sleep(std::time::Duration::from_secs(10)).await;
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use self::EthPrice;
    use super::*;
    use crate::db::tests::TestDb;
    use anyhow::Context;
    use chrono::SubsecRound;
    use test_context::test_context;

    #[test]
    fn test_hello_world() {
        assert!(true);
    }

    // run this locally, we need to export our db url to environment variables via executing command on terminal
    // export DATABASE_URL=postgresql://admin:admin@localhost:5432/defaultdb
    // but in CI environment, we do not need to care about this, because DB_URL already configured inside ci.yml
    // in tutorial-5's ci/cd job
    #[ignore = "failing in CI, caused by bybit API request limit"]
    #[test_context(TestDb)]
    #[tokio::test]
    async fn update_eth_price_with_most_recent_test(test_db: &TestDb) {
        assert!(true);
        let client = reqwest::Client::new();
        let key_value_store = KeyValueStorePostgres::new(test_db.pool.clone());
        let test_price = EthPrice {
            // mock timestamp fetched from the eth price db table old value
            timestamp: Utc::now().trunc_subsecs(0) - Duration::minutes(10),
            usd: 0.0,
        };

        let mut last_price = test_price.clone();

        // here we invoke the update_eth_price_with_most_recent
        // what we expected is the latest value will be fetched, filtered , converted and insert to the kv store table
        // we have not implement the table operations especially for eth price table, so only kv store table will be updated
        update_eth_price_with_most_recent(
            &client,
            &test_db.pool,
            &key_value_store,
            &mut last_price,
        )
        .await
        .unwrap();

        let value = caching::get_serialized_caching_value(
            &key_value_store,
            &CacheKey::EthPrice,
        )
        .await
        .context("cannot find value via cache key")
        .unwrap();

        // println!("value content: {:?}", value);
        // convert the Value into the string
        let db_saved_eth_price: EthPrice =
            serde_json::from_value::<EthPrice>(value)
                .context("failed to convert value from value")
                .unwrap_or(EthPrice {
                    timestamp: Utc::now(),
                    usd: 0.0,
                });

        assert_ne!(
            EthPrice {
                timestamp: Utc::now(),
                usd: 0.0,
            },
            db_saved_eth_price
        );

        println!("db_saved_eth_price item {:?}", db_saved_eth_price);
    }
}
