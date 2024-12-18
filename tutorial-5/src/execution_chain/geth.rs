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
    use futures::{SinkExt, StreamExt};
    use serde_json::{json, Value};
    use test_context::AsyncTestContext;
    pub struct WebSocketTestHandler {
        pub ws: Option<
            async_tungstenite::WebSocketStream<
                async_tungstenite::tokio::ConnectStream,
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
                ws: Some(ws),
                url: url.to_string(),
            }
        }

        pub async fn shutdown(self) {
            // do nothing
        }

        pub async fn send_request(&mut self, request: Value) -> Value {
            let ws = self.ws.as_mut().expect("WebSocket is not initialized");
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
        }
    }

    #[async_trait]
    impl AsyncTestContext for WebSocketTestHandler {
        async fn setup() -> Self {
            let url = "ws://127.0.0.1:8546"; // Example WebSocket URL
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

    #[tokio::test]
    async fn hello_world_test() {
        assert!(true);
        println!("hello world");
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

    // https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getblockbynumber
    #[tokio::test]
    async fn eth_getblockbynumber_test() {
        let url = "ws://127.0.0.1:8546".to_string();
    }

    // todo()! share the ws via the test_context so that this WebSocket can be thread-safely shared among diffrent async test cases
}
