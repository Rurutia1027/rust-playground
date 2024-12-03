use anyhow::Result;
use enum_iterator::Sequence;
use futures::executor;
use serde::Serialize;
use serde_json::Value;
use sqlx::{PgExecutor, PgPool};
use std::{fmt::Display, str::FromStr};
use thiserror::Error;
use tracing::debug;

use crate::key_value_store::{self, KeyValueStore};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Sequence)]
pub enum CacheKey {
    AverageEthPrice,
    EthPrice,
    BaseFeeOverTime,
    BaseFeePerGasBarrier,
    BaseFeePerGasStats,
}

impl CacheKey {
    pub fn to_db_key(self) -> &'static str {
        use CacheKey::*;
        match self {
            AverageEthPrice => "average-eth-price",
            BaseFeeOverTime => "base-fee-over-time",
            BaseFeePerGasBarrier => "current-base-fee",
            BaseFeePerGasStats => "base-fee-per-gas-stats",
            EthPrice => "eth-price",
        }
    }
}

impl Display for CacheKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_db_key())
    }
}

#[derive(Debug, Error)]
pub enum ParseCacheKeyError {
    #[error("failed to parse cache key {0}")]
    UnknownCacheKey(String),
}

impl FromStr for CacheKey {
    type Err = ParseCacheKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "average-eth-price" => Ok(Self::AverageEthPrice),
            "base-fee-over-time" => Ok(Self::BaseFeeOverTime),
            "current-base-fee" => Ok(Self::BaseFeePerGasBarrier),
            "base-fee-per-gas-stats" => Ok(Self::BaseFeePerGasStats),
            "eth-price" => Ok(Self::EthPrice),
            _ => Err(ParseCacheKeyError::UnknownCacheKey(
                "Receive Unknow Key".to_string(),
            )),
        }
    }
}

pub async fn publish_cache_update<'a>(
    executor: impl PgExecutor<'a>,
    key: &CacheKey,
) {
    debug!(?key, "publishing cache update");

    sqlx::query!(
        "
        SELECT pg_notify('cache-update', $1)
        ",
        key.to_db_key(),
    )
    .execute(executor)
    .await
    .unwrap();
}

pub async fn get_serialized_caching_value(
    key_value_store: &impl KeyValueStore,
    cache_key: &CacheKey,
) -> Option<Value> {
    key_value_store.get_value(cache_key.to_db_key()).await
}

pub async fn set_value<'a>(
    executor: impl PgExecutor<'_>,
    cache_key: &CacheKey,
    value: impl Serialize,
) {
    key_value_store::set_value(
        executor,
        cache_key.to_db_key(),
        &serde_json::to_value(value).expect("expect value to be serializable"),
    )
    .await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db, env::ENV_CONFIG, key_value_store::KeyValueStorePostgres};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    struct TestJsonItem {
        name: String,
        age: i32,
    }

    // test publish cache update
    #[tokio::test]
    async fn test_publish_cache_update() {
        // here we create a db listener and let monitor db's cache-updaet operaiton
        let mut listener =
            sqlx::postgres::PgListener::connect(ENV_CONFIG.db_url.as_str())
                .await
                .unwrap();

        // let pg listen to 'cache-update' this channel
        listener.listen("cache-update").await.unwrap();

        // here why we wrap the inner listener#recv()#await with async {}
        // this is because, even though the inner listenr#recv()#await is an async logic
        // but if we don't surround it with the async, our code logic will block here
        // and the coming logic will not be executed,
        // and here we use the async {...} to get a future handler
        // and let the code contine execute, in the end we use the async {} received 'future handler' to await to retrieve the final result
        let notification_future = async { listener.recv().await };
        let mut connection = db::tests::get_test_db_connection().await;
        publish_cache_update(&mut connection, &CacheKey::EthPrice).await;

        let notification = notification_future.await.unwrap();

        assert_eq!(notification.payload(), CacheKey::EthPrice.to_db_key())
    }
}
