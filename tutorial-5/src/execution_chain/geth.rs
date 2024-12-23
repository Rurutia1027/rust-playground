// add series of test codes for fetching
// ethereum block, transaction, account, smart contracts and receipts datasets via local Geth exposed WebSocket endpoint
#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use async_tungstenite::{
        tokio::{connect_async, ConnectStream},
        tungstenite::Message,
        WebSocketStream,
    };
    use axum::http::response;
    use futures::{SinkExt, StreamExt};
    use serde_json::{json, Value};
    use std::sync::Arc;
    use test_context::{test_context, AsyncTestContext};
    use tokio::sync::Mutex;

    #[derive(Debug, Clone)]
    pub struct WebSocketTestHandler {
        pub ws: Option<
            Arc<
                Mutex<
                    async_tungstenite::WebSocketStream<
                        async_tungstenite::tokio::ConnectStream,
                    >,
                >,
            >,
        >,
        url: String,
    }

    impl WebSocketTestHandler {
        pub async fn new(url: &str) -> Self {
            let (ws, _) = connect_async(url.clone())
                .await
                .expect("Failed to connect to WebSocket node");

            WebSocketTestHandler {
                ws: Some(Arc::new(Mutex::new(ws))),
                url: url.to_string(),
            }
        }

        pub async fn shutdown(mut self) {
            if let Some(ws) = self.ws.take() {
                let mut ws = ws.lock().await;
                ws.close(None).await.expect("Failed to close WebSocket");
            }
        }

        pub async fn send_request(&mut self, request: Value) -> Value {
            if let Some(ws) = self.ws.take() {
                let mut ws = ws.lock().await;
                ws.send(Message::Text(request.to_string()))
                    .await
                    .expect("Failed to send message");

                let response = ws
                    .next()
                    .await
                    .expect("Failed to receive response")
                    .expect("Failed to parse response");

                if let Message::Text(text) = response {
                    serde_json::from_str(&text)
                        .expect("Failed to parse JSON response")
                } else {
                    panic!("Expected text message from WebSocket")
                }
            } else {
                panic!("Expected current function get Websocket Successfully")
            }
        }
    }

    #[async_trait]
    impl AsyncTestContext for WebSocketTestHandler {
        async fn setup() -> Self {
            // this is our local setup Geth Websocket Endpoint
            let url = "ws://127.0.0.1:8546";
            WebSocketTestHandler::new(url).await
        }

        async fn teardown(self) {
            self.shutdown().await;
        }
    }

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
        topics: Vec<&str>,
    ) -> Value {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "eth_getLogs",
            "params": [{
                "topics": topics,
            }],
            "id": 74
        });
        send_request(ws, request).await
    }

    // https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getlogs
    #[tokio::test]
    async fn fetch_logs_test() {
        let url = "ws://127.0.0.1:8546".to_string();
        let mut ws = connect_to_ethereum_node(&url).await;
        let topics = vec![{
            "0x000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b"
        }];
        let response = fetch_logs(&mut ws, topics).await;
        // println!("response of fetch log test is : {:?}", response);
        let ret = response.get("result");
        assert!(ret.unwrap().is_array());
    }

    // https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_blocknumber
    #[tokio::test]
    async fn eth_blocknumber_test() {
        let url = "ws://127.0.0.1:8546".to_string();
        let mut ws = connect_to_ethereum_node(&url).await;
        let request = json!({
            "jsonrpc": "2.0",
            "method": "eth_blocknumber",
            "params": [],
            "id": 1
        });

        let response = send_request(&mut ws, request).await;
        println!("get response {:?}", response);
    }

    // https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getblockbynumber
    #[tokio::test]
    async fn eth_getblockbynumber_test() {
        let url = "ws://127.0.0.1:8546".to_string();
        let mut ws = connect_to_ethereum_node(&url).await;
        let block_number: u64 = 436;
        let response = fetch_block_by_number(&mut ws, block_number).await;
        if let Some(obj_content) = response.get("error") {
            if let Some(message) = obj_content.get("message") {
                let msg_str = message.as_str().unwrap_or("Not a String");
                println!("msg str content : {:?}", msg_str);
                assert!(msg_str.len() > 0);
            }
        }
    }
}
