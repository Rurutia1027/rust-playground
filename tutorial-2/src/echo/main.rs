use anyhow::{Context, Result};
use dist::echo::{EchoNode, Message};

fn main() -> anyhow::Result<()> {
    // Handlers for the system's standard input and output streams.
    // The `lock` ensures thread safety by preventing other threads
    // from accessing the input/output stream buffers at the same time
    let stdin = std::io::stdin().lock();

    // We need to write serialized json message and new line character to stdout, so let it be mutable
    let mut stdout = std::io::stdout().lock();

    // Receive input stream as iterators of Message
    let inputs = serde_json::Deserializer::from_reader(stdin)
        .into_iter::<Message>();

    // instance of EchoNode
    let mut state = EchoNode { id: 0 };

    for input in inputs {
        let input = input.context("Maelstrom input form STDIN could not be deserialized")?;
        state
            .step(input, &mut stdout)
            .context("EchoNode step function failed")?;
    }

    Ok(())
}
