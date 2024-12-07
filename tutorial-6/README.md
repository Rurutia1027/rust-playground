# Tutorial-6: Subscribing Data via Etherscan.io Platform API Endpoints

## Introduction of Tutorial

In this tutorial, we will explore how to interact with the [Etherscan.io](https://docs.etherscan.io/api-endpoints) platform to subscribe to data. We will focus on extracting data using specific timestamps-both start and end-and convert the data to relevant formats that can be utilized for further processing. The primiary objective of this guide is to familiarize with the structure of the data provided the [Etherscan.io](https://docs.etherscan.io/api-endpoints).

## Introduction of Etherscan.io

It is a popular block explorer and analytics platform for the Ethereum blockchain. It provides a wide variety of data related to Ethereum transactions, blocks, addresses, tokens, and more. It allows users to easily access, search, and analyze blockchain data, making it a valuable tool for developers, investors, and anyone interested in interacting with the Ethereum network.

## Key Data Structures and Features of Etherscan:

### Transactions

There are multiple definitions of Transactions, like **Normal Transactions**, **Internal Transactions**, and **Transaction Status**.

**Normal Transactions**: Etherscan tracks regular Ether(ETH) transactions between addresses. This includes details such as transaction hash, block number, timestamp, sender and receiver addresses, and the transaction value (in ETH or Gwei units).

**Internal Transactions**: Etherscan also tracks internal transactions(which are transactions triggered by smart contract execution). These transactions involve contract calls and token transfers and often happen within the context of decentrailized applciations(dApps).

**Transaction Status**: For every transaction, Etherscan shows the status (whether it succeeded or failed), gas used, and the corresponding miner or validator.

### Blocks

**Block Information**: For each block on the Ethereum blockchain, Etherscan provides a block number, the block hash, miner information, timestamp, the number of transactions included in the block, and the block's gas used.

**Uncles(or Ommer Blocks)**: These are blocks that were valid but not included in the main chain. They are typically valid but were produced too late in relation to other blocks.

### Addresses

**Account Balances**: Etherscan tracks Ether and token balances for each address. It also shows whether an address is externally owned (EOA) or a contract.
**Token Transfers**: This provides data on _ERC-20_ token transfers for a specific address, allowing users to track the movement of tokens within the Etherum ecosystem.

**ERC-721(NFTs)**: Similar to _ERC-20_ transfers, Etherscan provides data on NFT transactions that involve _ERC-721_ tokens.

### Contracts

**Contract Information**: Smart contract deployed on the Ethereum network can be accessed to view their ABI (Application Binary Interface), source code, bytecode, and contract interactions. Etherscan allows users to verify and interact with contracts directly through its interace.

**Read/Write Contract Functions**: Some contracts allow users to read data(such as token balance or status) or perform write actions (such as making a transaction or calling a function).

### Tokens

**ERC-20 Tokens**: THe platform provides extensive data on _ERC-20 Tokens_, including their total supply, transfers, holders, and token price information.

**ERC-721 Tokens**: For NFTs, users can track transfers, owners, and other metadata associated with each token.

### Gas Tracker

**Gas Price**: Etherscan provides real-time data on Ethereum gas prices. This includes current, fast, and standard gas fees required to successfully execute transcations or interact with smart contract on the Ethereum network.

**Transaction Costs**: It also estimates the transaction costs based on current gas prices and the size of the transaction.

### Logs and Events

**Event Logs**: Ethereum contracts emit logs when certain events(such as transfers or contract interactions) occur. Etherscan allows users to query these logs based on various fliters(e.g., address, topics).

### API

**Etherscan API**: Etherescan provides a comprehensive API that allows developers to access all the aforementioned data programmatically. This API can be used to query transaction details, address balances, block information,a nd much more information.
