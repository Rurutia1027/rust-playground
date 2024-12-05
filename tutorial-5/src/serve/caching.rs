use axum::{http::HeaderValue, response::IntoResponse, Extension, Json};
use chrono::Duration;
use enum_iterator::all;
use futures::{Stream, TryStreamExt};
use hyper::HeaderMap;
use lazy_static::lazy_static;
use reqwest::{header, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{postgres::PgNotification, PgPool};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tokio::task::JoinHandle;
use tracing::{debug, info, trace, warn};

use crate::{
    caching::{self, CacheKey, ParseCacheKeyError},
    env::ENV_CONFIG,
    key_value_store::{self, KeyValueStore, KeyValueStorePostgres},
};

use super::{State, StateExtension};

#[derive(Debug)]
pub struct Cache(RwLock<HashMap<CacheKey, Value>>);

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache {
    pub fn new() -> Self {
        Self(RwLock::new(HashMap::new()))
    }

    // This function:
    // first, converted all enumeration CacheKey into vector items
    // second, iterate each vector item and query correspoinding de-serialized value from the cache(key_value_store)
    // third, insert the db layer's de-serialized data value into local Memory Space --> RwLock<HashMap<CacheKey, Value>>
    //        which insort, load disk serialized data value, deserialize the data, and insert it into the
    // this is an inner function which do not expose operaiton to outside, but provide function like 'new_with_data' to
    // enable db -> hashmap cache during server setup period
    async fn load_from_db(&self, key_value_store: &impl KeyValueStore) {
        info!("loading cache from DB");

        for key in all::<CacheKey>().collect::<Vec<_>>() {
            // query data from db layer key-value store
            let value =
                caching::get_serialized_caching_value(key_value_store, &key)
                    .await;
            match value {
                Some(value) => {
                    // deserialized db queried value
                    let length = serde_json::to_string(&value)
                        .expect(
                            "expect db cache value to be convertable to string",
                        )
                        .len();
                    debug!(%key, %length, "loaded from DB");

                    // insert de-serialized value to local hash map
                    // self.0 means RwLock<HashMap<...>>
                    // because this caching is shared among multiple thread,
                    // so we use RwLock support multiple read, single write
                    self.0.write().unwrap().insert(key, value);
                }
                None => {
                    warn!(%key, "no value found in DB");
                }
            }
        }
    }

    pub async fn new_with_data(key_value_store: &impl KeyValueStore) -> Self {
        let cache = Self::new();
        cache.load_from_db(key_value_store).await;
        cache
    }
}

lazy_static! {
    static ref SIX_SECONDS: Duration = Duration::seconds(6);
    static ref TWO_MINUTES: Duration = Duration::seconds(120);
}

#[cfg(test)]
mod test {
    use std::{collections::HashSet, iter};

    use header::Keys;
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{caching::CacheKey, db};

    #[tokio::test]
    async fn test_cache_key_iter() {
        let keys = all::<CacheKey>().collect::<Vec<_>>();
        for key in keys {
            println!("key content {:?}", key);
        }
    }

    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
    struct TestCacheValue {
        id: u32,
        key_content: String,
        content: String,
    }

    #[tokio::test]
    async fn test_query_and_save_from_kv_store() {
        let test_db = db::tests::TestDb::new().await;
        let kv_store = KeyValueStorePostgres::new(test_db.pool.clone());
        let key_vec = all::<CacheKey>().collect::<Vec<_>>();
        for key in key_vec {
            let value = mock_cache_key_value(key);

            // here insert key&value to db layer's kv_store with value serialized as json
            kv_store
                .set_serializable_value(&key.to_db_key(), &value)
                .await;

            // here query key&value from db layer's kv_store with value de-serialized to string
            let value: Option<String> =
                kv_store.get_deserializable_value(&key.to_db_key()).await;
            println!(
                "query item-key: {:?}, query item-value: {:?}",
                key.to_db_key(),
                Json(&value)
            );
        }
    }

    // in this test, we first create kv_store and insert correspoinding key&value pairs
    // to key_value_store space, and then invoke the outsider's new_with_data this funciton
    // to load all CacheKey's serialized & deserialized value from the db disk layer and store them to the memory layer
    #[tokio::test]
    async fn test_load_new_data() {
        let test_db = db::tests::TestDb::new().await;
        let kv_store = KeyValueStorePostgres::new(test_db.pool.clone());
        let keys = all::<CacheKey>().collect::<Vec<_>>();
        for key in keys {
            let value = mock_cache_key_value(key);
            kv_store
                .set_serializable_value(&key.to_db_key(), &value)
                .await;
        }

        // in previous operation, we already insert data to db layer
        // here we try to invoke load data operation which load all data records from db to local cache
        let cache = Cache::new_with_data(&kv_store).await;
        let key_set: HashSet<CacheKey> = all::<CacheKey>().collect();
        let local_cache_map = cache.0.read().unwrap();
        for (k, v) in local_cache_map.iter() {
            assert!(key_set.contains(k));
        }
    }

    fn mock_cache_key_value(key: CacheKey) -> TestCacheValue {
        let key_str = &key.to_db_key();
        TestCacheValue {
            id: 1,
            key_content: key_str.to_string(),
            content: format!("content field content {}", key_str.to_string()),
        }
    }
}
