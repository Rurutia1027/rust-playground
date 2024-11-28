use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgExecutor, PgPool};
use tracing::trace;

pub async fn get_value(
    executor: impl PgExecutor<'_>,
    key: &str,
) -> Option<Value> {
    trace!(key = key, "getting key value pair");

    sqlx::query!(
        "
        SELECT value FROM key_value_store 
        WHERE key = $1
        ",
        key,
    )
    .fetch_optional(executor)
    .await
    .unwrap()
    .and_then(|row| row.value)
}

pub async fn set_value(
    executor: impl PgExecutor<'_>,
    key: &str,
    value: &Value,
) {
    trace!("storing key {}", &key);

    sqlx::query!(
        "
        INSERT INTO key_value_store (key, value) VALUES ($1, $2)
        ON CONFLICT (key) DO UPDATE SET 
            value = excluded.value
    ",
        key,
        value
    )
    .execute(executor)
    .await
    .unwrap();
}

#[async_trait]
pub trait KeyValueStore {
    async fn get_value(&self, key: &str) -> Option<Value>;
    async fn set_value(&self, key: &str, value: &Value);
}

pub struct KeyValueStorePostgres {
    db_pool: PgPool,
}

impl KeyValueStorePostgres {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn get_deserializable_value<D: for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Option<D> {
        get_value(&self.db_pool, key)
            .await
            .and_then(|value| serde_json::from_value(value).ok())
    }

    pub async fn set_serializable_value<S: Serialize>(
        &self,
        key: &str,
        value: S,
    ) {
        set_value(
            &self.db_pool,
            key,
            &serde_json::to_value(value)
                .expect("expect value to be serializable"),
        )
        .await
    }
}

#[async_trait]
impl KeyValueStore for KeyValueStorePostgres {
    async fn get_value(&self, key: &str) -> Option<Value> {
        get_value(&self.db_pool, key).await
    }

    async fn set_value(&self, key: &str, value: &Value) {
        set_value(&self.db_pool, key, value).await
    }
}

#[cfg(test)]
mod tests {
    use crate::db;
    use crate::init_tracing;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use sqlx::Connection;
    use tracing::{info, trace};

    #[tokio::test]
    async fn test_hello() {
        init_tracing();
        tracing::info!("Test Info");
    }
}
