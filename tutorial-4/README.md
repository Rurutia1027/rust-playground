# Web3-Indexing-Lab

In this `tutorial-4`, we document notes, scripts, and practices related to [Graph Node](). It includes deploying, debugging, and testing Graph Node in a Docker environment, working with real-world blockchain datasets, and configuring local mock blockchain datasets. Additionally we cover creating and customizing manifest files for defining subgraphs. 

## 🚀 0. What's Graph Node ? 
The GraphNode in the Graph ecosystem serves as a critical component for indexing and querying blockchain data. Acting as a middleware between `off-chain` applicaitons(dApps) and `on-chain` datasets, it bridges the gap by offering powerful abstractions for handling blockchain data.

GraphNode's core functinality lies its abilities to: 
* Index Blockchain Events: 
* Define Flexible Query Models: 
* Integrate with GraphSQL APIs: 

It's elegance stems from ability to abstract complex blockchain data into a graph-like structure, enabling developers to: 
* Define custom query rules through subgraph manifests. 
* Seamlessly execute these rules on blockchain datasets(both real-world and mock/test datasets).
* Retrieve structured results via a standardized GraphQL API. 

This design not only simplifies the complexities of interacting with blockchain data but also empowers Web3 developers to create, test, and deploy dApp backends that efficiently interact with decentralized networks. 

## 🚀 1. Deploying and Debugging Graph Node in Docker
* Step-by-step guides for setting up Graph Node in a Docker environment.
* Tips for debugging common issues during deployment. 
* How to manage Graph Nodes services and logs effectively. 

## 🚀 2. Working with Real-World Blockchain Datasets
* Setting up Graph Node to subscribe to real blockchain data. 
* Configuring encpoints like [Infura]() or [Alchemy]() for Ethereum networks. 
* Best practices for retrieving and processing live blockchain events. 


## 🚀 3. Working with Mock Blockchain Datasets
* Using local blockchain simulation tools such as Hardhat or Ganache. 
* Configuring Graph Node to interact with mocked datasets. 


## 🚀 4. Writing and Customizing Manifest Files
* Understanding the structure of manifest files (subgraph.yaml)
* Defining entities, data sources, and event handlers in the manifest. 
* Examples of mainfest file customization for various use cases. 


## 🚀 5. Adding New Features and Writing Test Cases
* Unit tests, integration tests, and e2e tests and their project's locations, also the CI/CD pipelne incorporates automated scripts to validate changes. 
* How to add new features to the Graph Node codebase. 
* Steps to recompile the Graph Node source code after modifications. 
* Writing test cases for verifying new functionalities and intergraitons.

## 🚀 6. Source Codes Layer's GraphNode's Inner Components 
* Core abstraction definitons 
* GraphNodes relationship with 
* `todo!()`

## 🚀 7. GraphNode's Associations with Different Objects Define in the Blockchain 
* GraphNode && Blocks 
* GraphNode && BlockChain 
* GraphNode && Transactions 
* GraphNode && Events 
* GraphNode && Smart Contract 
* GraphNode && Different Protocls 

## 🚀 8. Blockchain Protocols Supported & Configured in GraphNode
* GraphNode && Ethereum 
* GraphNode && Substrate 
* GraphNode && NEAR 
* GraphNode && Arweave 
* GraphNode && Cosmos 
* Binance Smart Chain(BSC)

