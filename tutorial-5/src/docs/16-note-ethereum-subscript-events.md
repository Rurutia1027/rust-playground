# Ethereum Events Subscription and UnSubscription

In Ethereum, events such as `newHeads` are a way for applications to subscribe to blockchain data updates in real time, rather than repeatedly polling for the latest information. These events are provided via Ethereum's **JSON-RPC API**. Specificially, the `eth_subscribe` method allows a client to subscribe to various types of events(such as new blocks, pending transactions, etc.) and receive notifications when the events occur.

## What's the newHead Event ?

- The **newHeads** event in Ethereum is triggered whenever a new block(or head) is added to the blockchain.
- This event allows clients to listen for new block headers in real-time, including the block's hash, block number, timestamp, and parent hash value.

## How to Subscribe to newHead Event Using eth_subscribe

We can subscribe to newHeads using the `eth_subscribe` method. This method is part of the **JSON-RPC API** and is used to request the Ethereum node to send updates when certain events occur.

- Description: Triggered whenever a new block header is added to the chain.
- Use case: Used to track new blocks being mined and added to the blockchain.

```json
{
  "jsonrpc": "2.0",
  "method": "eth_subscribe",
  "params": ["newHeads"], // support mulitiple event's subscription,
  "id": 1
}
```

The server side will respond with a subscription ID once the subscription has been established. From this point on,
the server side will notify the subscriber of new blocks(headers) via WebSockets.
That make sense, and that's why we use WebSocket instead of RESTful API because response is a serios of real time events stream, and it cannot be handled via RESTful API.

## Other Events That Can Be Subscribed to with eth_subscribe

### `newHeads`

- Triggered whenever a new block header is added to the chain.

```json
{
  "jsonrpc": "2.0",
  "method": "eth_subscribe",
  "params": ["newHeads"],
  "id": 1
}
```

### `logs`

- Descirption: Used to listen for logs, such as event logs emitted by smart contracts. (from my piont, this logs can following with serious of handlers or filters to help users filter out the logs they do not care)
- Use case: Ideal for applicaitons that want to track events such as token transfers, contract interactions, etc.

```json
{
  "jsonrpc": "2.0",
  "method": "eth_subscribe",
  "params": [
    "logs",
    {
      "address": "0xYourContractAddressHere" # filter condition here, and this should support multiple condition fields,
                                             #only keep required smart contract address
    }
  ],
  "id": 2
}
```

### `pendingTransactions`

- Description: Notifies when a new transactions is addded to the pending transaction pool
- Use case: This is useful for applications that want to listen for transactions before they are mined into a block.

```json
{
  "jsonrpc": "2.0",
  "method": "eth_subscribe",
  "params": ["pendingTransactions"],
  "id": 3
}
```

### `newPendingTransactions`

- Description: Similar to pendingTransactions, but this one provides transaction data as it arrives in the pool, before being mined.
- Use case: Often used to monitor transactions before they are included in a block.

```json

```

### `syncing`

- Descirpiton: provides updates about the node's synchronization status with the Ethereum network.
- Use case: This is useful to track when a node is syncing to the network(either full or fast sync).

```json
{
  "jsonrpc": "2.0",
  "method": "eth_subscribe",
  "params": ["syncing"],
  "id": 5
}
```

### `mined`

- Description: This event is triggered when a transaction has been mined into a block.
- Use case: Typically used to track specific transactions and know when they've been confirmed.

```json
{
  "jsonrpc": "2.0",
  "method": "eth_subscribe",
  "params": ["mined", "0xYourTransactionHashHere"],
  "id": 6
}
```

## Unsubscribing from Events

Once a subscription is established, we can unsubscribe from events using the `eth_unsubscribe` method. This method allows us to stop receiving notificaitons for a specific event.

- **Method**: `eth_unsubscribe`
- **Params**: The subscription ID received from the `eth_subscribe` response.

```json
{
  "jsonrpc": "2.0",
  "method": "eth_unsubscribe",
  "params": ["our-received-subscription-id-here"],
  "id": 7
}
```

## Handling Subscription Messages

Once subscribed to an event, our application will start receiving messages from the Ethereum node whenever the event is triggered.

```json
{
  "jsonrpc": "2.0",
  "method": "eth_subscription",
  "params": {
    "subscription": "our-received-subscription-id-here", // record this, when we need to unsubscribe event we need re-use this as params
    "result": {
      "hash": "0x...",
      "number": "0x1b4",
      "parentHash": "0x...",
      "timestamp": "0x5d5e8bbf"
      // other block fields...
    }
  }
}
```

## Summary

- `eth_subscribe`: Used to subscribe to events like `newHeads`, `logs`, `pendingTransactions`, etc.

Those events help you stay updated with the Ethereum network in real time (coool, the real time event streams, by the way will the events of the same type like newHeads be modified into the iterator?) without having to repeatedly query the blockchain for the latest data. By using _WebSockets_ and subscribing to relevant events, our application can efficiently track important blockchain updates, such as new blocks, transactions, or contract events.
