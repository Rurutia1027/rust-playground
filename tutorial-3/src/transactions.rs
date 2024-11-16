use std::collections::HashSet;

use crate::{hashtable, u64_bytes, Address, Block, Hashtable};

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
Outputs specify

*/
