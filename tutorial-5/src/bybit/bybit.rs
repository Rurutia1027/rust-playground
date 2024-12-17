use std::{cmp::Ordering, ops::Sub};

use anyhow::{Context, Result};
use backoff::{self, Error, ExponentialBackoff};
use chrono::{DateTime, Duration, TimeZone, Utc};
use format_url::FormatUrl;
use serde::Deserialize;
use tracing::{debug, info, warn};

use super::EthPrice;

// here we parse a fixed array of length 5 into a `BybitCandle`
#[derive(Debug, Deserialize)]
struct BybitCandle {
    timestamp: String,
    open: String,
    #[allow(unused)]
    high: String,
    #[allow(unused)]
    low: String,
    #[allow(unused)]
    close: String,
}

#[derive(Debug, Deserialize)]
struct BybitPriceResult {
    #[allow(unused)]
    symbol: String,
    #[allow(unused)]
    category: String,
    list: Vec<BybitCandle>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BybitPriceResponse {
    #[allow(unused)]
    ret_code: i64,
    #[allow(unused)]
    ret_msg: String,
    result: BybitPriceResult,
}

const BYBIT_API: &str = "https://api.bybit.com";

// 1min candles of index price made up of Kraken, Coinbase, Bitstamp & Bitfinex spot price
async fn get_eth_candles(
    client: &reqwest::Client,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<EthPrice>> {
    let url = FormatUrl::new(BYBIT_API)
        .with_path_template("/v5/market/index-price-kline")
        .with_query_params(vec![
            ("symbol", "ETHUSD"),
            ("interval", "1"),
            ("start", &start.timestamp_millis().to_string()),
            ("end", &end.timestamp_millis().to_string()),
        ])
        .format_url();

    println!("format url {:?}", url);

    backoff::future::retry(ExponentialBackoff::default(), || async {
            let candles = send_eth_price_request(client, &url).await.map_err(|err| {
                info!(%err, "error sending request to bybit, retrying");
                Error::transient(err)
            })?;
            if candles.is_empty() {
                warn!(%start, %end, "bybit returned no candles for the requested period");
            }
    
            Ok(candles)
        })
        .await
}

pub async fn send_eth_price_request(
    client: &reqwest::Client,
    url: &str,
) -> Result<Vec<EthPrice>> {
    debug!("sending request to {}", url);

    let body = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json::<BybitPriceResponse>()
        .await?;

    let candles: Vec<EthPrice> =
        body.result
            .list
            .iter()
            .map(|c| {
                // first, extract response body timestamp ret value into i64 in milliseconcs
                let timestamp_millis = c.timestamp.parse::<i64>().expect(
                    "expect bybit candles to contain integer timestamps",
                );

                // then, converted i64 timestamp into Utc formatted DateTime
                // during converting extract the earliest value
                let timestamp = Utc
                .timestamp_millis_opt(timestamp_millis)
                .earliest()
                .expect(
                    "expect bybit candles to contain millisecond timestamps",
                );

                let usd = c.open.parse::<f64>().expect(
                    "expect bybit cancles to contain float used prices",
                );
                // finally take the previous parsed timestamp & usd value to EthPrice instance
                EthPrice { timestamp, usd }
            })
            .rev()
            .collect();

    Ok(candles)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore = "failing in CI, probably temporary, try re-enabling"]
    #[tokio::test]
    async fn includes_end_timestamp_test() {
        let client = &reqwest::Client::new();
        let start = "2024-12-17T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let end = "2024-12-17T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let ret = get_eth_candles(client, start, end).await.unwrap();
        assert!(ret.len() > 0);
        assert_eq!(ret[0].timestamp, start); 
        println!("ret item {:?}", ret[0]); 
    }
}
