use crate::bybit::EthPrice;
use crate::units::UsdNewtype;
use async_trait::async_trait;
use chrono::{DateTime, Duration, DurationRound, Utc};
use sqlx::PgPool;
use thiserror::Error;

// This Postgres Store isolates operations on the `eth_prices` database table from the service layer.
// It defines a series of operations specific to the `eth_prices` table.
pub struct EthPriceStorePostgres {
    db_pool: PgPool,
}

impl EthPriceStorePostgres {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GetEthPriceError {
    #[error("closest price to given block was too old")]
    PriceTooOld,
}

// From a Java developer's perspective, this is similar to:
// - Declaring a service layer interface, e.g., `Service`, where we define a set of actions(functions).
// - Creating a `ServiceImpl` to implement the interface(trait) and provide specific logic for each functin.

// The reason we abstract a series of `eth_prices` table operations inside `EthPriceStore`
// is to maintain compatibility with multiple data sources, such as MySQL, Redis, GraphSQL, etc.
// This approach ensures that data source implementation details do not interfere with the business logic layer.

#[async_trait]
pub trait EthPriceStore {
    async fn average_from_time_range(
        &self,
        start_timestamp: DateTime<Utc>,
        end_timestamp: DateTime<Utc>,
    ) -> UsdNewtype;

    // this is rely on block series of functions which haven't implemented yet
    // all block node associated ether price query in this file
    // are based on block's timestamp value
    // add a todo!() here
    async fn average_from_block_plus_time_range(
        &self,
        // todo!() block: &ExecutionNodeBlock,
        // todo!() time_frame: &TimeFrame,
    ) -> UsdNewtype;

    // Retrieve the freshest Ether price from database `ether_prices` table
    async fn get_most_recent_price(&self) -> sqlx::Result<EthPrice>;

    // Save value to `ether_prices` table
    async fn store_price(&self, timestamp: &DateTime<Utc>, usd: f64);

    // Retrieves the 24-hour average Ether price
    #[allow(dead_code)]
    async fn get_h24_average(&self) -> f64;

    // Retrieve Ether price from the `eth_prices` table for 24 hours prior to the current timestamp.
    async fn get_price_h24_ago(&self, duration: &Duration) -> Option<EthPrice>;

    // Retrieve latest minutes ether price
    async fn get_eth_price_by_minute(
        &self,
        minute: DateTime<Utc>,
    ) -> Option<f64>;

    // Retrieve closet(timestamp)'s ether price via Block
    async fn get_closest_price_by_block(
        &self,
        // todo!(),  block: &ExecutionNodeBlock,
    ) -> Result<f64, GetEthPriceError>;

    // Retrieve ether price via Block
    async fn get_eth_price_by_block(
        &self,
        // todo!(),  block: &ExecutionNodeBlock,
    ) -> Result<f64, GetEthPriceError>;
}

#[async_trait]
impl EthPriceStore for EthPriceStorePostgres {
    async fn average_from_time_range(
        &self,
        start_timestamp: DateTime<Utc>,
        end_timestamp: DateTime<Utc>,
    ) -> UsdNewtype {
        UsdNewtype(0.0)
    }

    /**
     * To make compiler happy with this sqlx::query_as!(...), we need to exeucte commands below:
     * # this install sql-cli
     * cargo install sqlx-cli --no-default-features --features postgres
     *
     * # this add a migrate file on local
     * cargo sqlx migrate add_eth_prices_table
     *
     * # add eth_prices sql commands to generated migration file: migrations **_add_eth_prices_table
     * -- Add migration script here
     * CREATE TABLE IF NOT EXISTS eth_prices (
     * timestamp timestamptz PRIMARY KEY,
     * ethusd float8 NOT NULL
     * )
     *
     * # this execute migrate file's inner sql command on the deaultdb database
     * sqlx migrate run --database-url postgres://admin:admin@localhost:5432/defaultdb
     *
     */
    async fn get_most_recent_price(&self) -> sqlx::Result<EthPrice> {
        sqlx::query_as!(
            EthPrice,
            "
            SELECT 
                timestamp, ethusd as usd
            FROM
                eth_prices
            ORDER BY timestamp DESC
            LIMIT 1
            ",
        )
        .fetch_one(&self.db_pool)
        .await
    }

    async fn store_price(&self, timestamp: &DateTime<Utc>, usd: f64) {
        sqlx::query!(
            "
            INSERT INTO 
                eth_prices (timestamp, ethusd)
            VALUES($1, $2)
            ON CONFLICT (timestamp) DO UPDATE SET
                ethusd = excluded.ethusd 
            ",
            timestamp,
            usd
        )
        .execute(&self.db_pool)
        .await
        .unwrap();
    }

    #[allow(dead_code)]
    async fn get_h24_average(&self) -> f64 {
        sqlx::query!(
            r#"
        SELECT 
            AVG(ethusd) as "average!"
        FROM
            eth_prices
        WHERE timestamp >= NOW() - '24 hours'::INTERVAL
        "#,
        )
        .fetch_one(&self.db_pool)
        .await
        .unwrap()
        .average
    }

    // query ether price value from `eth_prices` which with current timestamp closest to
    // min(current timestamp - 24 hourse before)
    async fn get_price_h24_ago(
        &self,
        age_limit: &Duration,
    ) -> Option<EthPrice> {
        sqlx::query_as!(
            EthPrice,
            "
            WITH 
              eth_price_distances AS (
              SELECT 
                ethusd,
                timestamp,
                ABS (
                  EXTRACT(
                      epoch
                      FROM
                        (timestamp - (NOW() - '24 hours':: INTERVAL))
                  )
                ) AS distance_seconds
                FROM 
                  eth_prices
                ORDER BY
                  distance_seconds ASC
            )
            SELECT ethusd as usd, timestamp
            FROM eth_price_distances
            WHERE distance_seconds <= $1
            LIMIT 1
            ",
            age_limit.num_seconds() as i32
        )
        .fetch_optional(&self.db_pool)
        .await
        .unwrap()
    }

    async fn get_eth_price_by_minute(
        &self,
        timestamp: DateTime<Utc>,
    ) -> Option<f64> {
        sqlx::query!(
            r#"
            SELECT ethusd
            FROM eth_prices
            WHERE timestamp = $1
            "#,
            timestamp
        )
        .fetch_optional(&self.db_pool)
        .await
        .unwrap()
        .map(|row| row.ethusd)
    }

    // -- todo!() --
    async fn average_from_block_plus_time_range(
        &self,
        // todo!() block: &ExecuteNodeBlock,
        // todo!() time_frame: &TimeFrame
    ) -> UsdNewtype {
        UsdNewtype(0.0)
    }

    async fn get_closest_price_by_block(
        &self,
        // todo!() block: &ExecutionNodeBlock,
    ) -> Result<f64, GetEthPriceError> {
        Ok(0.0)
    }

    async fn get_eth_price_by_block(
        &self,
        // todo!() block: &ExecutionNodeBlock,
    ) -> Result<f64, GetEthPriceError> {
        Ok(0.0)
    }
}
