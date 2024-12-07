use std::ptr::addr_eq;

/// NOTE: etherscan api-endpoints of account: https://docs.etherscan.io/api-endpoints/accounts
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct SingleBalanceResponse {
    status: String,  // http status
    message: String, // response description message
    result: String, // balance value in unit of Wei (1 ETH = 10^18 Wei = 10^9 Gwei)
}

// Get Ether Balance for a Single Address
// This API provides the balance value of the given address(wallet address)
pub async fn get_ether_balance(
    address: &str,
    api_key: &str,
) -> Result<SingleBalanceResponse, reqwest::Error> {
    let url = format!("https://api.etherscan.io/api?module=account&action=balance&address={}&tag=latest&apikey={}",
    address, api_key);
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<SingleBalanceResponse>()
        .await?;

    println!("Ether Balance: {:?}", &response.result);
    Ok(response)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Balance {
    account: String,
    balance: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct MultiBalanceResponse {
    status: String,
    message: String,
    result: Vec<Balance>,
}

pub async fn get_ether_balances(
    addr_vec: &Vec<String>,
    api_key: &str,
) -> Result<MultiBalanceResponse, reqwest::Error> {
    let client = Client::new();
    let addresses = addr_vec.join(&",".to_string());
    println!("addresses : {}", addresses);

    let url = format!("https://api.etherscan.io/api?module=account&action=balancemulti&address={}&tag=latest&apikey={}",
                        addresses, api_key);
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<MultiBalanceResponse>()
        .await?;

    Ok(response)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct NormalTransaction {
    blockNumber: String,
    timeStamp: String,
    hash: String,
    nonce: String,
    blockHash: String,
    transactionIndex: String,
    from: String,
    to: String,
    value: String,
    gas: String,
    gasPrice: String,
    isError: String,
    txreceipt_status: String,
    input: String,
    contractAddress: String,
    cumulativeGasUsed: String,
    gasUsed: String,
    confirmations: String,
    methodId: String,
    functionName: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct NormalTransactionsResponse {
    status: String,
    message: String,
    result: Vec<NormalTransaction>,
}

pub async fn get_normal_transaction_via_address(
    address: &str,
    api_key: &str,
) -> Result<NormalTransactionsResponse, reqwest::Error> {
    let client = Client::new();
    let url = format!(
        "https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock=0&endblock=99999999&page=1&offset=10&sort=asc&apikey={}",
            address, api_key);
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<NormalTransactionsResponse>()
        .await?;
    Ok(response)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct InternalTransction {
    blockNumber: String,
    timeStamp: String,
    hash: String,
    from: String,
    to: String,
    contractAddress: String,
    input: String,
    #[serde(rename = "type")]
    type_str: String,
    gas: String,
    gasUsed: String,
    traceId: String,
    isError: String,
    errCode: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct InternalTransctionsResponse {
    status: String,
    message: String,
    result: Vec<InternalTransction>,
}

pub async fn get_internal_transactions_via_address(
    address: &str,
    api_key: &str,
) -> Result<InternalTransctionsResponse, reqwest::Error> {
    let client = Client::new();
    let url = format!(
        "https://api.etherscan.io/api?module=account&action=txlistinternal&address={}&startblock=0&endblock=2702578&page=1&offset=10&sort=asc&apikey={}",
        address, api_key
    );

    // println!("internal transaction addr {}", &url);

    let response = client
        .get(&url)
        .send()
        .await?
        .json::<InternalTransctionsResponse>()
        .await?;
    // println!("internal transaction response : {:?}", response);
    Ok(response)
}

// --
#[derive(Serialize, Deserialize, Clone, Debug)]
struct InternalTransctionV2 {
    blockNumber: String,
    timeStamp: String,
    from: String,
    to: String,
    value: String,
    contractAddress: String,
    input: String,
    #[serde(rename = "type")]
    type_str: String,
    gas: String,
    gasUsed: String,
    isError: String,
    errCode: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct InternalTransctionsResponseV2 {
    status: String,
    message: String,
    result: Vec<InternalTransctionV2>,
}

pub async fn get_internal_transactions_via_transaction_hash(
    transaction_hash: &str,
    api_key: &str,
) -> Result<InternalTransctionsResponseV2, reqwest::Error> {
    let client = Client::new();
    let url = format!(
        "https://api.etherscan.io/api?module=account&action=txlistinternal&txhash={}&apikey={}", transaction_hash, api_key);

    let response = client
        .get(&url)
        .send()
        .await?
        .json::<InternalTransctionsResponseV2>()
        .await?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use anyhow::Context;

    use super::*;

    #[tokio::test]
    async fn test_query_ehther_balance() {
        let api_key = "UAA5Y5IKQBHH3HUCS9GWA723666GGMEEN6".to_string();
        let address = "0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae".to_string();

        let ret = get_ether_balance(&address, &api_key).await.context(
            "Failed to get response body from etherscan endopoint account api ",
        ).unwrap();
        println!("ret content : {:?}", ret);
        assert_eq!(ret.message, "OK");
        assert_eq!(ret.status, "1");

        // here convert data from String into Float
        assert_ne!(ret.result, "0");
    }

    #[tokio::test]
    async fn test_query_multi_ether_balances() {
        let api_key = "UAA5Y5IKQBHH3HUCS9GWA723666GGMEEN6".to_string();
        let mut addr_vec: Vec<String> = Vec::new();

        addr_vec.push("0xddbd2b932c763ba5b1b7ae3b362eac3e8d40121a".to_owned());
        addr_vec.push("0x63a9975ba31b0b9626b34300f7f627147df1f526".to_owned());
        addr_vec.push("0x198ef1ec325a96cc354c7266a038be8b5c558f67".to_owned());
        addr_vec.push("0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae".to_owned());

        let ret = get_ether_balances(&addr_vec, &api_key)
            .await
            .context("Unable to get response from multiple balance query API endpoint")
            .unwrap();

        let res_vec: &Vec<Balance> = &ret.result;
        for res in res_vec {
            println!("account: {}, balance: {}", res.account, res.balance);
        }

        println!("ret content: {:?}", ret);
    }

    #[tokio::test]
    async fn test_query_normal_transactions_via_address() {
        let api_key = "UAA5Y5IKQBHH3HUCS9GWA723666GGMEEN6".to_string();
        let address = "0xc5102fE9359FD9a28f877a67E36B0F050d81a3CC".to_string();

        let ret = get_normal_transaction_via_address(&address, &api_key)
            .await
            .context(
                "Failed to get response from normal transaction query endpoint",
            )
            .unwrap();
        //println!("normal trans ret {:?}", ret);
        assert_eq!(ret.status, "1");
        assert_eq!(ret.message, "OK");

        let normal_trans_vec = &ret.result;
        for normal_transaction in normal_trans_vec {
            // println!("Normal Transaction {:?}", normal_transaction);

            // first we converted the struct object instance into json string via serde_json
            let trans_json_str =
                serde_json::to_string(&normal_transaction).unwrap();

            // then converted the serde json string into serde json value instance
            let trans_json_value: serde_json::Value =
                serde_json::from_str(&trans_json_str).unwrap();

            // then try to covnert the serde json object's key, value pairs and traverse each key's corresponding value
            if let serde_json::Value::Object(map) = trans_json_value {
                for (k, v) in map.iter() {
                    // then verify the value should not be null
                    assert!(!v.is_null());
                }
            }
        }
    }

    #[tokio::test]
    async fn test_query_internal_transactions_via_address() {
        let api_key = "UAA5Y5IKQBHH3HUCS9GWA723666GGMEEN6".to_string();
        let address = "0x2c1ba59d6f58433fb1eaee7d20b26ed83bda51a3".to_string();
        let response =
            get_internal_transactions_via_address(&address, &api_key)
                .await
                .context(
                    "Failed to query internal transactions via given address",
                )
                .unwrap();
        // println!("response content for internal transaciton: {:?}", response);
        assert_eq!(response.status, "1");
        assert_eq!(response.message, "OK");

        // here we traverse each item in response#result
        // and verify each content is not null
        let internal_transation_vec = &response.result;
        for internal_transaction in internal_transation_vec {
            // convert item into string
            let trans_json_str =
                serde_json::to_string(&internal_transaction).unwrap();

            // convert json string into serde json object
            let trans_json_obj: serde_json::Value =
                serde_json::from_str(&trans_json_str).unwrap();

            // extract key in json and traverse and fetch each value
            if let serde_json::Value::Object(map) = trans_json_obj {
                for (k, v) in map.iter() {
                    assert!(!v.is_null());
                }
            }
        }
    }

    #[tokio::test]
    async fn test_query_internal_transactions_via_transaction_hash() {
        let tx_hash = "0x40eb908387324f2b575b4879cd9d7188f69c8fc9d87c901b9e2daaea4b442170".to_string();
        let api_key = "UAA5Y5IKQBHH3HUCS9GWA723666GGMEEN6".to_string();
        let response = get_internal_transactions_via_transaction_hash(
            &tx_hash, &api_key,
        )
        .await
        .context(
            "Failed to query internal transctions by given transaction hash",
        )
        .unwrap();

        assert_eq!(response.status, "1");
        assert_eq!(response.message, "OK");

        let trans_vec = &response.result;
        for trans in trans_vec {
            let trans_json_str = serde_json::to_string(&trans).unwrap();
            let trans_json_value: serde_json::Value =
                serde_json::from_str(&trans_json_str).unwrap();

            if let serde_json::Value::Object(map) = trans_json_value {
                for (_, v) in map.iter() {
                    assert!(!v.is_null());
                }
            }
        }
    }
}
