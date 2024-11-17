# Miner in Blockchain 
## Miner Joining the Blockchain
When a miner joins the blockchain network, they do not create a new block or coinbase transaction just by joining. Instead, the miner begins participating in the mining or validation process: 
1. For Proof-of-Work(PoW) blockchains: 
The miner competes to solve a cryptograph puzzle to create the next block. 

2. For Proof-of-Stake(Pos) blockchains: 
Validators(miners in PoS) are selected based on their staked coins to validate blocks. 

## Miner's Role in Block Creation 
A miner's job is to:
### Collect Transactions:
To gather valid transactions from the transaction pool(mempool).

### Verify Transactions:
Ensure that each transaction is valid(e.g., signatures are correct, inputs match outputs, etc.)

### Construct the Block:
Bundle the verified transactions into a block.
Add a coinbase transaction to include the mining reward and transaction fees. 

### Solve or Verify Consensus Puzzle: 
In PoW, solve a cryptographic puzzle(find a valid hash).
In PoS, validate the block if they are selected as the validator. 

### Broadcast the Block: 
Once the block is successfully mined or validated, broadcast it to the network. 

## Key Activities Within a Block's Scope:
Once a miner successfully creates or validates a block: 
### Coinbase Transaction:
* The Miner includes the coinbase transaction in the block.
* This transaction rewards the miner with newly created coins and the transaction fees from all included transactions. 

### Verification Job:
* The miner ensures that all transactions in the block are valid before broadcasting the block.

### Adding the Block:
* The block is appended to the blockchain, making it part of the permanent ledger. 

## Import Notes: 
* Miner Joins != New Block: 
> A miner joining the network doesn't creates a block or receive a coinbase reward immediately.
> They must successfully mine or validate a block to earn rewards. 

* Block Creation is Competivive: 
> In PoW, miners compete to solve a puzzle first. 
> In PoS, only selected validators can create a block.


* Job Ends WHen Block is Done: 
> Once the miner successfully creates or validates a block, their work for that block is complete.
> They start working on the next block in the chain. 

## Workflow 
### Step-1: Miner Joins
* Begins mining(PoW) or waiting to validate(PoS).

### Step-2: Block Created 
* If the miner is successful to mining/validating all transactions in that Block, it called they create/validate a block.
* At this point, a coinbase transaction is generated!!!
- The coinbase transaction creates new coins(block reward) and includes the transaction fees from other transactions in the block.
- The reward and fee are assigned to the miner's address. 

### Step-3: Block Added to Blockchain 


### Step-4: Process Repeats