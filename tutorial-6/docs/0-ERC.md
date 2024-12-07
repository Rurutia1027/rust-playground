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
