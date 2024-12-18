# Notes for Geth

## What is Geth?

Geth (short for **"Geth Ethereum"**) is the official implementation of the Ethereum protocol written in **Go**. It is a command-line interface(CLI) tool that acts as a **full Ethereum node**, allowing you to interact with the Ethereum blockchain.
Geth is similar as **Spark Client** which interact with remote server/cluster through **RPC** protocol. Once set up and connected to the Ethereum node, we can fetch various kinds of datasets using **JSON-RPC** services.

#### Geth provides features such as:

- Running a full Ethereum node (syncing the blockchain).
- Interacting with **smart contracts**.
- Mining (in **PoW** environments or private chains).
- Fetching on-chain data such as transactions, blocks, and balances.
- Sending transactions and managing wallets/accounts.
- Hosting JSON-RPC API endpoints to enable applications to interact with the Ethereum blockchain.

---

## Installing & Configuring Geth on macOS

```bash
brew tap ethereum/ethereum
brew install ethereum
```

### Setup Geth with WebSocket Enabled

```shell
geth --http --http.api "eth,web3,net" \
     --ws --ws.api "eth,web3,net" \
     --ws.addr 0.0.0.0 \
     --ws.port 8546 \
     --syncmode "snap"
```

Explanation of Flags:

- `--ws`: Enables the WebSocket server.
- `--ws.api "eth,web3,net"`: Specifies the APIs we want to expose over WebSocket(eth for Ethereum RPC, web3 for utility functions, and net for network information).
- `--ws.addr 0.0.0.0`: Sets the WebSocket server to listen on all available network interfaces.
- `--ws.port 8546`: Specifies the port for the WebSocket server (default is 8546).
- `--syncmode "snap"`: Syncs the node using the "snap" mode(this is faster than "full" mode).

Once the above command is run successfully, Geth will expose a WebSocket on `ws://localhost:8546`.

### How it works or What happend when we type the shell command?

```shell
geth --http --http.api "eth,web3,net" \
     --ws --ws.api "eth,web3,net" \
     --ws.addr 0.0.0.0 \
     --ws.port 8546 \
     --syncmode "snap"
```

- **1. Geth is my local Ethereum Node**: Our local Geth client syncs with the Ethereum network and serves as a bridge between our application(e.g., Rust Codes, or local CURL JSON Requests) and the blockchain.

- **2. No middleman required**: Since we're running our own node, we do not rely on third-party services like **Infura** or **Alchemy**.

- **3. Local Successful Setup Geth Exposed HTTP/WebSocket Endpoints Are**:

```text
HTTP: http://127.0.0.1:8545
WebSocket: ws://127.0.0.1:8546
```

### Does the Setup Geth Connect to a Remote RPC Server ?

Actually, Nope... <br/>

**Our Setup Geth client does not connect to a remote RPC server.** Instead:

- It connects directly to **Ethereum peer nodes on the network** to sync the blockchain data.
- Our Geth node is **self-sufficient** once synced--it fetches data from the Ethereum network itself, not from any third-party services.

### Does It Listen for Local Requests?

**Yes, it listens for local requests** on:

```text
HTTP: http://127.0.0.1:8545 (default)
WebSocket: ws://0.0.0.0:8546
```

### Work Flow of Geth WebSocket with Rust WebSocket Fetch Ethereum Dataset

#### Geth syncs with the Ethereum blockchain

- Geth connects to peer nodes which are located in Ethereum network and downloads blockchain data to local database.
- Geth acts as a local Ethereum node(parallel with other nodes locates in Ethereum network).

#### Our Application(Rust Codes) connects to Geth via WebSocket

- Our application(Rust, Python, Javascript, etc.) send requests to the Geth client over HTTP or WebSockdet.

#### Geth Process the Request

- Geth fetches the requested data from its local blockchain database.
- If the data isn't available (e.g., the node isn't fully synced), Geth fetches it from Ethereum peers and caches it locally.

#### Key Points

- Our local setup Geth node is a **full Ethereum client** that syncs blockchain data independently from other services.
- It provides **RPC endpoints(HTTP/WebSocket)** so that our application can interact with the blockchain without requring third-party service like **Infura** or **Alchemy**.
- There is no "remote RPC server" involved (that make sense ... Ethereum is de-centrailized system, peer-to-peer rather than client/server, client/cluster such kind of centrialized architecutre ... ) -- Geth itself is the server node and our Rust codes are clients -- they(local Geth(ethereum node) and rust applications) are CS architecure.

---

## Connect Using Rust WebSocket Client

Since we already setup our Geth, we can test whether it's exposed Endpoints of HTTP and WebSocket with Curl commands and Rust WebSocket codes.

```shell
 % geth --http --http.api "eth,web3,net" \
     --ws --ws.api "eth,web3,net" \
     --ws.addr 0.0.0.0 \
     --ws.port 8546 \
     --syncmode "snap"
INFO [12-18|17:39:08.194] Starting Geth on Ethereum mainnet...
INFO [12-18|17:39:08.194] Bumping default cache on mainnet         provided=1024 updated=4096
INFO [12-18|17:39:08.196] Maximum peer count                       ETH=50 total=50
INFO [12-18|17:39:08.201] Set global gas cap                       cap=50,000,000
INFO [12-18|17:39:08.202] Initializing the KZG library             backend=gokzg
INFO [12-18|17:39:08.242] Allocated trie memory caches             clean=614.00MiB dirty=1024.00MiB
...
```

### Curl Commands

```shell
% curl -X POST http://127.0.0.1:8545 \
-H "Content-Type: application/json" \
-d '{
    "jsonrpc": "2.0",
    "method": "web3_clientVersion",
    "params": [],
    "id": 1
}'

% {"jsonrpc":"2.0","id":1,"result":"Geth/v1.14.12-stable/darwin-amd64/go1.23.3"}
```

### Rust Codes

```toml
// Cargo.toml
...

[dependencies]
ethers = { version = "2.0", features = ["ws"] }
```

```rust
use ethers::providers::{Provider, Ws};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error:Error>> {
    // connect to our local Geth WebSocket endpoint
    let ws = Ws::connect("ws://127.0.0.1:8546").await?;
    let provider = Provider::new(ws);

    // Retrieve the latest block number
    let block_number = provider.get_block_number().await?;
    println!("Latest block number: {}", block_number);

    Ok(())
}
```

---

## Ethereum Official On-Chain Data Model

### [Blocks](https://ethereum.org/en/developers/docs/blocks/)

Blocks are fundamental units of the Ethereum blockchain, containing a set of transactions and other metadata. Each block includes:

- **Block Number**: The position of the block in the blockchain.
- **Timestamp**: When the block was mined.
- **Transactions**: List of transactions included in the block.
- **Parent Hash**: Reference to the previous block. **Genesis Block** is the first block in the blockchain, and it **does not have a parent**. Therefore, its parentHash is set to a value of `0x0000...0000(a 32-byte zero value)`. **Coinbase(Miner Address)** refers to the address of the miner who created(or mined) the block. This is unrelated to the parentHash field. In the case of the genesis block, the coinbase is usually set to a default or placeholder value since there was no mining involved in its creation.

We can fetch block data via methods like [eth_getBlockByNumber](https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getblockbynumber) or [eth_getBlockByHash](https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getblockbyhash).

### [Transactions](https://ethereum.org/en/developers/docs/transactions/)

Transactions are instructions sent from one account to another on the Ethereum network. Each transaction includes:

- **Nonce**: Number of transactions sent from the sender's address.
- **Gas Price**: Amount of Ether the sender is willing to pay per unit of gas.
- **Gas Limit**: Maximum amount of gas units the transaction can consume.
- **To**: Recipient address(or contract creation).
- **Value**: Amount of Ether to transfer.
- **Data**: Input data for contract execution or message.

We can fetch transaction dataset via method [eth_getTransactionByHash](https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_gettransactionbyhash).

### [Accounts](https://ethereum.org/en/developers/docs/accounts/)

Ethereum accounts are entities with an Ether balance that can send transactions. THere are two types:

- **Exactly Owned Accounts(EOAs)**: Controlled by private keys; used by users.
- **Contract Accounts**: Controlled by contract code; associated with smart contracts.

Account data includes the address, balance, nonce, and storage. We can fetch account datasets via method [eth_getBalance](https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getbalance) and [eth_getTransactionCount](https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_gettransactioncount).

### Logs and Events

Logs are records produced by contract execution, often used to capture events. Each log contains:

- **Log Index**: Position in the block's list of logs.
- **Data**: Arbitrary length data field.
- **Topics**: Indexed parameters for filtering(e.g., event signatures.)

Log & Events datasets can be retrieved via [eth_getLogs](https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getlogs) based on filter address, topics, or block range parameters.

### [Smart Contracts](https://ethereum.org/en/developers/docs/smart-contracts/)

Smart contracts are self-executing code deployed on the Ethereum blockchain (a bit similar to the UDF(user-define-function) in the hive: implement the UDF codes obey interface requirements, commit codes to remote server side, and send request to trigger the UDF to process the datasets). They have associated bytecode and storage. We can interact with contracts using:

- **Contract Address**: Unique address where the contract is published and deployed.
- **ABI(Application Binary Interface)**: Defines how to encode/decode data to interact with the contract's functions and events.

### [Uncles(Ommer Blocks)](https://ethereum.org/en/developers/docs/consensus-mechanisms/pow/mining/#ommer-blocks)

Uncles are blocks that were mined but not included in the main blockchain(wait ... is this the coinbase? ), often due to network latency. They are referenced by later blocks and receive partial rewards. Uncle data includes:

- **Uncle Hash**: Hash of the uncle block.
- **Miner**: Address of the miner who mined the uncle.

#### _is_uncle_block_the_coinbase_ ?

No, the **uncle block** is not the same as the **coinbase block**.

#### **Uncle Block**

- A unicle block(or ommer block) is a valid block that was mined but did not become part of the main blockchain. This happens because another competing block was added to the chain first.
- Although uncle blocks are not part of the main chain, Ethereum includes them to reward miners for their work and improve blockchain connections t the network.

**Key Points About Uncle Blocks**:

- They are **not part of the main chain**, but they reference the canonical blockchain.
- Miners of uncle blocks receive a reduce block reward compared to miners of canonical blocks.
- Uncle blocks are included in the block header of a canonical block via the uncles field.

#### **Coinbase Block**

- The **coinbase block**(or miner block) refers to the block where the miner who successfully mined the block is rewarded.
- The term "coinbase" specifically refers to the miner's reward transaction (also called the "coinbase transactions"), which is the first transaction in a block. This transaction creates new Ether as a mining reward and pays it to the miner's address.

**Key Points about the coinbase block**:

- It is always the block that is part of the main chain.
- The miner who mined the block receives the coinbase reward.
- The miner's address is specified in the block header's coinbase field.

#### Conclusion

An **uncle block** is a valid block excluded fro the main chain, while the **coinbase block** is the canonical block where the miner earns the full reward.

Fetch data via method [eth_getUncleByBlockNumberAndIndex](https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getunclebyblocknumberandindex) to retrieve uncle block information.

### [Receipts](https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_gettransactionreceipt)

Transaction receipts provide the outcome of transaction execution, including:

- **Status**: Success or failure of the transaction.
- **Cumulative Gas Used**: Total gas used in the block up to this transaction.
- **Logs**: Array of log objects generated by the transaction.

Fetch datasets of receipts via method [eth_getTransactionReceipt](https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_gettransactionreceipt).

### References

- [Ethereum Data Model Offical Document](https://ethereum.org/en/developers/docs/)
- [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf)

---
