use std::io::StdoutLock;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "snake_case")]
enum Payload {
    Echo { echo: String },
}

struct EchoNode;

impl EchoNode {
    pub fn step(
        &mut self,
        input: Message,
        output: &mut serde_json::Serializer<StdoutLock>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    // block both console input and output streaming, let the input/output circuling in side the program
    // let the inner state machine only deal with the messages in and out and not deal with the i/o's input stream and output stream
    let stdin = std::io::stdin().lock();
    let stdout = std::io::stdout().lock();

    // we want to add a wraper surrounding the inputstream(stdin)
    // and converted the coming streaming into the types that statisfy the definition of the message
    // and then let the messages be accessed via the iterator
    // !! notice !! we also need to know iterator is a layze defintion
    // which means the de-serialize manipulation happens only when we execute a itearte opeartion upon the iterator
    let inputs = serde_json::Deserializer::from_reader(stdin)
        .into_iter::<Message>();

    // !! that's why we need to add an extra exception to resovle de-serialzie failuer case here !!
    for input in inputs {
        // here we add an anyhow::context function to resolve if the input data stream cannot be deserialized into the Message this situation
        let input = input.context("Maelstrom input from STDIN could not be deserialized")?;
    }

    Ok(())
}
