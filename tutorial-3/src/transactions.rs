use std::collections::HashSet;
use std::fmt::{self, Debug, Formatter};

use crate::{
    block::Block, hashtable::Hashtable, u64_bytes, Address, Hash,
};

/*
In a blockchain, a transaction represents a record of a state change, typically a transfer of value (liek cryptocurrency or tokens).
Transactions are the fundamental building blocks of blockchain systems.

Relationship between Transactions and Blocks:
- 1:N
A block contains a list or batch of transactions. One block can contain multiple transactions, but each transaction belongs to exactly one block.

- M:N (Indirect Relationships -- of course, because it is a directed graph !)
Across the blockchain, the relationship between transactions and blocks may appear as multiple-to-multiple because:
> A single transaction can involve outputs from multiple blocks (when spending coins received from different transactions in previous blocks.)
> The inputs of one transaction can come from the output of transactions in multiple blocks.

Structure of Transactions

A transaction in a blockchain is often structured with inputs & outputs:

- INPUT:
Input specify where the coins comes from. Each input references the output of a previous transaction (the coins begin to spent).
Input includes:
> Reference to the previous transaction's output(e.g., a unique identifier or hash).
> A cryptogrpah signature proviing ownership of the output being spent.

- OUTPU:
Outputs specify where the coins are going. Each output includes:
> The recipient's address(public key).
> The amount of value (e.g., coins) being transferred.
> A coindition or script that must be satisfied to spend this output.

We can treat the input(s) & output(s) as a ledger of payments.
Inputs are the sources of funds and Outputs are the destinations of funds.

Relationship Between Blockchain and Transactions.

The blockchain is a distributed ledger that orgnizes all transactions into blocks.
Each block:
- groups a batch of transactions.
- endures the integrity of those transactions by hashing and linking the block to the previous block.

(* i think this is useful *)
Transactions form the backbone of the blockchain's purpose: recording the transfer of assets or values in a secure and immutable manner.


(**) And here are few things to append here, that is (**):
(1) Users create transactions to transfer coins - exchange
(2) Miners/validators collect transactions into blocks and validate them - mining & validating & provide <storage resources & computing resources>
(3) The block is added to the blockchain, making the transactions part of the ledger.
todo: maybe I can expand this to the physical layer's Node's concept.


Here I got a question and trying to explain it:
That is we already know there are Users, Miners or Validators exists in the blockchain, and they play different roles and provide different operaitons in the blockchain.
So, in which scenario who provide the physical layer's resources like the Node ?
We know that Blocks are stored on the Nodes of the block chain, and Nodes are more physical layer's concept.

> Anyone can add a node:
>> (A) In a public blockchain, any user can set up a node or a minder node by downloading the blockchain software,
configuring it, and joining the network. There is no central authority controlling node membershp.
>> (B) In a private blockchain,(e.g. hyperledger, Quorum) it is controlled by adminisrators or organizaitons, in such environment, node management is centralized. Administrators define who can join as a node and what role each node plays.
>> (C) Consortium Blockchains(e.g., Corda, Ripple), this managed by a group of stakeholders, and they have some pre-defined rules.

## How may kinds of nodes are there ?
A node is any device (computer, server, mobile phone) connected to the blockchain network. It runs the blockchain software and participates in the network by :
>> storing data (part or all of the blockchain)
>> validating transactions and blocks
>> propagating information to other nodes(transactions, blocks, or consensus messages)
(a) Full Node: this stores a complete copy of the blockchain ledger and enforces all the rules of the protocol,
---> like: Bitcoin Core node, Ethereum full node;

(b) Light Node(SPV Node): this stores only parts of the blockchain data, typically the block headers,
---> like: Wallet apps that uses Simplified Payment Verfications(SPV) in Bitcoins.

(c) Miner/Validator Node: specialized nodes that participate in the consensus mechanism by validating and proposing new blocks.
---> like: Ethereum validator, Bitcoin miner.

(d) Archive Node: a full  node that stores all historical states of the blockchain(not just the current state)
---> Ethereum archive node .

## How many kinds of valdiations are there in different layers of blockchain ?


Maybe we can divide into different perspecitve by roles?
> Suppose a Minder want to mine blockcoins on a bunch of blocks, what validaitons he/she needs todo ?
> And what validations & calculations does a validator needs to do ?
> And what a user need to validate during the process of he/she trying to purchasing a coin in the blockchain?
*/

pub struct Output {
    // operator's address
    pub to_addr: Address,

    // spending/receiving actual coin value
    pub value: u64,
}

impl Output {
    pub fn new(to_addr: String, value: u64) -> Self {
        Output { to_addr, value }
    }
}

/*
Implement trait `Hashtable` and implement its inner function :bytes.
In it's bytes function we create a vector of u8 and convert all variables defined
in struct Output into Vec<u8> append to the bytes vector.
*/
impl Hashtable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend(self.to_addr.bytes());
        // here covnerted self#value from u64 into len = 8's Vec<u8> vector
        bytes.extend(&u64_bytes(&self.value));

        bytes
    }
}

/*
Define Transaction
*/
pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

/*
Add functions associated with struct Transaction
fun1: input_value: accumulate each input item's value together, to calculate total spending value in current Transaction.
fun2: output_value: accumuate each output item's value together, to calculate total receiving value in current Transaction.
fun3: input_hashes: traverse each Output item that stores in vector of Vec<Output>, and get its hash value append to HashSet
fun4: output_hashes: traverse each Output item that stores in vector of Vec<Output>, and get its hash value append to HashSet
*/
impl Transaction {
    /**
     * here we provide an implementaion of creating a new instance of the Transaction struct.
     * refering to the implemnetaiton of the new function that defined in the Block.
     */
    pub fn new(
        inputs: Vec<Output>,
        outputs: Vec<Output>,
    ) -> Self {
        Transaction { inputs, outputs }
    }

    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|input| input.value).sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|output| output.value).sum()
    }

    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<Hash>>()
    }

    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<Hash>>()
    }
    /*
      Notes for coinbase in blockchain.
      A coinbase transaction is a special type of transaction in a blockchain system, typically the first transaction in each block.
      The purpose of coinbase is to reward the miner(or validator) of the block with newly created coins and possibly include transaction fees.

      - Coinbase transactions: with its inputs(spending) empty, this introduces new coins to the system(and the new coins are the reward of the miner).
            - inputs: None
            - outputs: transfers the newly minted coins to the miner's address.

      - Regular transactions: with both non-empty inputs(spending) and outputs(receiving),
            - inputs: spends outputs from previous transactions.
            - outputs: transfers the value to new addres.

    */
    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }

    pub fn is_validate(&self) -> bool {
        // todo: add more validation details here to verify
        // 1. whether this transaction is validated
        // 2. whether the inputs and outputs in the scope fo the transaction will be
        //    ready to be calcuated as expeted.
        true
    }

    pub fn input_total_value(&self) -> u64 {
        self.inputs.iter().map(|item| item.value).sum()
    }

    pub fn output_total_value(&self) -> u64 {
        self.outputs.iter().map(|item| item.value).sum()
    }
}

/*
Let Transaction implement trait of Debug and add more inner details to the function of fmt.
*/
impl Debug for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Transaction[{:?}]: is_coinbase: {:?}, is_validate: {}, input_len: {}, output_len: {}, input_total_value: {}, output_total_value: {}",
            hex::encode(&self.hash()),
            &self.is_coinbase(),
            &self.is_validate(),
            &self.inputs.len(),
            &self.outputs.len(),
            &self.input_total_value(),
            &self.output_total_value()
        )
    }
}

/*
Let Transaction implement trait of Hashtable and implement its inner declared method: bytes(...)
*/
impl Hashtable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        // here we iterate each element:Output that stores in vector of input: Vec<Output>
        // and invoke each element:Output's hash bytes function to get its bytes vector
        // and then append the vectors to bytes: Vec<u8>
        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|item| item.bytes())
                .collect::<Vec<u8>>(),
        );

        // here we iterate each element:Output that stores in vector of output: Vec<Output>
        // and invoke each element:Output's hash bytes function to get its bytes vector
        // and then append the vectors to bytes: Vec<u8>

        bytes.extend(
            self.outputs
                .iter()
                // we use flat_map here, because in iteration
                // one item may be converted into multiple Vec<u8> items, we cannot only handle the
                // first element, so we need to use flat_map to retrive all of them
                .flat_map(|item| item.bytes())
                .collect::<Vec<u8>>(),
        );

        // return bytes of both input bytes value and output bytes value
        bytes
    }
}
