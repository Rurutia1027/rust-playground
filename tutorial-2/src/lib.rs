use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::StdoutLock;

pub mod broadcast;
pub mod grow_only_counter;
pub mod kafka_style_log;
pub mod totally_available;

pub trait Node<Payload> {
    fn step(
        &mut self,
        input: Message<Payload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<Payload> {
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: Body<Payload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<Payload> {
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct Init {
    node_id: String,
    node_ids: Vec<String>,
}

// here define main_loop
pub fn main_loop<S, Payload>(mut state: S) -> anyhow::Result<()>
where
    S: Node<Payload>,
    Payload: DeserializeOwned,
{
    // Handlers for the system's standard input and output streams.
    // The `lock` ensures thread safety by preventing other threads
    // from accessing the input/output stream buffers at the same time
    let stdin = std::io::stdin().lock();

    // We need to write serialized json message and new line character to stdout, so let it be mutable
    let mut stdout = std::io::stdout().lock();

    // Receive input stream as iterators of Message
    let inputs = serde_json::Deserializer::from_reader(stdin)
        .into_iter::<Message<Payload>>();

    for input in inputs {
        let input = input
            .context("Maelstrom input form STDIN could not be deserialized")?;
        state
            .step(input, &mut stdout)
            .context("EchoNode step function failed")?;
    }

    Ok(())
}
