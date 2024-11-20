# Rust Playground  
[![Rust Playground CI](https://github.com/Rurutia1027/rust-playground/actions/workflows/ci.yml/badge.svg?event=status)](https://github.com/Rurutia1027/rust-playground/actions/workflows/ci.yml)  

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

## 🛠️ Setup  

To get started, clone the repository and navigate into the desired project folder.  

```bash
git clone https://github.com/Rurutia1027/rust-playground.git  
cd rust-playground/tutorial-1  
```
**Run Tests** 
All projects include test cases to verify functionality. Use the following command to execute tests:
```bash 
cargo test --nocapture 
```
---

## 📝 Contributing
Contributions are welcome! Feel free to fork the repository, make your changes, and submit a pull request.

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