use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    io::{BufRead, StdoutLock, Write},
    os::unix::thread,
};

pub trait Node<S, Payload> {
    fn from_init(s: S, init: Init) -> anyhow::Result<Self>
    where
        Self: Sized;

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
pub enum InitPayload {
    Init(Init),
    InitOk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

impl<Payload> Message<Payload> {
    pub fn into_reply(self, id: Option<&mut usize>) -> Self {
        Self {
            src: self.dst,
            dst: self.src,
            body: Body {
                id: id.map(|id| {
                    let mid = *id;
                    *id += 1;
                    mid
                }),
                in_reply_to: self.body.id,
                payload: self.body.payload,
            },
        }
    }
}

// here define main_loop
pub fn main_loop<S, N, P>(init_state: S) -> anyhow::Result<()>
where
    P: DeserializeOwned + Send + 'static,
    N: Node<S, P>,
{
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    let mut stdin = stdin.lines();

    let init_msg: Message<InitPayload> = serde_json::from_str(
        &stdin
            .next()
            .expect("no init message received")
            .context("failed to read init message from stdin")?,
    )
    .context("init message could not be deserialized")?;

    let InitPayload::Init(init) = init_msg.body.payload else {
        panic!("first message should be init");
    };

    let mut node: N = Node::from_init(init_state, init)
        .context("node initialization failed")?;

    let reply = Message {
        src: init_msg.dst,
        dst: init_msg.src,
        body: Body {
            id: Some(0),
            in_reply_to: init_msg.body.id,
            payload: InitPayload::InitOk,
        },
    };

    serde_json::to_writer(&mut stdout, &reply)
        .context("serialize response to init")?;
    stdout.write_all(b"\n");

    // --- thread && channel ---
    drop(stdin);

    let (tx, rx) = std::sync::mpsc::channel();
    let stdin_tx = tx.clone();

    let jh = std::thread::spawn(move || {
        let stdin = std::io::stdin().lock();
        for line in stdin.lines() {
            let line =
                line.context("Maelstrom input form STDIN could not be read")?;
            let input: Message<P> = serde_json::from_str(&line).context(
                "Maelstrom input from STDIN could not be deserialized",
            )?;

            if let Err(_) = stdin_tx.send(input) {
                return Ok::<_, anyhow::Error>(());
            }
        }

        Ok(())
    });

    // loop input items in receiver channel
    for input in rx {
        node.step(input, &mut stdout)
            .context("Node step function failed")?;
    }
    jh.join()
        .expect("stdin thread panicked")
        .context("stdin thread err")?;

    Ok(())
}
