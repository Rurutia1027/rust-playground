use anyhow::Context;
use dist::{main_loop, Body, Event, Message, Node};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::StdoutLock;
use std::time::Duration;

struct BroadcastNode {
    id: usize,
    node: String,

    // record values that we already 'know' (receive from other peer nodes)
    messages: HashSet<usize>,

    // key: node id
    // value: set of values that we(current BroadcastNode) know that peer BroadcastNode knows
    known: HashMap<String, HashSet<usize>>,

    // neighborhood ids vector
    neighborhood: Vec<String>,
    msg_communicated: HashMap<usize, HashSet<usize>>,
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
        messages: HashSet<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
    Gossip {
        seen: HashSet<usize>,
    },
}

enum InjectedPayload {
    Gossip,
}

impl Node<(), Payload, InjectedPayload> for BroadcastNode {
    fn from_init(
        _state: (),
        init: dist::Init,
        tx: std::sync::mpsc::Sender<Event<Payload, InjectedPayload>>,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let gossip_tx = tx.clone();

        // in fact, we don't even hold the injection in the scope of node to hold it all the time
        std::thread::spawn(move || {
            // generate gossip events
            // TODO: handle EOF signal

            loop {
                std::thread::sleep(Duration::from_millis(300));
                if let Err(_) =
                    tx.send(Event::Injected(InjectedPayload::Gossip))
                {
                    break;
                }
            }
        });

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
            msg_communicated: HashMap::new(),
        })
    }

    fn step(
        &mut self,
        input: Event<Payload, InjectedPayload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        match input {
            Event::EOF => {}

            Event::Injected(payload) => match payload {
                InjectedPayload::Gossip => {
                    for n in &self.neighborhood {
                        let known_to_n = &self.known[n];
                        let (already_known, mut notify_of): (
                            HashSet<_>,
                            HashSet<_>,
                        ) = self
                            .messages
                            .iter()
                            .copied()
                            .partition(|m| known_to_n.contains(m));

                        eprintln!(
                            "notify of {}/{}",
                            notify_of.len(),
                            self.messages.len()
                        );

                        let mut rng = rand::thread_rng();

                        notify_of.extend(already_known.iter().filter(|_| {
                            rng.gen_ratio(
                                10.min(already_known.len() as u32),
                                already_known.len() as u32,
                            )
                        }));

                        Message {
                            src: self.node.clone(),
                            dst: n.clone(),
                            body: Body {
                                id: None,
                                in_reply_to: None,
                                payload: Payload::Gossip { seen: notify_of },
                            },
                        }
                        .send(&mut *output)
                        .with_context(|| format!("gossip to {}", n))?;
                    }
                }
            },

            Event::Message(input) => {
                let mut reply = input.into_reply(Some(&mut self.id));
                match reply.body.payload {
                    Payload::Gossip { seen } => {
                        self.known
                            .get_mut(&reply.dst)
                            .expect("got gossip from unknown node")
                            .extend(seen.iter().copied());
                        self.messages.extend(seen);
                    }

                    Payload::Broadcast { message } => {
                        self.messages.insert(message);
                        reply.body.payload = Payload::BroadcastOk;
                        reply
                            .send(&mut *output)
                            .context("reply to broadcast")?;
                    }

                    Payload::Read => {
                        reply.body.payload = Payload::ReadOk {
                            messages: self.messages.clone(),
                        };
                        reply.send(&mut *output).context("reply to read")?;
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
                        reply
                            .send(&mut *output)
                            .context("reply to topology")?;
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
    main_loop::<_, BroadcastNode, _, _>(())
}
