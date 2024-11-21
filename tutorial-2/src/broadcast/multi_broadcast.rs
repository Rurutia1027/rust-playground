use anyhow::Context;
use dist::{main_loop, Event, Message, Node};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::{StdoutLock, Write};

struct BroadcastNode {
    id: usize,
    node: String,

    // record values that we already 'know' (receive from other peer nodes)
    messages: HashSet<String>,

    // key: node id
    // value: set of values that we(current BroadcastNode) know that peer BroadcastNode knows
    known: HashMap<String, HashSet<String>>,

    // neighborhood ids vector
    neighborhood: Vec<String>,

    // injected a series of event handlers
    inject: std::sync::mpsc::Sender<Event<Payload>>,

    //
    msg_communicated: HashMap<usize, HashSet<String>>,
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
        messages: HashSet<String>,
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
        tx: std::sync::mpsc::Sender<Event<Payload>>,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            id: 1,
            node: init.node_id,
            messages: HashSet::new(),
            known: init
                .node_ids
                .into_iter()
                .map(|nid| (nid, HashSet::new()))
                .collect(),
            neighborhood: Vec::new(),
            inject: tx,
            msg_communicated: HashMap::new(),
        })
    }

    fn step(
        &mut self,
        input: Event<Payload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        match input {
            Event::EOF => {}

            Event::Injected(payload) => {
                std::thread::spawn(move || {
                    
                });
            }

            Event::Message(input) => {
                let mut reply = input.into_reply(Some(&mut self.id));
                match reply.body.payload {
                    Payload::Broadcast { message } => {
                        self.messages.insert(message.to_string());
                        reply.body.payload = Payload::BroadcastOk;
                        serde_json::to_writer(&mut *output, &reply)
                            .context("")?;
                        output
                            .write_all(b"\n")
                            .context("write trailing newline")?;
                    }

                    Payload::Read => {
                        reply.body.payload = Payload::ReadOk {
                            messages: self.messages.clone(),
                        };
                        serde_json::to_writer(&mut *output, &reply)
                            .context("")?;
                        output
                            .write_all(b"\n")
                            .context("write trailing newline")?;
                    }
                    Payload::Topology { mut topology } => {
                        self.neighborhood =
                            topology.remove(&self.node).unwrap_or_else(|| {
                                panic!(
                                    "no topology given for node {}",
                                    self.node
                                )
                            });

                        reply.body.payload = Payload::TopologyOk;
                        serde_json::to_writer(&mut *output, &reply)
                            .context("")?;
                        output
                            .write_all(b"\n")
                            .context("write trailing newline")?;
                    }

                    Payload::BroadcastOk
                    | Payload::ReadOk { .. }
                    | Payload::TopologyOk => {}
                }
            }
        }

        // let mut reply = input.into_reply(Some(&mut self.id));

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<_, BroadcastNode, _>(())
}
