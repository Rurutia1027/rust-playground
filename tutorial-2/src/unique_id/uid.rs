use anyhow::Context;
use dist::{main_loop, Body, Message, Node};
use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};

struct UniqueNode {
    id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Payload {
    Generate {},
    GenerateOk {
        id: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

impl Node<Payload> for UniqueNode {
    fn step(
        &mut self,
        input: Message<Payload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Init { .. } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::GenerateOk { id: "".to_string() },
                    },
                };
                serde_json::to_writer(&mut *output, &reply).context("err")?;
                output.write_all(b"\n").context("")?;
            }
            Payload::InitOk { .. } => {}
            Payload::Generate {} => {}
            Payload::GenerateOk { .. } => {}
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop(UniqueNode { id: 0 })
}
