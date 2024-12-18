# Understand Issuance in Beacon Chain

In the context of Ethereum's **Beacon Chain**, **issuance** refers to the **creation of new ETH** as rewards for validators in the **Proof-of-Stake(PoS)** consensus mechanism.

## What is Issuance ?

- **Issuance** is the process of mining new ETH as a reward for network participants.
- In Ethereum's **Proof-of-Stake** system (the Beacon Chain), validators are incentivized to propose and attest to blocks correctly. These incentives are paid out as **newly issued ETH**, which increases the total supply of ETH over time.
- This is similar to how miners were rewarded in **Proof-of-Work(PoW)** pre-Merge, but in **PoS**, the rewards go to validators instead of miners.

---

## Why Does the Beacon Chain Issue ETH ?

The issuance mechanism serves the follow purposes:

### Incentivize Validators

- Validators must stake **32 ETH** to participate in block validation.
- Issuance (rewards) encourages validators to maintain the network's security, propose new blocks, and attest to the blocks propsed by others.

### Secure the Network

- Validators earn rewards for behaving honestly (proposing/attesting to valid blocks).
- Misbehavior, like being offline or submitting malicious attestations, results in penalties or slashing.

### Sustain Network Operations

- ETH issuance ensures validators are eonomically incentivized to participate over the long term.

---

## How Issuance Works on the Beacon Chain?

#### **ETH Rewards** are issued to validators based on their **performance**:

- Proposing a block
- Attesting to other blocks
- Participant in sync committees(helping the chain's finality).

#### Validators Earn Rewards

Validators earn both **base rewards** and **additional rewards** for correctly contributing to consensus.

#### Source of Issurance

- New ETH created at the protocol level as validator rewards.
- This issuance increases the total ETH supply, but with **burn mechanisms** (like EIP-1559 on the execution layer), issuance can be offest by burned ETH.

---

## Issuance Rates on the Beacon Chain

- Issuance on the Beacon Chain depends on the **number of validators** in the network.
- As more ETH is staked:
  The total issuance increases because more validators are rewarded. However, the issuance per validator **decreases** due to the dilution effect.
- Ethereum's design balances issuance to ensure that:
  The network is sufficiently secure, and validators earn a reasonable yield without overly inflating the ETH supply.

**Formular(Simplified)**:

- The rewards for each validator are proportional to their **effective balance** (capped at 32 ETH) and the overall **network size**.

- `Burned ETH = base fee * gase used`

---

## Issuance vs. Burn (Net Issuance)

- After **EIP-1559**(introduced in Auguest 2021), a portion of translation fee is **burned** on the execution layer.
- This burning mechanism can offset or even exceed the issuance of ETH on the Beacon Chain.
- When ETH burned > ETH issued -> **ETH becomes deflationary**(total supply decreases).

---

## Key Takeaways

- **Issuance** on the Beacon Chain refers to ETH created to reward validators for their participation in the **PoS** system.
- Issuance is critical for incentivizing validators and maintaining network security.
- The **net issuance** of ETH depends on the balance between new ETH created (via Beacon Chain rewards) and ETH burned (via translation fees).

---

## Rea-Life Example

- A validator with **32 ETH** staked might earn approximately **4%-5% annual yield** from rewards issued on the Beacon Chain.
- Meanwhile, Ethereum's translation fee burning (from EIP-1559) may reduce the total supply, making ETH potentially **deflationary** despite issuance.

In Summary, issuance on the Beacon Chain ensures that validators are incentivized to secure the network, but Ethereum's overall design aims to balance issuance with fee burns to keep ETH's supply stable or deflationary.

---

## How Does Burn(Issuance) Prevent Inflation

The **burn** mechanism is primarly derived from the improvements introduced by **EIP-1559**. It reduces the total supply of ETH by burning a portion of translation fees(the **base fee**), thereby offsetting the supply increase caused by **issuance**. Here's how it works:

- **Supply Increase from Issuance**: Validators receive rewards(ETH) for their consensus and validation work, which increase the total supply of ETH.
- **Supply Reduction from Burn**: When users execute transactions, the **base fee** they pay is directly burned(destroyed), instead of being given to miners or validators. This destroy portion of ETH offsets the supply increase caused by issuance.

In simple terms, issuance is the "creation" of ETH, while **burn** is the "destruction" of ETH. Together, they ensure that the total supply of ETH remains balanced to some extent, preventing inflation.

---

## How is the Burn Mechanism Made Transparent ?

The **burn mechanism** is fully transparent and traceable on the Ethereum blockchain. Every burned ETH is recorded in the transaction data, and users can verify these records on block explorers like **Etherscan**. Specifically:

- **Base Fee Burning**: The base fee (introduced by EIP-1559) is programmatically calculated and included in each transaction. This ensures the burning process is predictable and follows the protocol's rules.

- **On-Chain Transparency**: The burned ETH is sent to a special address(often referred as a **black hold address**) that has no private key, meaning the ETH sent there is irretrievably lost. Anyone can view the transactions to this address on the blockchain.

- **Auditing**: Because Ethereum is an open, public blockchain, anyone can audit and monitor the burn mechanism in real time, ensuring its integrity and transparency.

---

## What is the Formula for Calculating the Burn Amount ?

The burn amount is primarily determined by the **base fee** and the gas used in the transaction. The formular is as follows:

```
Burned ETH = Base Fee * Gas Used
```

- **Base Fee**: The minimum amount of ETH per unit of gas that must be paid for a transaction to be included in a block. This fee adjusts dynamically based on network congestion.

- **Gas Used**: The total amount of gas consumed by the transaction (which depends on the transaction's complexity).

---

## Why is the Burn and Issuance Balance Important ?

The balance between **issuance** and **burn** is critical for Ethereum's long-term economic sustainability.

- **Preventing Inflation**: If issuance significantly outspaces burn, the total ETH supply will increase, potentially leading to inflation and a decrease in ETH's value.

- **Encouraging Deflation**: When burn exceeds issuance(which often occurs during periods of high network activity), the total supply of ETH decreases, creating a deflationary effect.

- **Incentivizing Validators**: Issuance ensures that validators re adequately rewarded for their work in securing the network. Meanwhile, burn aligns economic incentives by reducing excess supply.

This dynamic relationship between issuance and burn ensures that Ethereum's economic model remains robust and sustainable, promoting scarcity and maintaining the value of ETH over time.

---

## Formulas

- **Burn Formula**

```
Burned ETH = Base Fee * Gas Used
```

- **Net Supply Change**

```
Net Supply Change = Issuance - Burn
```

- **Validator Rewards**

```
Validator Rewards = Issuance + Priority Fees(Tips)
```

---

## What's the Specific Formula for Issuance?

Actually **issuance** in the Ethereum **Proof-of-Stake(Pos)** system can be though of as **new ETH minted by the network** to reward validators for securing the blockchain. It is a separate mechanism from **gas fees** and **burning**, and its value depends on the staking process and validator participation.

**Issuance = New ETH Rewards to Validators**

More spcificially:

- **Issuance = ETH rewarded for block proposals and attestations**
  Validators are rewarded for proposing blocks, attesting to blocks, and participating in consensus.

- The total issuance depends on the **total amount of ETH staked** in the Beacon Chain. As more ETH is staked, the overall annual percentage rate(APR) decreases because the rewards are distributed among more validators.

### Issuance Formula

Ethereum issuance is based on a **reward curve** that depends on the total staked ETH. The reward rate is defined by the following formular:

- **Annual Validator Issuance**

```
Annual Issuance (in ETH) = Base Reward Per Validator × Total Validators × Epochs Per Year
```

- **Base Reward Per Validator** is determined by

```
Base Reward = Effective Balance × Base Reward Factor ÷ SQRT(Total ETH Staked)
```

**Effective Balance**: Typically capped at 32 ETH per validator.
**Base Reward Factor**: A constant defined in Ethereum's PoS specification.
**Total ETH Staked**: Total amount of ETH staked across all validators.

```
// Net Supply Change: how many ETH in total on Beacon Chain
// Issuance: Ethereum's validator validate blocks(provide secure service) will trigger minted new ETHs
// Burned ETH: How many minted ETHs are destroyed with the aim of reducing total ETHs on Beacon Chain caused deflation(decrease of the value of ETHs which are allcoated to validators)
Net Supply Change = Issuance - Burned ETH
```
