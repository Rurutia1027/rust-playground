use axum::{http::HeaderValue, response::IntoResponse, Extension, Json};
use chrono::Duration;
use enum_iterator::all;
use futures::{Stream, TryStreamExt};
use hyper::HeaderMap;
use lazy_static::lazy_static;
use reqwest::{header, StatusCode};
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
    key_value_store::{KeyValueStore, KeyValueStorePostgres},
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

    // function create an instance of Cache
    // and initialized with data items loaded from database
    pub async fn new_with_data(key_value_store: &impl KeyValueStore) -> Self {
        let cache = Self::new();
        cache.load_from_db(key_value_store).await;
        cache
    }
}

pub async fn cached_get_with_custom_duration(
    Extension(state): StateExtension,
    analysis_cache_key: &CacheKey,
    max_age: &Duration,
    stale_while_revalidate: &Duration,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    // here we create an instance of http header cache map
    // and set some init config value in it, like cache max time and stale time
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_str(&format!(
            "public, max-age={}, stale-while-revalidate={}",
            max_age.num_seconds(),
            stale_while_revalidate.num_seconds()
        ))
        .unwrap(),
    );

    // here we try to query the value from local memory based hash cache
    // by given cache key
    // if query something then build response body with the previous hash cache, and reply
    // otherwise, reply the response with 'service unavailable'
    match state.cache.0.read().unwrap().get(analysis_cache_key) {
        None => StatusCode::SERVICE_UNAVAILABLE.into_response(),
        Some(cached_value) => {
            (headers, Json(cached_value).into_response()).into_response()
        }
    }
}

pub async fn cached_get(
    state: StateExtension,
    analysis_cache_key: &CacheKey,
) -> impl IntoResponse {
    cached_get_with_custom_duration(
        state,
        analysis_cache_key,
        &SIX_SECONDS,
        &TWO_MINUTES,
    )
    .await
}

async fn process_notifications(
    mut notification_stream: impl Stream<Item = Result<PgNotification, sqlx::Error>>
        + Unpin,
    state: Arc<State>,
    key_value_store: impl KeyValueStore,
) {
    while let Some(notification) = notification_stream.try_next().await.unwrap()
    {
        let payload = notification.payload();
        match payload.parse::<CacheKey>() {
            Err(ParseCacheKeyError::UnknownCacheKey(cache_key)) => {
                trace!(
                    %cache_key,
                    "unsupported cache update, skipping"
                );
            }
            Ok(cache_key) => {
                debug!(%cache_key, "cache update");
                let value = caching::get_serialized_caching_value(
                    &key_value_store,
                    &cache_key,
                )
                .await;

                if let Some(value) = value {
                    state.cache.0.write().unwrap().insert(cache_key, value);
                } else {
                    warn!(%cache_key, 
                    "got a message to update our served cache, but DB had no valid to give");
                }
            }
        }
    }
}

pub async fn update_cache_from_notifications(
    state: Arc<State>,
    db_pool: &PgPool,
) -> JoinHandle<()> {
    let db_url = format!(
        "{}?application_name={}",
        ENV_CONFIG.db_url, "serve-rs-cache-update"
    );

    let mut listener =
        sqlx::postgres::PgListener::connect(&db_url).await.unwrap();
    listener.listen("cache-update").await.unwrap();
    debug!("listening for cache updates");

    let notification_stream = listener.into_stream();
    let key_value_store = KeyValueStorePostgres::new(db_pool.clone());

    tokio::spawn(async move {
        process_notifications(notification_stream, state, key_value_store)
            .await;
    })
}

lazy_static! {
    static ref SIX_SECONDS: Duration = Duration::seconds(6);
    static ref TWO_MINUTES: Duration = Duration::seconds(120);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{caching::CacheKey, db, serve::health::ServeHealth};
    use std::any::Any;
    use std::collections::HashSet;

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

    use chrono::Utc;
    use futures::channel;
    use tokio::sync::mpsc;

    // in this test case, we test the logic of cached_get this funciton which is implemented to retrive data
    // from memory hash cache and wrap return value into http response message
    // first, we initialize the test db , and build key value store based on the test db
    // second, we create instance of local memory based cache and passing previously created kv-store to initialize it
    // third, we inject series of values to the db based key-value store,
    // fourth, we let created memory based cache load all data from db layer to memory layer
    // fifth, we traverse all set of the CacheKey set and passing it to the cached_get function to verify whether the existing value
    // in the memory based cache can be converted into the IntoResponse correctly
    #[tokio::test]
    async fn test_cached_get_logic() {
        let test_db = db::tests::TestDb::new().await;
        let kv_store = KeyValueStorePostgres::new(test_db.pool.clone());
        let keys = all::<CacheKey>().collect::<Vec<_>>();
        for key in keys {
            let value = mock_cache_key_value(key);
            kv_store
                .set_serializable_value(&key.to_db_key(), &value)
                .await;
        }

        let cache = Cache::new_with_data(&kv_store).await;

        // here we create an instance of State this just like the spring context
        // which manages and holds a series of singleton insteances thread-safely
        let state = Arc::new(State {
            cache: cache,
            db_pool: test_db.pool.clone(),
            health: ServeHealth::new(Utc::now()),
        });

        // just kidding ... let me understand state better in Rust
        let spring_context = Extension(state);
        let keys = all::<CacheKey>().collect::<HashSet<_>>();
        for analysis_cache_key in keys {
            let contex = spring_context.clone();
            let ret = cached_get(contex, &analysis_cache_key).await;
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

    // here we create a MockPgNotificaiton which mocks the PgNotificaiton
    struct MockPgNotification {
        channel: String,
        payload: String,
    }

    impl MockPgNotification {
        fn new(channel: &str, payload: &str) -> Self {
            MockPgNotification {
                channel: channel.to_string(),
                payload: payload.to_string(),
            }
        }
    }
}
