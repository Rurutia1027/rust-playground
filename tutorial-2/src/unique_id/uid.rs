use anyhow::Context;
use dist::{main_loop, Body, Message, Node};
use serde::{Deserialize, Serialize};
use std::{
    io::{StdoutLock, Write},
    thread::sleep,
};

struct UniqueNode {
    id: usize,
    node: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Payload {
    Generate {},
    GenerateOk {
        #[serde(rename = "id")]
        guid: String,
    },
}

impl Node<(), Payload> for UniqueNode {
    fn from_init(_state: (), init: dist::Init) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(UniqueNode {
            id: 1,
            node: init.node_id,
        })
    }

    fn step(
        &mut self,
        input: Message<Payload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Generate {} => {
                // global unique id which generation based on current node's id
                let guid = format!("{}-{}", self.node, self.id);

                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::GenerateOk { guid },
                    },
                };

                serde_json::to_writer(&mut *output, &reply)?;
                output.write_all(b"\n").context("")?;

                self.id += 1;
            }

            Payload::GenerateOk { .. } => {}
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<_, UniqueNode, _>(())
}
