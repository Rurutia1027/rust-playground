# Event Drive in Rust

After reviewing the implementation of the [call](../execution_chain/mod.rs) function, I realized that the [connect](../execution_chain/mod.rs) function is executed only once. Once the WebSocket connection is successfully established, a channel is returned to the caller. Other methods like `get_block_number` are all built on top of the call method. The call function retrieves results via the `message_rx_mapping` using rx.

The connect function initialize two `tokio::spaw` tasks to handle the WebSocket connection:

**First thread(Single-threaded)**
This thread uses the `handle_messages` function to process incoming WebSocket messages. It loops through the message stream and processes each response based on its ID by looking it up in a thread-safe shared HashMap(message_rx_map).

**Second thread(Multi-threaded)**
This thread union is responsible for sending messages. The `message_tx` sender uses a multi-threaded channel to enqueue outgoing message(out going to Ethereum via local established GETH).

Although the connection function does not map these thread pools to global variables, the spawned tasks presist throughout the program's lifetime. These threads remain active until the program terminates.

When a method like get_block_by_number (hypotentical example) calls the `call` function, the following sequence occurs:

- **Prepare JSON-RPC Request**
  The call method constructs the JSON body containing the method name, parameters, and a unique ID(retrieved from local id_pool).
  This body is serialized into a string for JSON-RPC transmission via Geth through WebSocket connection.

- **Register Message Handler**
  The `call` function creates a onshot channel(with a tx sender and rx receiver) and registers the tx sender in the message_rx_map hash map using the unique ID as the key.

- **Send the JSON-RPC Request**
  The serialized JSON message is sent through the message_tx sender, which belongs to the second thread pool(multi-threaded, responsible for sending messages). it does not block the first thread pool.

- **Await Response**
  The first thead pool(via handle_message) processes incoming WebSocket messages. When a response message arrvies:

* The unique ID is extracted.
* The correspoinding tx sender is located in the message_rx_map.
* The result (success or error) is sent back through the channel.

- **Return Result to Caller**
  The rx receiver in the call method awaits the result (`rx.await.unwraup()`). Once the result is avaiable, it is returned to the caller as a `Result<serde_json::Value, RpcError>`.

## Asynchronous Behavior

Even though the call function and WebSocket message handler (handle_messages) are logically sequential in the code, they operate asynchronously. This ensures the following:

- The second thread pool(message (to Ethereum) sender) sends the JSON-RPC request without blocking.
- The first rhead pool(message handler) continuously processes WebSocket events independently.
