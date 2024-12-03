use anyhow::Result;
use enum_iterator::Sequence;
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

