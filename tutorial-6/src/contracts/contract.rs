use anyhow::Context;
use futures::FutureExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ContractABIResonse {
    status: String,
    message: String,
    result: Option<String>,
}

pub async fn get_contract_ABI_via_addresses(
    addr_list: &Vec<String>,
    api_key: &str,
) -> Result<Vec<String>, reqwest::Error> {
    let client = Client::new();
    let mut ret_vec: Vec<String> = Vec::new();

    for addr in addr_list {
        let url = format!(
            "https://api.etherscan.io/api?module=contract&action=getabi&address={}&apikey={}",
            addr, api_key
        );

        let ret = client
            .get(&url)
            .send()
            .await?
            .json::<ContractABIResonse>()
            .await
            .context("Failed to extract Result From Response")
            .map(|item| {
                item.result
                    .context("Failed to retrive inner String Contract Content")
                    .unwrap()
            })
            // .and_then(|item| {
            //     item.result
            //         .into_iter()
            //         .map(|s| {
            //             serde_json::from_str::<Value>(&s).map_err(|e| {
            //                 anyhow::anyhow!(
            //                     "Failed to parse String into JSON: {:?}",
            //                     e
            //                 )
            //             })
            //         })
            //         .collect::<Result<Vec<Value>, _>>()
            // })
            .context("Failed to retreive result json from response body")
            .unwrap();

        ret_vec.push(ret.clone());
    }
    println!("total ret length {}", ret_vec.len());

    Ok(ret_vec)
}

// we ignore message, and status fields
#[derive(Deserialize, Serialize, Debug)]
struct ContractSrcResponse {
    result: Vec<SmartContractSrc>,
}

// we ignore the fields we do not care
#[derive(Deserialize, Serialize, Debug)]
struct SmartContractSrc {
    SourceCode: String,
    ABI: String,
    ContractName: String,
    CompilerVersion: String,
    EVMVersion: String,
}

pub async fn get_contract_src_code_via_address(
    address: &str,
    api_key: &str,
) -> Result<ContractSrcResponse, reqwest::Error> {
    let url = format!(
        "https://api.etherscan.io/api?module=contract&action=getsourcecode&address={}&apikey={} ",
        address, api_key
    );
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<ContractSrcResponse>()
        .map(|item| {
            item.context("Failed to convert response into Json Instance")
                .unwrap()
        })
        .await;

    Ok(response)
}

pub fn print_keywords_from_json_contract(contract_abi: &Value) {
    if let Value::Array(functions) = contract_abi {
        for func in functions {
            if let Value::Object(fields) = func {
                // Extract the function name here
                if let Some(Value::String(name)) = fields.get("name") {
                    println!("Function Name: {}", name);
                }

                // Extract the type of the element (function, constructor, event, etc.)
                if let Some(Value::String(typ)) = fields.get("type") {
                    println!("Type: {}", typ);
                }

                // Extract input value from fields
                if let Some(Value::Array(inputs)) = fields.get("inputs") {
                    println!("Inputs: ");
                    for input in inputs {
                        if let Value::Object(input_fields) = input {
                            if let Some(Value::String(param_name)) =
                                input_fields.get("name")
                            {
                                if let Some(Value::String(param_type)) =
                                    input_fields.get("type")
                                {
                                    println!(
                                        "- {}: {}",
                                        param_name, param_type
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("The provided JSON is not an array, not a valid smart contract json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_contract_src_code_via_address() {
        let api_key = "UAA5Y5IKQBHH3HUCS9GWA723666GGMEEN6".to_string();
        let address = "0xBB9bc244D798123fDe783fCc1C72d3Bb8C189413".to_string();

        let res  = get_contract_src_code_via_address(
            &address, &api_key,
        )
        .await
        .context(
            "Failed to query source code of smart contract via given adddress",
        )
        .unwrap();

        println!("Content of Smart Contract {:?}", res.result);
        assert!(res.result.len() > 0);
    }

    // #[tokio::test]
    async fn test_query_contract_ABI_via_addresses() {
        let api_key = "UAA5Y5IKQBHH3HUCS9GWA723666GGMEEN6".to_string();
        let mut addr_list: Vec<String> = Vec::new();
        // contract address coming from: https://etherscan.io/contractsVerified
        addr_list
            .push("0xBB9bc244D798123fDe783fCc1C72d3Bb8C189413".to_string());
        addr_list
            .push("0x281e580502788A32d5333f6901524833e2d3F0e5".to_string());
        addr_list
            .push("0x964206BD32aD6d14e18d73749767b4102297C857".to_string());
        addr_list
            .push("0x0874e88CC65CC679A49A6a9720bc9Fd3896D1Be3".to_string());
        addr_list
            .push("0x5C28A91515f72442460d2BDB62De92F11175bBCb".to_string());

        let res = get_contract_ABI_via_addresses(&addr_list, &api_key)
            .await
            .context("Failed to query contract via address list")
            .unwrap();

        for contract in &res {
            let contract_json =
                serde_json::from_str::<serde_json::Value>(&contract)
                    .context("Failed to convert text contract into json")
                    .unwrap();
            // print_keywords_from_json_contract(&contract_json);
            println!("contract_json: {:?}", contract_json);
        }

        assert!(res.len() > 0);
    }
}
