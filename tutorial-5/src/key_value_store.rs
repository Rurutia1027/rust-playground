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

#[derive(Debug)]
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
    use super::*;
    use crate::db;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use sqlx::Connection;

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    struct TestJsonItem {
        id: String,
        key: String,
        value: u64,
    }

    #[tokio::test]
    async fn test_hello() {
        println!("Test Info");
    }

    #[tokio::test]
    async fn get_set_value_test() {
        let mut connection = db::tests::get_test_db_connection().await;

        let mut transaction = connection.begin().await.unwrap();

        let test_json_value = TestJsonItem {
            id: "test_id_str".to_string(),
            key: "key_str".to_string(),
            value: 2345,
        };

        set_value(
            &mut *transaction,
            "test-key",
            &serde_json::to_value(&test_json_value).unwrap(),
        )
        .await;

        let value = get_value(&mut *transaction, "test-key").await.unwrap();
        let query_test_json_item =
            serde_json::from_value::<TestJsonItem>(value).unwrap();

        assert_eq!(query_test_json_item, test_json_value);
        println!(
            "value: {:?}, previous value: {:?}",
            query_test_json_item, test_json_value
        );
    }

    #[tokio::test]
    async fn get_null_value_test() {
        let mut connection = db::tests::get_test_db_connection().await;
        let mut transaction = connection.begin().await.unwrap();

        set_value(
            &mut *transaction,
            "test-key-3",
            &serde_json::to_value(json!(None::<String>)).unwrap(),
        )
        .await;

        let value = get_value(&mut *transaction, "test-key-3").await.unwrap();
        let test_json_query_value =
            serde_json::from_value::<Option<String>>(value).unwrap();

        assert_eq!(None, test_json_query_value);

        println!("query value {:?}", test_json_query_value);
    }

    #[tokio::test]
    async fn test_query_nonexist_value() {
        let mut test_db = db::tests::TestDb::new().await;
        let db_name = &test_db.name;
        println!(
            "#test_query_nonexist_value we got db name {} and db info {:?}",
            db_name, test_db
        );
        let mut store = KeyValueStorePostgres::new(test_db.pool.clone());

        // since we use create function to create the db connection instance,
        // we need to create the init table `key_value_store` in the db

        sqlx::query!(
            "
            CREATE TABLE IF NOT EXISTS key_value_store(
                key VARCHAR(256) PRIMARY KEY,
                value TEXT
            ); 
            "
        )
        .execute(&test_db.pool)
        .await
        .unwrap();

        println!("#test_query_nonexist_value store info {:?}", store);

        let key = "nonexisting_key";
        let query_value: Option<String> =
            store.get_deserializable_value(key).await;
        println!("#test_query_nonexist_value queried value {:?}", query_value);
        assert_eq!(query_value, None);
    }
}
