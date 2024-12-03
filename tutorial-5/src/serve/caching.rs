use chrono::Duration;
use enum_iterator::all;
use futures::{Stream, TryStreamExt};
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

use crate::caching::{self};

// #[derive(Debug)]
// pub struct Cache(RwLock<HashMap<CacheKey, Value>>);

// impl Default for Cache {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl Cache {
//     pub fn new() -> Self {
//         Self(RwLock::new(HashMap::new()))
//     }
// }
