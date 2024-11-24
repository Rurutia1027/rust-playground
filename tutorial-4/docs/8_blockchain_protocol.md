# Blockchain Protocols & Platforms & Configure Different Blockchain Platforms in GraphNode 

In this note, we mainly focus on configuring and using GraphNode with different blockchain protocols, and different blockchain protocol features and their platforms that store the 'real-world' blockchain datasets we can fetch and analyze. 

I’ve been pondering the relationship between blockchain protocols, protocol providers (platforms), and the blockchain itself. Can I consider protocols as defining distinct sets of actions or specifications for blockchains, with each protocol focusing on different features? Platforms then implement these protocols, but the underlying blockchain data remains consistent across platforms adhering to the same protocol.

* Protocols: Represent the rules and standards for how blockchain network operate(e.g., Ethereum, NEAR, Polkadopt). Each protocol defines how blocks are added, transaction are processed, and smart contracts are executed. These are the core blockchain architecures. 
* Platforms: (e.g., Infura, Geth): Those act as **implementation or access points** for interacting with these protocols. Platforms implement the protcols' standard to provide services such as querying blockchain data, broadcasting transactions, or running full nodes. 

But, all in all, the **blockchain underlying data** remains consistent across all platforms interacting with the same protocol and blockchain network. For example, if you query Ethereum's mainnet using **Infura** or **Geth**, the data retrieved(e.g., transactions, smart contract events) will be the same because they access the same decentralized Ethereum blockchain. 

**Practical View**
* Protocols = RUles(Blockchain Standards)
* Platforms = Tools (Protocol Implementations)

--- 
## Configuring GraphNode to Retrieve Ethereum Data 

### Step-1: Connect to an Ethereum Node: 
To synchronize with Ethereum, GraphNode needs access to an Ethereum node. This can be achived through: 
* Local Node: Run a local Ethereum node using Geth or OpenEthereum. 
* Hosted Services: Use Infura, Alchemy, or QuickNode for remote access. 
* Update the GraphNode configuraiton to point to your Ethereum node's HTTP or WebSocket RPC endpoint. 
  
```yaml 
# docker-compose.yml 
graph-node: 
  environment: 
    ethereum: 'mainnet:http://locoalhost:8545'
```

### Step-2: Configure IPFS 
IPFS is used to store subgraph metadata. Ensure you have an IPFS instance running, and incldue its URL in the configuraiton: 
```yaml 
ipfs:
  environment: 
    ipfs: 'http://localhost:5001'
```

### Step-3: Adding a Subgraph Manifest 
#### 3.1 Define the Subgraph:
  
Create a manifest(`subgraph.yml`) to describe your subgraph. This file includes 

```yaml 
# Smart contract addresses
# Events to listen for 
# Data entities and their relationships 

specVesion: 0.0.2 
description: A subgraph for tracking token transfers. 
schema: 
  file: ./schema.graphql 
dataSource:
  - kind: ethereum/contract
    name: Token 
    network: mainnet 
    source: 
      address: "0x..."  
      abi: Token 
      startBlock: 123456213
    mapping:
      kind: ethereum/events 
      apiVerison: 0.0.4
      language: wasm/assemblyscript
      entitties:
        - Transfer 
      eventHandlers:
        - event: Transfer(indexed address, indexed address, uint256)
          handler: handlerTransfer
      file: ./src/mapping.ts 
```

#### 3.2 Deploy the Subgraph(retrive data from remote/local blockchain, datasets will be filtered and converted into the format we declare in subgraph.yml, and stored to the IPFS ): 
* use graph-cli to deploy the subgraph to the GraphNode. 
```shell 
graph deploy \
   # this is the remote/lcoal blockchain's node address or actually the exposed endpoint(API)
   --node http://localhost:8020 \ 
   
   # API exposed by IPFS which stores the subgraph queried from the blockchain dataset. 
   --ipfs http://localhost:5001  
   <SUBGRAPH_NAME>
```

#### 3.3 Querying Data Using GraphQL 

```graphql 
query {
    transfers(first: 10) {
        id
        from
        to
        value
    }
}
```

#### 3.4 Integrate dApps with GraphQL 
In this steps, we can integrate the GraphNode queried and stored 'on-chain' data with the 'off-chain' dApps, by let the dApp query dataset by GraphQL query langauge via the GraphQL client. 


#### 3.5 Define and Implement Handlers 
We first need to declare the entities in the `subgraph.yml` to tell the GraphNode which entities we need to retrieve, and which need to filtred from the blockchain 'raw data items'. We also declare a series of handlers that need to manipulate upon the items we retrieved from the blockchain 'raw data'. Like this declaration: 

```yaml 
...
      entitties:
        # here we define the entity name
        - Transfer 
      eventHandlers:
        # declare we monitor the blockchain's Transfer this 'raw data object', and only focus 3 parameters in the object
        # that is the source indexed address, indexed destination address, and a extra data item
        # here we define the inner fields of the previous declared entities 
        - event: Transfer(indexed address, indexed address to, uint256, ts timestamp)

          # here we declare the name of the function that play the role of the handler that manipulate the 
          # Transfer this object, do filter, convert follow the business requirements
          handler: handlerTransfer   
      # this we declare the handler is declared and locates in which sources file, here it is under the src path and implemented in typescrpt 
      file: ./src/mapping.ts # this is the handler'
```

* Implementaiton of Handlers in `mapping.ts` 
```typescript 
import {Transfer} from "../generated/schema"; 
import {Transfer as TransferEvent} from "../generated/Token/Token"; 

// This is similar as the netty's inner handler manipulation 
// since, we already declare the entities and it's inner fields 
// this function handlerTransfer will extract TransferEvent multiple parameter's {from, to, and inner vlaue}
// to the Transfer entity 
export function handleTransfer(event: TransferEvent): void {
     // Create a new entity using the transaction hash as an ID
    let transfer = new Transfer(event.transaction.hash.toHex());
    transfer.from = event.params.from;
    transfer.to = event.params.to;
    transfer.value = event.params.value;
    transfer.timestamp = event.block.timestamp;
    transfer.save();
}
```

* Bind Event Data to Entities
* Compilation of the Mappings:
  - Ensure the `mapping.ts` file is valid and contains all handlers referenced in subgraph.yml 
  - Use graph-cli to compile the mappings into WebAssembly(Wasm). 
  ```shell 
  graph codegen && graph build 
  ```

--- 

## Ethereum(e.g., Geth, Infura, Alchemy) in GraphNode 
### Features 
* Smart Contract Execution: Ethereum pioneered the concept of programmable smart contracts. 
* Proof of Stake (Post Merge): Ethereum now operates on PoS, reducing energy usage. 
* Token Standards: Supports **ERC-20** (fungible tokens) and **ERC-721** (NFTs).
* EVM compatibility: Ethereum Virtual Machine enables compabilities across chains like BSC and Avalanche. 

### Platforms
* Geth: A widely used Ethereum client for interacting with nodes. 
* Infura: Cloud-based API service for Ethereum interaction, ideal for developers who want to avoid running their own node. 
* Alchemy: Offers advanced analytics, APIs, and tooling for Ethereum.
* QuickNote: Focused on performance and speed for blockchain data access. 

### Configuration 
* Use an Ethereum client like **Geth** or **Infura** .
* Update the GraphNode config `docker-compose.yml`
```yaml 
environment: 
  ethereum:
    network_name: 'mainnet'    
    rpc: 'https://mainnet.infura.io/v3/${YOUR_INFURA_PROJECT_ID}'
```

### Subgraph Definition 
```yaml 
dataSources: 
  - kind: ethereum/contract 
    network: mainnet 
    source: 
      address: "0xContractAddress"
      abi: "ContractABI"
    mapping: 
      entities: 
        - ExampleEntity 
      eventHandlers:
        - event: ExampleEvent(indexParam: Bytes, nonIndexedParam: Bytes)
        - handler: handleExampleEvent
```

### GraphQL Querying
```
query {
    exampleEntities(first: 20) {
        id
        indexedParam
        nonIndexedParam 
    }
}
```
--- 

## Substrate(e.g., Polkadot) in GraphNode 
### Features 
* Shared Security: Allows multiple blockchains(parachains) to share security.
* Governance: Advanced on-chain governance mechanisms. 
* Interoperability: Parachains are interoperable by design, facilitating cross-chain transactions. 

### Platforms 
* Polkadopt.js: Official interface for interacting with Substrate-based chains. 
* OnFinality: Cloud service offering Substrate APIs and hosting.
* Parity Substrate: Core framework for building Substrate-compatible blockchains. 


### Configuration 
* Use a Substrate RPC endpoint: 
```yaml 
environment: 
  substrate:
    network_name: 'polkadot'
    rpc: 'wss://rpc.polkadot.io'
```

### Subgraph Definitin 
* Adjust `subgraph.yaml` for Substrate: 
```yaml 
dataSources:
  - kind: substrate/runtime
    network: polkadot 
    source: 
      module: balances
      call: transfer 
    mapping:
      entities: 
        - Transfer 
      eventHandlers: 
        - event: balances.Transfer(from: AccountId, to: AccountId, value: Balance)
          handler: handleTransfer 
```

### GraphQL Querying
```graphql
query {
    transfers(first: 5) {
        from
        to
        value
    }
}
```

--- 
## NEAR(e.g., NEAR CLI, Pagoda) in GraphNode 
### Features 
* Nightshard Sharding: Scalable sharding approach 
* Human-Readable Account IDs: Simplifies usability compared to cryptographic wallet addresses. 
* Developer Incentives: Built-in tools for developers , including easy smart contract deployment. 

### Platforms 
* NEAR CLI: Official command-line tool for interacting with NEAR. 
* Pagoda: NEAR's developer platform for analytics and infrastructure. 


### Configuration 
* Use a NEAR RPC endpoint 
```yaml 
environment: 
  near: 
    network_name: 'mainnet'
    rpc: 'https://rpc.mainnet.near.org'
```

### Subgraph Definition 
* Define the subgraph for NEAR event: 
```yaml 
dataSources:
  - kind: near/runtime 
    network: mainnet 
    source: 
      account: "contract.account.near"
    mapping: 
      entities: 
        - Transaction 
      receiptHandlers: 
        - receipt: functionCall(methodName: "method_name")
          handler: handlerFunctionCall 
```

### GraphQL Query 
```
query {
    transactions(first: 10) {
        id
        methodName 
        timestamp 
    }
}
```

--- 

## Arweave (e.g., Bundlr, ArConnect)
### Features 
* Permaweb: Data stored on Arweave is permanent and immutable.
* Profit-Sharing Tokens: Reward models for app developers on Arweave.
* Data Scalability: Focuses on permanent storage rather than mart contracts. 


### Platforms 
* Bundlr: Offers scalable and fast uploads to Arweave.
* ArConnect: Browser extension for managing wallets and Arweave interactions. 

### Configuration 
* Use an Areweave gateway:
```yaml 
environment:
  arweave:
    network_name: 'mainnet'
    rpc: 'https://arweave.net'
```


### Subgraph Definition 
* Define data for Arweave storage: 
```yaml 
dataSources:
  - kind: arweave/storage
    network: mainnet 
    source: 
      address: "transaction_id"
    mapping: 
      entities: 
        - StorageData
      handlers:
        - handler: handleTransactions
```


### GraphQL Querying
```
query {
  storageData(first: 10) {
    id
    data
  }
}
```

---
## Cosmos (e.g., Tendermint, Big Dipper)
### Features
* Inter-Blockchain Communication(IBC): Facilitates seamless cross-chain communication.
* Modular Framework: Developers can build custom blockchains using the Cosmos SDK.
* Energy Efficient: Operates on a Proof-of-Stake mechanism 

### Platforms 
* Tendermint: Core engine for Cosmos-based blockchains. 
* Big Dipper: Blockchain explorer for Cosmos. 

### Configuration 
* Use a Cosmos SDK RPC endpoint: 

```yaml 
environment:
  cosmos:
    network_name: 'cosmoshub-4'
    rpc: 'https://rpc.cosmos.network'
```

## Subgraph Definition 
```yaml 
dataSource:
  - kind: cosmos/transaction
    network: cosmoshub-4
    source:
      type: message 
      action: send 
    mapping:
      entities: 
        - Transaction
      handlers:
        - handler: handleMessage 
```


### GraphQL Querying 
```graphql 
query {
    transactions(first: 10) {
        id
        sender
        receiver 
        message 
    }
}
```

---

## Binance Smart Chain (e.g., Ankr, Chainstack)
### Features 
* EVM Compatibility: Allows Ethereum tools and dApps to work seamlessly on BSC. 
* Low Fees: Optimized for lower transaction costs compared on Ethereum. 
* Fast Transactions: Block times are significantly reduced. 

### Platforms 
* Ankr: BSC API and RPC services for developers. 
* Chainstack: Offers managed BSC nodes for developers. 

### Configuraiton 
* Use a BSC RPC endpoint 
  
```yaml 
environment:
  bsc: 
    network_name: 'mainnet'
    rpc: 'https://bsc-dataseed.binance.org'
```

### Subgraph Definition 
* Define data from smart contracts on BSC: 
  
```yaml 
dataSources: 
  - kind: bsc/contract 
    network: mainnet 
    source: 
      address: "0xBSCContractAddress"
      abi: "ContractABI"
    mapping: 
      entities: 
        - ExampleEntity 
      eventHandlers:
        - event: ExampleEvent(indexParam: Bytes, nonIndexParam: Bytes)
          handler: handleExampleEvent 
```

### GraphQL Querying 
```
query {
  exampleEntities(first: 10) {
    id
    indexedParam
    nonIndexedParam
  }
}
```

