# Understanding the GraphNode Workflow Through Protobuf

In previous notes, we mentioned how to use GraphNode extract real data and store the data items to database in [8_blockchain_protocol](./8_blockchain_protocol.md). This note will add more details of the data flow and how GraphNode has similariteis as the Protobuf. 

Let's review the steps of using GraphNode subscribe datasets from real blockchain platform, that is: 

## 1. step-1: we need to add the platform(e.g., ethereum) type, endpoint those configure options in the `subgraph.yml` file, like: 
```yaml 
graph-node:
  environment: 
    ethereum: 'mainnet:http://localhost:8545'
```

## 2. step-2: configure IPFS in `subgraph.yml`

```yaml 
ipfs:
  environemnt: 
    ipfs: 'http://localhost:5001'
```

## 3. step-3: add a Subgraph Manifest to `subgraph.yml`, as: 

```yaml 
specVersion: 0.0.2
description: A subgraph for tracking token transfers.
schema:
  file: ./schema.graphql
dataSources:
  - kind: ethereum/contract
    name: Token
    network: mainnet
    source:
      address: "0x..."
      abi: Token
      startBlock: 12345678
    mapping:
      kind: ethereum/events
      apiVersion: 0.0.4
      language: wasm/assemblyscript
      entities:
        - Transfer
      eventHandlers:
        - event: Transfer(indexed address, indexed address, uint256)
          handler: handleTransfer
      file: ./src/mapping.ts
```

The declaration is very similar to the developers define `.proto` files based on the Protobuf framework.
When we try to define some RPC protocols we often use `.proto` file, and in it we declare both the definitons of messages and rpc interfaces.
Then handle the `.proto` to the Protobuf provided compiler we can get different sources codes that implement the messages and interfaces. 

Similar to step-3, we declare the Object's name and its inner fields in `dataSources#mapping#entities` and `dataSources#mapping#entities#eventHandlers#event` it just like we declare message and message inner fields in Protobuf. 


Then we continue declare the functions' signatures(function name and its parameters) in the `dataSources#eventHandlers#handler`, it just like we declare the interface in Protobuf. 

GraphNode tries combine the handler's implementation code file with its declaration the `subgraph.yml` together by adding an anchor `datasources#mapping#file` this gives in which source code gives the exactly the implemention logic of the handler's this interface, and typescript is always the coding language that used to implement the handler function. 

## 4. step-4: Execute GraphNode CLI and let it genearte the objects and handlers under the path.
```shell 
graph codegen
```

After executing the above command, we can find the delcared entities are write to the `schema.graphql` and `typescript` files underspecified folders. Then we can continue add implementation logics based on the generated source codes of typescript.  

## 5. step-5: Implement the handler by adding more manipulate details in `anchor file` which is `file: ./src/mapping.ts`, as:

```typescript 
import {Transfer} from "../generated/schema"; 
import {Transfer as TransferEvent} from "../generated/Token/Token"; 

export function handleTransfer(event: TransferEvent): void {
  let transfer = new Transfer(event.transaction.hash.toHex());
  transfer.from = event.params.from;
  transfer.to = event.params.to;
  transfer.value = event.params.value;
  transfer.save();
}
```

## 5. step-5: Retrieve Datasets from Blockchain Platform

Blockchain Raw Data -> Filter -> Converted -> Handler Manipulation -> Stored to PostgreSQL DB

```shell 
graph build 
```

---
# Workflow of GraphNode 
## 1. Declaring the Subgraph
- Begin by defining the subgraph in `subgraph.yml`. 
  - **Smart contract addresses**
  - **Events and handlers**
  - **Entities and their relationships**(defined in `schema.graphql`)

## 2. Executing `graph codegen`
- After defining the subgraph, run `graph codegen` command. 
- This command generates: 
  - **Entity Classes**: Based on `schema.graphql`, creates TypeScript/AssemblyScript classes. 
  - **Event Types**: From the smart contract ABI, generates strongly-typed classes for events. 

## 3. Handler Implementation in Typescript 
- Once the types are generated, implement the event handlers in `mapping.ts`
  - Use the `Transfer` class from `../generated/schema` to create and save data(from blockchain platform) to the database. 
  - Use the `TransferEvent` class from `../generated/Token/Token` to access raw event data emitted by the smart contract. 

## 4. Binding Types to Handlers 
- The handlers declared in `subgraph.yml` are mapped to your implementations in `mapping.ts`.
- Ensure that the event names and handler functions match exactly. 

## 5. Execution 
- After implementing the handlers, proceed to build and deploy the subgraph using: 
  - `graph build` and `graph deploy` commands. 
- The deployed subgraph:
  - Listens to the blockchain events. 
  - Processes them using the handlers.
  - Stores the structured data as defined by your entities. 

## 6. Queried PG DB Stored Data via GraphQL Grammar 
- Once the subgraph has been deployed and starts processing blockchain events, the data is stored in the PostgreSQL database configured for your GraphNode.
- You can query this data using **GraphQL** via the exposed GraphNode endpoint. 

### Examples GraphQL Query 
- The following example demostrates how to fetch the first 10 transfers from the `Transfer` entity: 
```graphql 
query {
    # transfers is the name of the schema name defined in the schema.graphql 
    # first describe we only fetch top-10 of the return datasets         
    transfers(first: 10) { 
        # id, from, to , value are specified inner fields we declared in the `subgraph.yml`'s entities 
        id
        from
        to
        value 
    }
}
```
--- 

# Actions of Graph CLI Commands Trigger 
## Graph Codegen:
* This step focuses purely on generating TypeScript or AssemblyScript binding for entities(from schema.graphql) and event types (from ABIs).
* It does not with PostgreSQL. In this step no PostgreSQL DB tables are defined. 

## Graph Build: 
* This compiles the subgraph source code into WebAssembly(Wasm).
* In this step no PostgreSQL DB tables are defined. 

## Graph Deploy: 
* This step PostgreSQL becomes active. 
* When the subgraph is deployed to the Graph Node, the Graph Node reads the schema.graphql and it is the **GraphNode** in charge of translate it into SQL schema definitions for the PostgreSQL database. Specificially: 
  * **Entities** declared in the schema.graphql are translated into tables in PostgreSQL.
  * **Fields** of those entities are translated into columns in those tables. 
  * **Relationships** (like the @derivedFrom directives) are translated into SQL constrains or joins. 
* After doing those mappings, GraphNode executes the SQL commands against the PostgreSQL database to create the required schema and tables. 
* After This step, PostgreSQL is ready to store the data (queried from blockchain, after filtering, converting)

## Syncing Blockchain Data: 
* Once subgraph is deployed, the Graph Node begins syncing blockchain data according to the subgraph's configuration. 
* Listened events emitted from smart contracts are processed by the `subgraph.yml` declared `mapping.ts` implemented handlers. 
* Data is transformed by your handlers and stored in the PostgreSQL tables created eariler. 
