use async_tungstenite::tungstenite::Message;
use chrono::{DateTime, Utc};

use serde::Deserialize;
use serde_json::json;

use super::{
    blocks::{BlockNumber, ExecutionNodeBlock},
    decoders::*,
    ExecutionNode,
};

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
