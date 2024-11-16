// use std::io::StdoutLock;
// use serde::{Serialize, Deserialize};
// use serde_json::{Result, StreamDeserializer};
// use anyhow::{bail, Context};
// use serde_json::Deserializer;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct Message {
//     src: String,
//     #[serde(rename = "dest")]
//     dst: String,
//     body: Body,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct Body {
//     #[serde(rename = "msg_id")]
//     id: Option<usize>,
//     in_reply_to: Option<String>,
//     #[serde(flatten)]
//     payload: Payload,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type")]
// #[serde(rename_all = "snake_case")]
// enum Payload {
//     Echo { echo: String },
//     EchoOK { echo: String },
// }

// struct EchoNode {
//     id: usize,
// }

// impl EchoNode {
//     pub fn step(&mut self,
//                 input: Message,
//                 output: &mut serde_json::Serializer<StdoutLock>,
//     ) -> anyhow::Result<()> {
//         match input.body.payload {
//             Payload::Echo { echo } => {
//                 let reply = Message {
//                     src: input.dst,
//                     dst: input.src,
//                     body: Body {
//                         id: Option::from(self.id),
//                         in_reply_to: Option::from(input.body.id),
//                         payload: Payload::EchoOK {
//                             echo
//                         },
//                     },
//                 };
//                 reply
//                     .serialize(output)
//                     .context("serialization failed")?;
//                 self.id += 1;
//             }
//             Payload::EchoOK { echo } => {}
//         }
//         Ok(())
//     }
// }

// fn main() -> anyhow::Result<()> {
//     let stdin = std::io::stdin().lock();
//     let inputs: StreamDeserializer<_, Message> = serde_json::Deserializer::from_reader(stdin).into_iter();

//     let stdout = std::io::stdout().lock();
//     let mut output = serde_json::Serializer::new(stdout);

//     let mut state = EchoNode {id: 0};

//     for input in inputs {
//         let input = input.context("Maelstrom input from STDIN could not be deserialized")?;
//         state.step(input, &mut output).context("Node step function failed")?;
//     }

//     Ok(())
// }
