# Rust Playground

[![Rust Playground CI](https://github.com/Rurutia1027/rust-playground/actions/workflows/ci.yml/badge.svg)](https://github.com/Rurutia1027/rust-playground/actions/workflows/ci.yml)

A collection of Rust-based projects for learning, experimenting, and implementing foundational programming concepts. This repository is perfect for beginners and intermediate developers looking to enhance their Rust skills.

---

## 🚀 Projects Overview

### 1. [Tutorial 1: Basics of Rust](./tutorial-1/)

**Focus:**

- Learning basic Rust syntax and features.
- Topics include ownership, borrowing, basic data structures, and control flow.

### 2. [Tutorial 2: Intermediate Rust](./tutorial-2/)

**Focus:**

- Dive deeper into Rust while solving real-world distributed systems challenges.
- **Echo Challenge**: Implement a simple service that echoes received messages.
- **Unique ID Generation**: Build a system that generates globally unique IDs across nodes.
- **Broadcast System**: create a gossip-based broadcast system to share messages efficiently.

### 3. [Tutorial 3: Blockchain Implementation](./tutorial-3/)

**Focus:**

- Building a basic blockchain in Rust, most of the codes are referring from [Blockchain-Rust](https://github.com/GeekLaunch/blockchain-rust).
- Key Features:
  - Genesis block creation.
  - Transaction and block structures.
  - Mining simulation.
  - Hashing and validation logic.

### 4. [Tutorial 4: Graph Node Usage and Source Code Reading Notes](./tutorial-4/)

**Focus:**
This tutorial delves into **GraphNode**, a critical middleware layer for indexing and querying blockchain data, with a focus on Ethereum. We'll lean how Graph Node extract blockchain data, organize it into subgraphs, and store it in relational databases based on a user-defined schema. We also explore how to read and understand Graph Node source codes.

### 5. [Tutorial 5: Advanced Rust Applications in Blockchain and APIs](./tutorial-3/)

**Focus:**
Building a production-grade Rust application with real-world use cases, such as implementing a key-value store, managing configuraiton environments, integrating with Ethereum, and handling event-driven workflows. This tutorial showcases how to create a robust system that interacts with blockchain data and local databases.

**Key Features:**

1. Key-Value Store(KVStore):
2. Environment Loader and Config Management:
3. Framework API Binding with Handlers:
4. Ethereum Event Subscription:
5. Data Synchronization to Local Database:
6. Monitoring Blockchain Events:
7. Handler and Listener Design:

**Project Files:**

- Code and examples are in the `tutorial-5` folder.
- Contains implementations for KVStore, Database Operations, Ethereum event monitoring & handling, and API bindings.

**Modules:**

```rust
todo!();
```

**Goals:**

- Provide a clear path to building production-ready applications.
- Combine blockchain intergration with local database management.
- Enhance understanding of Rust's ecosystem, async programming, and database interactions.

**References:**
We referred to several amazing open-source repositories, listed below with brief descriptions:

- [eth-analysis-rs](https://github.com/ultrasoundmoney/eth-analysis-rs): A Rust-based toolkit for analyzing Ethereum blockchain data, including transaction tracing and state changes.
- [mars](https://github.com/deepeth/mars): A service for querying and storing Ethereum data efficiently, providing a robust data pipeline for blockchain analysis.

---

## 🛠️ Setup

To get started, clone the repository and navigate to the desired project folder:

```bash
git clone https://github.com/Rurutia1027/rust-playground.git
cd rust-playground/
```

This repository is **not project-oriented**. Instead, it contains **classic Rust examples** and small, modular code snippets to demonstrate essential concepts and practical use cases. 
Each tutorial highlights specific topics:
In **Tutorial-5** we introduce **blockchain-oriented practices** and **database** oriented operations and test cases: 

To ensure all test cases executed correctly, we have to deploy a Database of PostgreSQL locally by following steps, then we can make sure the local test cases can be executed successfully. 

###  Database Setup(Tutorial 5 unit test cases rely on)
**1. Set Up PostgreSQL**
Deploy PostgreSQL DB locally or via Docker, and create a PostgreSQL database named `defaultdb` with the following credentials: 
**username**: admin 
**password**: admin 

* Dockerfile setup is ok
```
docker run --name postgres-db-container \
  -e POSTGRES_USER=admin \
  -e POSTGRES_PASSWORD=admin \
  -e POSTGRES_DB=defaultdb \
  -p 5432:5432 \
  -d postgres
```
  
**2. Export Database URL as Environment Variable**
Export the deployed ok database connection URL string as an environment variable so that the test cases can access it. 
In [GitHub CI/CD] environment, unit test cases require environment variables that are already declared in each job. 

```shell
export DATABASE_URL=postgresql://admin:admin@localhost:5432/defaultdb
```

**Run Tests**
All projects include test cases to verify functionality. All of the test cases should be passed in the local environment and [GitHub CI/CD]. 
Use the following command to execute tests in local environments: 

```bash
cargo test -- --nocapture
```
The `--nocapture` flag ensures that `println!` messages in unit test cases are visible on the console during test execution.  

---

## 🌟 Features

- **CI/CD Pipeline**:  
  Continuous integration is set up using GitHub Actions to ensure all projects are built and tested.  
  ![Rust Playground CI Status](https://github.com/Rurutia1027/rust-playground/actions/workflows/ci.yml/badge.svg?branch=main)

- **Hands-On Examples**:  
  Each tutorial is a standalone project with well-documented code and tests.

- **Learning Focus**:  
  Tutorials are designed to teach Rust concepts while building meaningful projects.

---

## 📝 Contributing

Contributions, discussions, and issues are always welcome! This is not a formal project, but rather a playground for learning and experimenting with Rust.
If you have **good ideas**, suggestions, or want to collaborate, feel free to: 
- Fork the repository, make your changes, and submit a **pull request**.
- Open an **issue** to discuss bugs, improvements, or new ideas.
- Reach out to me directly via email: [rurutia1027@gmail.com](rurutia1027@gmail.com).

Let's learn and build together! 🚀

---

## 📚 Resources

### Blockchain References

- [Bitcoin Transaction Rules](https://en.bitcoin.it/wiki/Protocol_rules#tx_messages): Learn about the protocol rules for Bitcoin transactions.
- [Coinbase Transaction](https://bitcoin.org/en/glossary/coinbase): A glossary entry explaining the concept of a Coinbase transaction.
- [How is a transaction’s output signed?](https://bitcoin.stackexchange.com/q/45693): Explanation of transaction output signing in Bitcoin.
- [What is a double-spend?](https://bitcoin.stackexchange.com/q/4974): An introduction to the concept of double-spending in blockchain.

### Tutorial Resources

- [Blockchain-Rust GitHub Repository](https://github.com/GeekLaunch/blockchain-rust/): The original tutorial repository for implementing blockchain in Rust.
- Tutorial Videos:
  1. [Build a cryptocurrency! - Blockchain in Rust #01: Blocks & Hashing](https://www.youtube.com/watch?v=vJdT05zl6jk)
  2. [Making Magic Internet Money? - Blockchain In Rust #02: Mining](https://www.youtube.com/watch?v=PZGlYa-6U5U)
  3. [BlockCHAINs and Verification tests - Blockchain In Rust #03: Chains & Checks](https://www.youtube.com/watch?v=buYvuIPdwHU)
  4. [Adding Transactions to the Blockchain: Concepts - Blockchain in Rust #04: Transactions 1](https://youtu.be/-k1Yk9D_lU4?si=6Q7c71jlAEC1vz0e)
  5. [Adding Transactions to the Blockchain: Implementation - Blockchain In Rust #05: Transactions 2](https://youtu.be/1t4TXnB4Qj4?si=St1OkysJemWqdUzF)

### Rust Programming Course

- [Rust Programming Course: From Beginner to Expert 2.0 (Udemy)](https://www.udemy.com/share/1062Ck/): A comprehensive Rust course for all levels.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
