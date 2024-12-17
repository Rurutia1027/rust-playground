# Under Execution Chain and Beacon Chain

## Execution Chain (Ethereum 1.0 / Legacy Chain)

The **Execution Chain** referes to the **Ethereum 1.0 chain** where transactions, smart contracts, and state data were processed. This chain handles the **execution layer** of Ethereum:

- Processes **user transactions**
- Executes **smart contracts**
- Manages the **Ethereum Virtual Machine (EVM)**
- Calculates **balances**, **gas fees**, and **execution states**

It existed **before the Merge** and continued to run alongside the **Beacon Chain** as the **main chain**.

In the **post-Merge Ethereum**, the Execution Chain still processes transactions and interacts with the EVM with the EVM but delegates consensus duties to the Beacon Chain.

### Key Components of Execution Chain

- Transactions
- State update (balances, contract storage, etc.)
- Blocks (executing EVM-based operations)
- Gas fees and rewards (handled within the execution environment)

---

## Beacon Chain (Ethereum 2.0 / Consensus Chain)

The **Beacon Chain** is a parallel chain introduced as part of **Ethereum 2.0**. Its role is to manage **consensus** using **Proof-of-Stake(PoS)** mechanism. The Beacon Chain became the backbone of Ethereum's consensus layer after the **Merge**. It coordinates the network of validators, handles staking, and ensures that blocks are agreed upon in a secure, decentralized manner.

### Key Components of Beacon Chain

- Consensus mechanism (Proof-of-Stake).
- Validator coordination (staking, attestation, and block proposals).
- Rewards and penalties for validators.
- No execution of transactions or smart contracts-it's strictly about achieving consensus.

---

## Understand Consesus

**Consensus** in blockchain refers to the process through which participants in a distributed network agree on the state of the blockchian. It ensures that all nodes (computers in the network) have a unified view of the ledger without relying on a central authority.

There are two major consensus mechanisms in Ethereum's history:

### **Proof-of-Work (PoW)**(this is used before **The Merge**):

- **How it works**: Miners compete to solve complex cryptographic puzzles, and the first to solve it validates the block.
- **Purpose**: Ensures security and decentralization but consumes a lot of energy.

### **Proof-of-Stake (PoS)**(introduced with Beacon Chain):

- **How it works**: Validators are chosen to propose and attest to new blocks based on the amount of ETH they have staked.
- **Purpose**: Achieves consensus with less energy consumption and faster finality while incentivizing honest participation through rewards and penalties.

---

## The Merge

After **The Merge(September 15, 2022)**, the **Beacon Chain** and **Execution Chain** combined to form a single Ethereum blockchain:

- **Execution Chain** continues to process transactions and smart contracts(execution layer).
- **Beacon Chain** ensures consensus with the **Proof-of-Stake** protocol (consensus layer).

## Summary of Execution Chain vs. Beacon Chain

### Key Differences

- **Execution Chain**
  Handles Ethereum Virtual Machine(EVM) execution, including transactions, smart contracts,a nd state updates. Previously relied on **Proof-of-Work** for consensus before **The Merge**.

- **Beacon Chain**
  Responsible for **Proof-of-Stake** consensus, coordinating validators, and ensuring block finality. It does not process transactions but manages attestations(votes) and validator rewards or penalties.

| **Aspect**              | **Execution Chain**                       | **Beacon Chain**                          |
| ----------------------- | ----------------------------------------- | ----------------------------------------- |
| **Role**                | Executes transactions and smart contracts | Handles consensus via Proof-of-Stake      |
| **Consensus Mechanism** | Proof-of-Work (before Merge)              | Proof-of-Stake                            |
| **Focus**               | EVM, state updates, gas fees              | Validator coordination, rewards, slashing |
| **State**               | Manages account balances, storage, etc.   | Manages validator states                  |
| **Blocks**              | Contains transaction data                 | Contains attestations and proposals       |
