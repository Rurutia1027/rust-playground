# Understand Beacon(as in Beacon Chain)

The **Beacon Chain** is the core chain of Ethereum's **proof-of-stake(PoS)** network. It manages and coordinates validators, staking and consensus. Unlike the older **proof-of-work (PoW)** chain, the Beacon Chain does not handle user transactions or smart contracts; instead it focuse on:

- Validating blocks.
- Ensuring the integrity of the network.
- Managing validator roles and rewards.

Take the Beacon Chain as the **brain** of Ethereum's **PoS** system, orchestrating the entire blockchain.

# Understand Withdrawals

In the Ethereum **PoS** contex, **withdrawals** refer to the proces of withdrawing funds from a validator account. Validators are participants who stake ETH to secure the network. Over time, they may wish to:

- Withdraw **staking rewards** earned for validating transactions.

- Fully exit and withdraw their entire staked ETH.

# Understand ERC-20 Token Standard

ERC-20 is a **fungible token standard** on the Etherum blockchain. Tokens created using ERC-20 are interchangeable, meaning each token is identical to another. This standard defines six mandatory functions (e.g., transfer, balanceOf) and optional metadata(e.g., name, symbol). ERC-20 simplifies token creation, enables interoprability, and is widely used for cryptocurrencies, utility tokens, and stablecoins.

**Type**: Fungible tokens (identical and interchangeable, like cryptocurrencies).
**Use Case**: Cryptocurrencies, utility tokens, stablecoins.
**Data Structure**:

```json
{
  "from": "0xSenderAddress", // from Smart Contract or EOR (owner wallet)
  "to": "0xRecipientAddress", // to Smart Contract or EOR (owner wallet)
  "value": "1234567890", // Amount in smallest units
  "contractAddress": "0xERC20ContractAddress",
  "tokenName": "TokenName",
  "tokenSymbol": "TKN",
  "tokenDecimal": "18"
}
```

## Functions Defined in ERC-20

ERC-20 defines a set of **standardized functions** that make it easy to create an interact with fungible tokens in Ethereum blockchain.

### `balanceOf(address)`

**What it Does?**: Returns the number of tokens owned by a specific address.
**Role in Scenarios**: If a **DeFi** protocol wants to verify whether a user has enough tokens to paticipate in a staking pool or trade, it uses this function.

### `transfer(address recipient, uint256 amount)`

**What it Does?**: Moves tokens from the sender's account(actuall the wallet account address or Smart Contract address) to another address.
**Role in Scenarios**: This is the most basic transfers mechanism. For instance, a user sends tokens to a friend or pays for a service with `ERC-20 tokens`.

### `approve(address spender, uint256 amount)`

**What it Does?**: Allows another address (a spender) to spend up to a specified amount of tokens on behalf of the owner.
**Role in Scenarios**: This is critical for **smart contracts**. For example, when a user interacts with a decentralized exchange (DEX), they approve the DEX to spend tokens on their behalf.

### `allowance(address owner, address spender)`

**What it Does?**: Returns the remaining number of tokens that a spender is allowed to spend from the owner's account.
**Role in Scenarios**: This is used by smart contracts to ensure the spender doesn't exceed the approved limit. For example, when executing multiple trades, the DEX check this to avoid overspending.

### `transferFrom(address sender, address recipient, uint256 amount)`

**What it Does?**: Move tokens from one account to another, but initiated by a third-party spender who has been approved.
**Role in Scenarios**: A DEX or lending platform typically uses this to handle user funds. For instance, if you borrow tokens from a lending pool, the pool might transfer tokens from your collaterized account to cover the loan.

### `totalSupply()`

**What it Does?**: Returns the total number of tokens in existence.l
**Role in Scenarios**: Important for transparency. For example, projects can use this to calculate the circulating supply of their tokens for metrics like market capitalization.

## Smart Contracts && ERC-20

In some scenarios where smart contracts modify tokens:

**Automated Token Swaps(e.g., UniSwap)**: The approve `transferFrom(...)` function allows DEX(decentrailized-exchange) to swap tokens on behalf of users.

**Token Vesting**: Smart contracts can use `transfer(...)` to periodically release tokens to beneficiaries.

**Burning or Minting Tokens**: While not in the ERC-20 standard, custom extensions often add functions to destroy (burn) or create (mint) tokens to manage supply.

## Data Structures Defined in ERC-20 Associated Scenarios and Functions in Rust

### Token Balance

In Solidity, token balances are represented as mapping (`address => uint256`).

```rust
use std::collections::HashMap;

// Represents a simple ERC-20 token balances structure
struct ERC20 {
  // this is a local memory based cache
  // key is the address {in ERC20 either EOA or Smart Contract address is OK}
  // value is the amount of token this address owns in unit 256
  balances: HashMap<String, u256>
}

impl ERC20 {
  // Check the balance of an address
  fn balance_of(&self, address: &str) -> u256 {
    *self.balance.get(address).unwrap_or(&0);
  }

  // Transfer tokens from one address to another
  fn transfer(&mut self, from: &str, to: &str, amount: u256) -> Result<(), String> {
    let from_balance = self.balances.get(from).cloned().unwrap_or(0);
    if from_balance < amount {
      return Err("Insufficient balance".into());
    }

    // minus the amount of token from 'from' address account
    self.balances.insert(from.into(), from_balance - amout);
    let to_balance = self.balances.get(to).cloned().unwrap_or(0);

    // add the amount of token from 'to' address account
    self.balances.insert(to.into(), to_balance + amount);

    Ok(())
  }
}
```

### Allowances

In Solidity, allowances are represented as mapping (address => mapping(address => unint256)).

```rust

use std::collections::HashMap;

struct ERC20 {
  balances: HashMap<String, u256>, // key: Address, Value: Balance Amount value
  allowances: HashMap<String, HashMap<String, u256>>, // key: Owner, value: { spender -> amount value}
}

impl ERC20 {
  // Approve an address to spend a specific amount of tokens
  fn approve(&self, owner: &str, spender: &str, amount: u256) {
    let owner_allowances = self.allowance.entry(owner.into()).or_default;
    owner_allowances.insert(spender.into(), amount);
  }

  // Check the allowance of a spender for a given owner
  fn allowance(&self, owner: &str, spender: &str) -> u256 {
    self.allowances
      .get(owner)
      .and_then(|spender_map| spender_map.get(spender))
      .cloned()
      .unwrap_or(0);
  }

  // Transfer tokens on behalf of an owner
  fn transfer_from(&mut self, owner: &str, spender: &str, to: &str, amount: u256) -> Result<(), String> {
    let allowance = self.allowance(owner, spender); 

    if allowance < amount {
      return Err("Allowance exceeded".into()); 
    }

    let owner_balance = self.balances.get(owner).cloned().unwrap_or(0); 

    if owner_balance < amount {
      return Err("Insufficient balance".into()); 
    }

    // Deduct from allowance and owner balance 
    self.allowances.get_mut(owner).unwrap().insert(spender.into(), allowance - amount); 
    self.balances.insert(owner.into(), owner_balance - amount); 

    // Add to recipient balance 
    let to_balance = self.balances.get(to).cloned().unwrap_or(0); 
    self.balances.insert(to.into() , to_balance + amount); 
    Ok(())
  }

}
```

### Total Supply

### Putting It All Together

### How [Etherscan](https://etherscan.io/myapikey) Relates?

## Problems Ethereumers Face Before ERC-20 was Innovated

# Understand ERC-721 Token Standard

ERC-721 is a **non-fungible token standard** that allow for creating unique and indivisible tokens. Unlike ERC-20, ERC-721 tokens each have a unique identifier(**tokenId**) and cannot be exchanged on a one-to-one baisis. This standard is commonly used for digital collectibles, gaming items, and tokenized real-world assets. Its uniqueness has revolutionized ownership and provenance in the digital realm.

**Type**: Non-Fungible Tokens (NFTs) -- unique and indivisible.
**Use Case**: Digitial art, collectibles, real estate, etc.
**Data Structure**:

```json
{
  "from": "0xSenderAddress",
  "to": "0xRecipientAddress",
  // tokenId: this is designed to identify the specific NFT being transferred.
  "tokenId": "12345", // Unique identifier for the NFT
  "contractAddress": "0xERC721ContractAddress",
  "tokenName": "TokenName",
  "tokenSymbol": "NFT"
}
```

---

# Understand ERC-1155 Token Standard

ERC-1155 is a **multi-token standard** that supports both fungible and non-fungible tokens within a single smart contract. It allows batch transfers of multiple tokens in a single transaction, saving gas and improving efficiency. This hybrid approach makes ERC-1155 ideal for gaming assets, market places, and use cases where a combination of token types is required. It is more flexible than ERC-20 or ERC-721 alone.

**Type**: Multi-token standard (supports fungible, non-fungible, and semi-fungible tokens).
**Use Case**: Gaming assets, batch token transfers, hybrid fungible/non-fungible token systems.
**Data Structure**:

```json
{
  "from": "0xSenderAddress",
  "to": "0xRecipientAddress",
  "id": "67890", // Token ID (can represent a fungible or non-fungible token)
  "value": "100", // Number of tokens being transferred
  "contractAddress": "0xERC1155ContractAddress",
  "tokenName": "TokenName",
  // tokenSymble represent the type of the token {MTK, NFT, TKN}
  // for multi-token(ERC-1155), non-fungible-token(ERC-721), fungible token(ERC-20)
  "tokenSymbol": "MTK" // Multi-token,
}
```

---

# Understand **Fungible**

The term **fungible** refers to an asset or item that is **interchangeable** with another of the same kind and value. For example:

- **Fungible Assets**: THse are divisible and identical, meaning one unit of the asset is equivalent to another.
  Example:

  > A 1 USD bill is fungible because you can exchange it for another 1 USD bill, and they will have the same value.
  > Cryptocurrencies like Bitcoin or Ethereum are fungible because 1 BTC or 1 ETH is the same as another 1 BTC or 1 ETH.

- **Non-Fungible Assets**: These are unique and cannot be replaced or exchanged on a one-to-one basis.
  Example:
  > A piece of art or a collectible trading card is non-fungible because each hash unique characteristics and value.
  > NFTs(Non-Fungible Tokens) like digital art or gaming assets are non-fungible, as each token represents something distinct.

## How Those Fungible Tokens and Non-Fungible Tokens Realtes to Blockchain ?

- **ERC-20 Tokens**: are fugible because each token is the same as another (e.g.,1 USDT = 1 USDT).

- **ERC-721 Tokens**: NFTs are non-fungible because each token has unique properties (e.g., a specific digital collectible or a piece of virtual land).
