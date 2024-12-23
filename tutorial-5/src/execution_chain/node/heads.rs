use async_tungstenite::{tokio as tungstenite, tungstenite::Message};
use chrono::{DateTime, Utc};
use futures::{channel::mpsc, SinkExt, Stream, StreamExt, TryStreamExt};
use serde::Deserialize;
use serde_json::json;

use super::{
    blocks::{BlockNumber, ExecutionNodeBlock},
    decoders::*,
    ExecutionNode,
};
use crate::{env::ENV_CONFIG, execution_chain::BlockRange};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Head {
    pub hash: String,
    #[serde(deserialize_with = "from_i32_hex_str")]
    pub number: BlockNumber,
    pub parent_hash: String,
    #[serde(deserialize_with = "from_unix_timestamp_hex_str")]
    pub timestamp: DateTime<Utc>,
}

// impl NewHeadMessage#into() -> Head
impl From<NewHeadMessage> for Head {
    fn from(message: NewHeadMessage) -> Self {
        message.params.result
    }
}
// impl ExecutionNodeBlock#into() -> Head
impl From<ExecutionNodeBlock> for Head {
    fn from(block: ExecutionNodeBlock) -> Self {
        Self {
            hash: block.hash,
            number: block.number,
            parent_hash: block.parent_hash,
            timestamp: block.timestamp,
        }
    }
}

#[derive(Deserialize)]
pub struct NewHeadParams {
    result: Head,
}

#[derive(Deserialize)]
pub struct NewHeadMessage {
    params: NewHeadParams,
}

enum HeadMessage {
    Subscribe,
    #[allow(dead_code)]
    Unsubscribe(String),
}

// impl HeadMessage#into() -> Message logic here
impl From<HeadMessage> for Message {
    fn from(message: HeadMessage) -> Self {
        match message {
            // if HeadMessage is Subscribe enum value, then go this logic
            HeadMessage::Subscribe => {
                let msg = json!({
                    "id":0,
                    "jsonrpc": "2.0",
                    "method": "eth_subscribe",
                    "params": ["newHeads"]
                });
                let message_text = serde_json::to_string(&msg).unwrap();
                Message::text(message_text)
            }
            // other wise it is UnSbuscribe enum value, then go this logic
            HeadMessage::Unsubscribe(id) => {
                let msg = json!({
                    "id":0,
                    "jsonrpc": "2.0",
                    "method": "eth_unsubscribe",
                    "params": [id]
                });
                let message_text = serde_json::to_string(&msg).unwrap();
                Message::text(message_text)
            }
        }
    }
}

#[derive(Deserialize)]
struct SubscriptionError {
    code: i32,
    message: String,
}

// deserializing successfully is all that matters
#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(untagged)]
enum SubscriptionResponse {
    SuccessMessage {
        id: i32,
        jsonrpc: String,
        result: String,
    },
    ErrorMessage {
        error: SubscriptionError,
        id: i32,
        jsonrpc: String,
    },
}

// method to subscribe newHead events from Ethereum
// and extract required fields from event body into local struct variable Head
pub fn stream_new_heads() -> impl Stream<Item = Head> {
    let (mut new_heads_tx, new_heads_rx) = mpsc::unbounded();

    tokio::spawn(async move {
        let url = ENV_CONFIG
            .geth_url
            .as_ref()
            .expect("GETH_URL is required in env to stream new heads")
            .to_string();

        let mut ws = tungstenite::connect_async(&url).await.unwrap().0;
        ws.send(HeadMessage::Subscribe.into()).await.unwrap();

        // here we expect a subscription confirmation message first.
        // then we loop on None that's the reason we use clippy::never_loop
        #[allow(clippy::never_loop)]
        while let Some(message) = ws.try_next().await.unwrap() {
            let message_text = message.to_text().unwrap();
            let message: SubscriptionResponse =
                serde_json::from_str(message_text).unwrap();

            match message {
                // receive subscribe success response handle logic:
                SubscriptionResponse::SuccessMessage { .. } => {
                    // we don't care about the success message inner content,
                    // write log , then break go comming loop to handle coming subscribed event messages
                    tracing::debug!("got subscription confirmation message");
                    break;
                }
                // receive subscribe fail response handle logic:
                SubscriptionResponse::ErrorMessage { error, .. } => {
                    // skip other fields only keep error add the error info to panic
                    panic!(
                        "subscription error, code {}, message: {}",
                        error.code, error.message
                    )
                }
            }
        } // loop for receiving subscription conformation or failure response

        // this loop is looping for receving subscribed & handle newHead events
        while let Some(message) = ws.try_next().await.unwrap() {
            // waiting for the next message to arrive can take many seconds, during this
            // waiting we may receive a ping message, just skip is ok
            if message.is_ping() {
                continue;
            }

            // here we manipulate the coming newHead events
            let message_text = message.to_text().unwrap();
            let new_head_message: NewHeadMessage =
                serde_json::from_str(message_text).unwrap();
            // convert new head message into Head
            let new_head = new_head_message.into();

            // here append the coming & converted newHead event into Head
            // and append it to real-time streaming receiver
            new_heads_tx.send(new_head).await.unwrap();
        }
    });

    // take the stream handler as reutrn value
    new_heads_rx
}

// add extra map operation to extract each Head#number as BlockNumber
fn stream_new_head_block_numbers() -> impl Stream<Item = BlockNumber> {
    stream_new_heads().map(|head| head.number)
}

fn stream_historic_block_numbers(
    block_range: BlockRange,
) -> impl Stream<Item = BlockNumber> {
    let (mut tx, rx) = futures::channel::mpsc::channel(10);
    todo!();
    rx
}

pub async fn stream_heads_from(
    gte_slot: BlockNumber,
) -> impl Stream<Item = BlockNumber> {
    let (mut tx, rx) = futures::channel::mpsc::channel(10);
    todo!();
    rx
}
