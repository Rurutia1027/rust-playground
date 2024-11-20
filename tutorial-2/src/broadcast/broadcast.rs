use anyhow::Context;
use dist::{main_loop, Body, Message, Node};
use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};

struct BroadcastNode {
    id: usize,
    node: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Payload {
    Broadcast,
    Read,
    Topology,
}

impl Node<(), Payload> for BroadcastNode {
    fn from_init(_state: (), init: dist::Init) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
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
            Payload::Broadcast { .. } => {}
            Payload::Read { .. } => {}
            Payload::Topology { .. } => {}
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<_, BroadcastNode, _>(())
}
