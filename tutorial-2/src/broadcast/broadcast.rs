use anyhow::Context;
use dist::{main_loop, Event, Node};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{StdoutLock, Write};

struct BroadcastNode {
    id: usize,
    node: String,
    messages: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Payload {
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

impl Node<(), Payload> for BroadcastNode {
    fn from_init(
        _state: (),
        init: dist::Init,
        inject: std::sync::mpsc::Sender<Event<Payload>>,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            id: 1,
            node: init.node_id,
            messages: Vec::new(),
        })
    }

    fn step(
        &mut self,
        input: Event<Payload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        let Event::Message(input) = input else {
            panic!("got injected event when there's event injection");
        };
        // let mut reply = input.into_reply(Some(&mut self.id));
        let mut reply = input.into_reply(Some(&mut self.id));
        match reply.body.payload {
            Payload::Broadcast { message } => {
                self.messages.push(message);
                reply.body.payload = Payload::BroadcastOk;
                serde_json::to_writer(&mut *output, &reply).context("")?;
                output.write_all(b"\n").context("write trailing newline")?;
            }

            Payload::Read => {
                reply.body.payload = Payload::ReadOk {
                    messages: self.messages.clone(),
                };
                serde_json::to_writer(&mut *output, &reply).context("")?;
                output.write_all(b"\n").context("write trailing newline")?;
            }
            Payload::Topology { topology: _ } => {
                reply.body.payload = Payload::TopologyOk;
                serde_json::to_writer(&mut *output, &reply).context("")?;
                output.write_all(b"\n").context("write trailing newline")?;
            }

            Payload::BroadcastOk
            | Payload::ReadOk { .. }
            | Payload::TopologyOk => {}
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<_, BroadcastNode, _, _>(())
}
