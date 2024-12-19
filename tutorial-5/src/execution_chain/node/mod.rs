mod blocks;
mod decoders;
mod transaction_receipts;

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use anyhow::Result;
use async_tungstenite::{
    tokio::{connect_async, TokioAdapter},
    tungstenite::Message,
    WebSocketStream,
};

use futures::{channel::oneshot, stream::SplitStream};
use futures::{
    stream::{FuturesOrdered, StreamExt},
    SinkExt,
};

use serde::Deserialize;
use serde_json::{json, Value};
use thiserror::Error;
use tokio::{net::TcpStream, sync::mpsc};
