mod blocks;
mod decoders;
mod heads;
mod transaction_receipts;

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
    u16,
};

use anyhow::Result;
use async_tungstenite::{
    tokio::{connect_async, TokioAdapter},
    tungstenite::Message,
    WebSocketStream,
};

use futures::{
    channel::oneshot, future::err, stream::SplitStream, TryFutureExt,
};
use futures::{
    stream::{FuturesOrdered, StreamExt},
    SinkExt,
};

use serde::Deserialize;
use serde_json::{json, Value};
use thiserror::Error;
use tokio::{net::TcpStream, sync::mpsc};

pub struct ExecutionNode {
    id_pool: Arc<Mutex<IdPool>>,
    message_rx_map: Arc<Mutex<MessageHandlers>>,
    message_tx: mpsc::Sender<Message>,
}

pub use blocks::BlockNumber;
pub use blocks::Difficulty;
pub use blocks::ExecutionNodeBlock;
pub use blocks::TotalDifficulty;
pub use heads::stream_heads_from;
pub use heads::stream_new_heads;

use self::transaction_receipts::TransactionReceipt;
use crate::env::ENV_CONFIG;
#[cfg(test)]
pub use blocks::tests::ExecutionNodeBlockBuilder;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct RpcError {
    code: i32,
    message: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RpcMessage {
    Error { error: RpcError, id: u16 },
    Result { id: u16, result: serde_json::Value },
}

impl RpcMessage {
    fn id(&self) -> u16 {
        match self {
            RpcMessage::Error { id, .. } => *id,
            RpcMessage::Result { id, .. } => *id,
        }
    }
}

struct IdPool {
    next_id: u16,
    in_use_ids: HashSet<u16>,
}

// here we define an error to wrap info for transaction unavailable this situation
#[derive(Error, Debug)]
#[error("transaction receipt unavailable for tx hash: {0}")]
pub struct TransactionReceiptUnavailable(String);

impl IdPool {
    fn new(size: usize) -> Self {
        Self {
            next_id: 0,
            in_use_ids: HashSet::with_capacity(size),
        }
    }

    fn get_next_id(&mut self) -> u16 {
        if self.in_use_ids.len() == self.in_use_ids.capacity() {
            panic!("execution node id pool exhaustted")
        }

        while self.in_use_ids.contains(&self.next_id) {
            self.next_id += 1;
        }

        self.in_use_ids.insert(self.next_id);

        self.next_id
    }

    fn free_id(&mut self, id: &u16) {
        self.in_use_ids.remove(id);
    }
}

type NodeMessageRx = SplitStream<
    WebSocketStream<
        async_tungstenite::stream::Stream<
            TokioAdapter<TcpStream>,
            TokioAdapter<tokio_native_tls::TlsStream<tokio::net::TcpStream>>,
        >,
    >,
>;

type MessageHandlers = HashMap<u16, oneshot::Sender<Result<Value, RpcError>>>;

async fn handle_messages(
    mut ws_rx: NodeMessageRx,
    message_rx_map: Arc<Mutex<MessageHandlers>>,
    id_pool: Arc<Mutex<IdPool>>,
) {
    while let Some(message_response) = ws_rx.next().await {
        // loop websocket received message stream as msg on by on in while loop
        // extract message from Result<Message, Error> Result<...>
        let msg = message_response.expect("expect websocket message to be ok");

        // case-1: receive a ping message
        if msg.is_ping() {
            continue;
        }

        // case-2: receive peer's message
        let msg_bytes = msg.into_data();
        let rpc_msg = serde_json::from_slice::<RpcMessage>(&msg_bytes)
            .expect("expect node messages to be JsonRpcMessages");

        // when we send a request to ethereum endpoint, we will let thread-safe shared instance of
        // id_pool generate a global(program scoped) unique id, and recrod the uid to the id_pool
        // then, at there when we receive peer's response message, we'll remove the uid from the id_pool
        let id = rpc_msg.id();
        id_pool.lock().unwrap().free_id(&id);

        // message_rx_map: <request_uid, request_message_handler>
        // message_rx_map & id_pool work together, both thread-safe and shared among all threads
        // They all can be updated outside the scope of current message-handler function
        // in this way, new requests(calls) will be marked by global unique id and saved to the id_pool.
        // The request/call's message type's handler(functions defined only handle specified kind of message)
        // will be registered/insert to the message_rx_map.

        // rx,tx we can treat them as a pair of <input/receiver/rx, output/sender/tx> mmap(memory-map pairs)
        // binded as an union as a channel. Every time when message handler(this function the handle_messages)
        // receives a neither non-ping nor rpc error message, it will unwrap the message
        // extract it's inner field's id field --> the uid registered in both hashmap(message_rx_map) and id_pool(unique id pool and generate)
        // Then extract registered channel#output as tx to send it to the tx,rx pair the channel
        let tx =
            message_rx_map.lock().unwrap().remove(&id).expect(
                "expect a message handler for every received message id",
            );

        match rpc_msg {
            RpcMessage::Error { error, .. } => tx.send(Err(error)).unwrap(),
            RpcMessage::Result { result, .. } => tx.send(Ok(result)).unwrap(),
        };
    } // while loop
}

impl ExecutionNode {
    async fn call(
        &self,
        method: &str,
        params: &Value,
    ) -> Result<serde_json::Value, RpcError> {
        // prepare uid for coming ethereum endpoint request
        let id = self.id_pool.lock().unwrap().get_next_id();

        // create request body in JSON-RPC
        let json = json!(
            {
                "jsonrpc": "2.0",
                "id": id,
                "method": method,
                "params": params
            }
        );

        // convert json into string as message
        let message = serde_json::to_string(&json).unwrap();
        println!("msessage request {}", message);

        // create sender/tx, receiver/rx pair as a channel instance
        let (tx, rx) = oneshot::channel();

        // insert the channle's sender side to hash map as value, and take the uid as the key thread safely
        self.message_rx_map.lock().unwrap().insert(id, tx);
        self.message_tx.send(Message::Text(message)).await.unwrap();

        rx.await.unwrap()
    }

    // create websocket connection via env's GETH_URL address
    pub async fn connect() -> Self {
        // here we create a thread-safe unique id pool which gonna be shared among multiple threads as member variables
        let id_pool_am = Arc::new(Mutex::new(IdPool::new(u16::MAX.into())));
        let message_rx_map =
            Arc::new(Mutex::new(HashMap::with_capacity(u16::MAX.into())));

        let (connected_socket, _) =
            connect_async(ENV_CONFIG.geth_url.as_ref().expect(
                "GETH_URL is required in env to connect to execution node",
            ))
            .await
            .unwrap();

        let (mut sink, stream) = connected_socket.split();
        // add a todo here: modify the call function to let sending request from thread union instad of main thread union
        // avoid block the system via some functions of ethereum calles
        // todo!() , but for now we follow the original logic is fine
        let default_panic = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            default_panic(info);
            std::process::exit(1);
        }));

        let id_pool_ref = id_pool_am.clone();
        let message_handlers_ref = message_rx_map.clone();
        tokio::spawn(async move {
            handle_messages(stream, message_handlers_ref, id_pool_ref).await;
        });

        let (message_tx, mut rx) = mpsc::channel(512);
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                sink.send(message).await.unwrap();
            }
        });

        ExecutionNode {
            id_pool: id_pool_am,
            message_rx_map,
            message_tx,
        }
    }

    pub async fn get_latest_block(&self) -> ExecutionNodeBlock {
        let value = self
            .call("eth_getBlockByNumber", &json!(("latest", false)))
            .await
            .unwrap();
        // println!("block content {:?}", value);
        serde_json::from_value::<ExecutionNodeBlock>(value).unwrap()
    }

    pub async fn get_block_by_hash(
        &self,
        hash: &str,
    ) -> Option<ExecutionNodeBlock> {
        self.call("eth_getBlockByHash", &json!((hash, false)))
            .await
            .map_or_else(
                |err| {
                    tracing::error!(
                        "eth_getBlockByHash bad response {:?}",
                        err
                    );
                    None
                },
                |value| {
                    serde_json::from_value::<Option<ExecutionNodeBlock>>(value)
                        .unwrap()
                },
            )
    }

    pub async fn get_block_by_number(
        &self,
        number: &BlockNumber,
    ) -> Option<ExecutionNodeBlock> {
        let hex_number = format!("0x{number:x}");
        println!("hex_number {}", hex_number);
        self.call("eth_getBlockByNumber", &json!((hex_number, false)))
            .await
            .map_or_else(
                |err| {
                    tracing::error!(
                        "eth_getBlockByNumber bad response {:?}",
                        err
                    );
                    None
                },
                |value| {
                    // println!("block content {:?}", value);
                    serde_json::from_value::<Option<ExecutionNodeBlock>>(value)
                        .unwrap()
                },
            )
    }

    pub async fn get_transaction_receipt(
        &self,
        tx_hash: &str,
    ) -> Result<TransactionReceipt, TransactionReceiptUnavailable> {
        self.call("eth_getTransactionReceipt", &json!((tx_hash)))
            .await
            .map(|value| {
                let receipt = serde_json::from_value::<
                    Option<TransactionReceipt>,
                >(value)
                .expect("expect a transaction receipt response to be JSON");
                match receipt {
                    Some(receipt) => Ok(receipt),
                    None => {
                        Err(TransactionReceiptUnavailable(tx_hash.to_string()))
                    }
                }
            })
            .unwrap()
    }

    pub async fn get_transaction_receipts_for_block(
        &self,
        block: &ExecutionNodeBlock,
    ) -> Result<Vec<TransactionReceipt>, TransactionReceiptUnavailable> {
        let mut receipt_futures = FuturesOrdered::new();

        for tx_hash in block.transactions.iter() {
            receipt_futures.push_back(self.get_transaction_receipt(&tx_hash));
        }

        let mut receipts = Vec::new();
        while let Some(receipt) = receipt_futures.next().await {
            match receipt {
                Ok(receipt) => {
                    receipts.push(receipt);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(receipts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn get_latest_block_test() {
        let node = ExecutionNode::connect().await;
        let _block = node.get_latest_block().await;
    }

    #[tokio::test]
    async fn get_block_by_number_test() {
        let node = ExecutionNode::connect().await;
        let block = node
            .get_block_by_number(&0)
            .await
            .expect("Expect get block instance");
        assert_eq!(block.number, 0x0);
        // here we got hash value: 0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3
        println!("block content {:?}", block.hash);
    }

    #[tokio::test]
    async fn get_block_by_hash_test() {
        let node = ExecutionNode::connect().await;
        let hash = "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3";
        let block = node
            .get_block_by_hash(hash)
            .await
            .expect("Expect extract block from query response body");
        assert_eq!(block.hash, hash);
    }

    #[tokio::test]
    async fn get_unavailable_block_by_hash_test() {
        let hash_value = "0x-hashvalue-does-not-exist";
        let node = ExecutionNode::connect().await;
        let block = node.get_block_by_hash(hash_value).await;
        assert_eq!(block, None);
    }

    #[tokio::test]
    async fn get_transaction_receipt_test() {
        // Geth is a node in peer-to-peer ethereum system, it is setup and connect to the other peers
        // ExecutionNode::connect this enables current program connect to local Geth process via WebSocket
        let node = ExecutionNode::connect().await;
        let block = node
            .get_block_by_number(&0)
            .await
            .expect("Expect get block instance");

        let tx_hash: Option<String> = if !block.transactions.is_empty() {
            Some(block.transactions[0].clone())
        } else {
            None
        };

        if let Some(tx_hash) = tx_hash {
            let receipt = node.get_transaction_receipt(&tx_hash).await.unwrap();
            assert_eq!(tx_hash, receipt.transaction_hash);
        } else {
            println!("Latest block's tx_hash field is empty")
        }
    }

    #[tokio::test]
    async fn get_transaction_receipts_for_block_test() {
        // take block as query condition to fetch all transaction items that are located in the queried block

        let node = ExecutionNode::connect().await;
        // to make sure test case stable running,
        // we always take the latest block as the query block
        let block = node.get_latest_block().await;

        // query transactions via the current block and it may empty since geth may always fetch the 0x0 block as the latest block in CI env
        let receipts = node
            .get_transaction_receipts_for_block(&block)
            .await
            .expect("expect receipts can be found");

        // consider in CI pipeline, receipts may not always fetch from peer node, we comment this assertaion:
        // assert!(!receipts.is_empty(), "No transaction receipts found");

        // once we fetch some transactions we need to verify their block number should match with our query block number
        // and each receipt's transaction hash value should match with the query block's transactoins field correspondingly in order
        for (i, receipt) in receipts.iter().enumerate() {
            assert_eq!(
                receipt.transaction_hash, block.transactions[i],
                "Mismatch in transaction hash"
            );
            assert_eq!(
                receipt.block_number, block.number,
                "Mismatch in block number"
            );
        }
    }
}
