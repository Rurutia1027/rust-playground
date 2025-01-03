use anyhow::{bail, Context};
use dist::{main_loop, Body, Event, Message, Node};
use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};

pub struct EchoNode {
    pub id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Payload {
    Echo { echo: String },
    EchoOk { echo: String },
}

impl Node<(), Payload> for EchoNode {
    fn from_init(
        _s: (),
        _init: dist::Init,
        _tx: std::sync::mpsc::Sender<Event<Payload>>,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(EchoNode { id: 1 })
    }

    fn step(
        &mut self,
        input: Event<Payload>,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        let Event::Message(input) = input else {
            panic!("got injected event when there's no event injection");
        };

        let mut reply = input.clone().into_reply(Some(&mut self.id));
        match reply.body.payload {
            Payload::Echo { echo } => {
                reply.body.payload = Payload::EchoOk { echo };
                serde_json::to_writer(&mut *output, &reply)
                    .context("serialize respoinse to echo")?;
                output.write_all(b"\n").context("write trailing newline")?;

                self.id += 1;
            }
            Payload::EchoOk { .. } => {}
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<_, EchoNode, _, ()>(())
}
