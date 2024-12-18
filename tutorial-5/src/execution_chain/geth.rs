// add series of test codes for fetching
// ethereum block, transaction, account, smart contracts and receipts datasets via local Geth exposed WebSocket endpoint

#[cfg(test)]
mod tests {
    use async_tungstenite::{
        tokio::{connect_async, ConnectStream},
        tungstenite::Message,
        WebSocketStream,
    };
    use futures::{SinkExt, StreamExt};
    use serde_json::{json, Value};
    use std::time::{Duration, Instant};

    fn generate_json_rpc_request_body(
        method: &str,
        params: &[Value],
        id: u64,
    ) -> String {
        let msg = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id,
        });

        msg.to_string()
    }

    // WebSocket Connection Setup function
    async fn connect_to_ethereum_node(
        ws_url: &str,
    ) -> WebSocketStream<ConnectStream> {
        let (ws_stream, _) = connect_async(ws_url)
            .await
            .expect("Failed to connect to Ethereum node");
        ws_stream
    }

    // Send JSON-RPC Request function
    async fn send_request(
        ws: &mut WebSocketStream<ConnectStream>,
        request: Value,
    ) -> Value {
        ws.send(Message::Text(request.to_string()))
            .await
            .expect("Failed to send request to Ethereum Node");
        if let Some(Ok(Message::Text(response))) = ws.next().await {
            serde_json::from_str(&response)
                .expect("Failed to parse JSON response")
        } else {
            panic!("No response received!")
        }
    }

    async fn fetch_block_by_number(
        ws: &mut WebSocketStream<ConnectStream>,
        block_number: u64,
    ) -> Value {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockNumber",
            // true -> enable fetch transaction
            "params": [format!("0x{:x}", block_number), true],
            "id": 1
        });

        send_request(ws, request).await
    }

    async fn fetch_transaction_by_hash(
        ws: &mut WebSocketStream<ConnectStream>,
        tx_hash: &str,
    ) -> Value {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "eth_getTransactionByHash",
            "params": [tx_hash],
            "id": 2
        });

        send_request(ws, request).await
    }

    async fn fetch_transaction_receipt(
        ws: &mut WebSocketStream<ConnectStream>,
        tx_hash: &str,
    ) -> Value {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "eth_getTransactionReceipt",
            "params": [tx_hash],
            "id": 3
        });

        send_request(ws, request).await
    }

    async fn fetch_logs(
        ws: &mut WebSocketStream<ConnectStream>,
        address: &str,
        from_block: u64,
        to_block: u64,
    ) -> Value {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "eth_getLogs",
            "params": [{
                "address": address,
                "fromBlock": format!("0x{:x}", from_block),
                "toBlock": format!("0x{:x}", to_block),
            }],
            "id": 4
        });
        send_request(ws, request).await
    }

    #[tokio::test]
    async fn hello_world_test() {
        assert!(true);
        println!("hello world");
    }
}
