use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

pub struct EchoNode {
    pub id: usize,
}

impl EchoNode {
    pub fn step(
        // EchoNode's inner state may change according what kind of the message it receives
        &mut self,
        input: Message,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        // here we use enum pattern matching to decide
        // 1. how decode and handle different kind of payloads
        // 2. what to write as reply result via output this mimic channel(actually stdout stream, since we already lock it,
        //    no worry about other threads are allowed to write data to this channel)
        match input.body.payload {
            Payload::Init { .. } => {
                // create Message, and write it to output, and do not forget add a new line
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::InitOk,
                    },
                };

                // Serialize reply into json, and write it directly to output stream.
                // Here we dereference the mutable reference `&mut *output` to avoid transferring ownership.
                // `.context(xxx)` adds extra error information in the case serialziation or writing files fails.
                // This is cricual because we cannot use `println!` or `log` to debug errors, as `stdout` and `stdin`
                // are locked and reserved for protocol communicaiton.
                serde_json::to_writer(&mut *output, &reply)
                    .context("Serialize response to init")?;
                // append a new line character
                output
                    .write_all(b"\n")
                    .context("write trailing newline")?;
                self.id += 1;
            }

            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo },
                    },
                };

                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize respoinse to echo")?;
                output
                    .write_all(b"\n")
                    .context("write trailing newline")?;

                self.id += 1;
            }
            Payload::EchoOk { echo } => {}

            // throw exception, when receive init_ok
            Payload::InitOk { .. } => {
                bail!("Receive InitOk Message, this cannot be happen!!")
            }
            // do nothing, when receive echo_ok
            Payload::EchoOk { .. } => {}
        }

        Ok(())
    }
}
