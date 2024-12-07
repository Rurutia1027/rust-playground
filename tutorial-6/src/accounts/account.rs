/// NOTE: etherscan api-endpoints of account: https://docs.etherscan.io/api-endpoints/accounts
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
}
