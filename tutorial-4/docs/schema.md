# Schema.graphql in GraphNode 
The schema.graphql file is an essential part of defining a subgraph in GraphNode. It's manually created by the developer and serves as a blueprint for the data you want to extract, transform, store, and query from the blockchain.

The schema.graphql file is entirely in developer's control. It bridges the gap between raw blockchain data and your application's business logic. While some aspects of its use depend on the blockchain protocol, the content is driven by your specific data needs. By mastering its deisng, we can efficiently query, store, and use blockchain data in our dApp projects. 

During `graph deploy` step GraphNode will scan the `schema.graphql` and translate its inner info into PostgreSQL DB's table, table schemas, and tables' relationship. 


## What is `schema.graphql` ? 
* Developer-Defined: Unlike some generated files(e.g., TypeScript classes from `graph codegen`), `schema.graphql` is created manually by develoeprs. It defines the structure of the data you care about, tailored to your applciation's needs. 

* Purpose-Driven Design: It represents your business logic by specifying: Entities, (Entity Inner)Fields, Relationships between entities(one to one, one to multiple, or multiple to multiple).

## Workflow and Connections to Other GraphNode Workflow Steps: 
#### Declaration in subgraph.yaml 
* When we declare in `subgraph.yaml`, we need to add a reference ot already created and intialized `schema.graphql` file, as:

```yaml 
schema:
  file: ./schema.graphsql 
```

#### Running `graph codegen`
* The scehma.graph provides the foundation for generating TypeScript/AssemblyScript classes. 
* Events and entity definition in the smart contract ABI are used to create strongly-typed classes. 

#### Execution Process: 
* When deploying a subgraph, the schema.graphql guides how data is stored in underlying PostgreSQL database. 
* The structure dclared in the GraphQL schema becomes SQL table definitions during deployment. 

## What Does schema.graphql Contain? 
* Entities and Fields: 
```graphql 
type Transfer @entity {
    id: ID!
    from: Bytes!
    to: Bytes!
    value: BigInt!
    timestamp: BigInt!
}
```

## Why Declare Event Fields in subgraph.yaml?
The declaration of event fields in subgraph.yaml serves a different purpose than entity definitions in `schema.graphql`.

#### Events in `subgraph.yaml`:
* The eventHandlers section specifies the blockchain events the subgraph should listen to. These events come from the smart contract’s ABI (Application Binary Interface) and describe the structure of the raw data emitted by the blockchain.

* The field declarations (e.g., indexed address, uint256) mirror the event’s arguments as defined in the smart contract. This helps Graph Node understand what data to capture when the event is triggered on-chain.

#### Entities in `schema.graphql`:
* The schema.graphql defines how data is stored in the database and queried via GraphQL. It is entirely up to you to decide which fields from the event (or other sources) are necessary for your application.
* Entities are more abstract and can aggregate data from multiple events or combine raw blockchain data with derived fields.


#### Key Difference:
* subgraph.yaml: connects to the blockchain and listens for specific events, defining their structures based on the ABI. 
* schema.graphql: stores the data transformed from those events, representing your app's logic and data model. 

