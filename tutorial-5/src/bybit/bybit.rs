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

// here: &'_ EthPrice, this return type of the function is a reference to an EthPrice object
// and the lifetime of the reutrn refrence is tied to the input reference of the &[EthPrice]
// as long as the [EthPrice] exists, the return reference will be existed.
fn find_closest_price(
    prices: &[EthPrice],
    target_minute_rounded: DateTime<Utc>,
) -> &'_ EthPrice {
    let mut best_distance = None;
    let mut best_candidate = None;

    for price in prices {
        let distance = (target_minute_rounded - price.timestamp)
            .num_seconds()
            .abs();

        match best_distance {
            None => {
                // here we init the best price and candidate's init value
                best_distance = Some(distance);
                best_candidate = Some(price);
            }

            Some(current_best) => match distance.cmp(&current_best) {
                Ordering::Less => {
                    // if current distance < best distance
                    // then update the current distance value and current candidate -> best distance, best candidate
                    best_candidate = Some(price);
                    best_distance = Some(distance);
                }

                Ordering::Greater => {
                    // return value are sorted from oldest -> youngest,
                    // if we get gerater value, stop searching and break is ok
                    break;
                }

                //
                Ordering::Equal => (),
            },
        }
    }

    // if not best candiate found print message to warn
    best_candidate.expect("one to be closet for non-empty prices")
}

// Function to return current 1 minute's candle open price
pub async fn get_eth_price(client: &reqwest::Client) -> Result<EthPrice> {
    // first get [start, end] end's current timestamp (current timestamp value)
    // then let end timestamp minus 1 minute as the value of query start timestamp value
    let end = Utc::now();
    let start = end.sub(Duration::minutes(1));
    get_eth_candles(client, start, end).await.and_then(|es| {
        es.into_iter()
            .last()
            .context("tried to retrieve last element in empty array")
    })
}

#[cfg(test)]
mod tests {
    use chrono::{Date, DurationRound};

    use super::*;

    #[ignore = "failing in CI, probably temporary, try re-enabling"]
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

    #[ignore = "failing in CI, probably temporary, try re-enabling"]
    #[tokio::test]
    async fn returns_in_progress_candle_test() {
        let client = &reqwest::Client::new();
        let now = Utc::now();

        // like current timestamp is
        // 2024-12-17T04:25:55Z --> upper 2024-12-17T04:26:00Z
        // 2024-12-17T04:25:55Z --> down 2024-12-17T04:25:00Z
        let rounded_down_timestamp =
            now.duration_trunc(Duration::minutes(1)).unwrap();
        let rounded_up_timestamp =
            now.duration_round(Duration::minutes(1)).unwrap();
        let ret = get_eth_price(client).await.unwrap();
        assert_eq!(ret.timestamp, rounded_down_timestamp);
    }

    #[test]
    fn find_closest_price_test() {
        let price_1 = EthPrice {
            timestamp: "2024-12-17T04:25:55Z".parse::<DateTime<Utc>>().unwrap(),
            usd: 0.0,
        };
        let price_2 = EthPrice {
            timestamp: "2024-12-10T04:25:55Z".parse::<DateTime<Utc>>().unwrap(),
            usd: 2.0,
        };

        let prices_vector = vec![price_1, price_2];
        let closest = find_closest_price(
            &prices_vector,
            "2024-12-09T04:25:55Z".parse::<DateTime<Utc>>().unwrap(),
        );

        assert_eq!(prices_vector[1], *closest);
    }
}
