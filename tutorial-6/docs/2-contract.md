# Smart Contract Notes

## EtherScan API EndPoints

---

## Understand What's ABI ?

The **Application Binary Interface(ABI)** is a standarized interface that allows smart contracts to interact with other smart contracts and external applications(off-line apps) on the Ethereum blockchain. It defines how functions, data types, and structure within the contract are encoded/decoded and allows for communication between **Ethereum Virtual Machine(EVM)** and external applicaitons, such as wallets or decentralized applications(dApps).

When you call a function in a smart contract, the **ABI** ensures that the parameters are correctly encoded when sending the transaction and properly decoded when the transaction is processed.

---

## Understand What's Smart Contract?

A **smart contract** is a self-executing contract with the terms of the agreement between buyer and seller directly written into code. Smart contracts run on a blockchain(like Ethereum) and automatically enfoce the terms and conditions without needing intermediaries.

Smart contracts are typically written in languages like `Solidity`(for Ethereum) and are deployed on the blockchain. They(**Smart Contracts**) consist of:

**Code**: that defiens the logic of the contract.
**State**: that represents the contract's data(balances, records, etc.)
**Event**: that are emitted when specific actions or conditions are triggered(help to trace the transactions and tokens).

The blockchain ensures that once a smart contract is deployed, it cannot be altered, providing transparency and immutability.

## Relationship between ABI and Smart Contract?

The ABI is the interface between the smart contract(on-chain) and the outside world(off-chain). It allows external(off-chain) entities(like dApps or wallets) to interact with the smart contract(on-chain) on the blockchain. The ABI is generated based on the smart contract's code and functions, and it is needed to encode and decode functions calls and responses between a client(like a web browser or mobile app) and the smart contract.

```
The ABI is similar to Protobuf in that both define a standardized way to encode and decode data for communication. However, while Protobuf is typically used for defining the structure of messages exchanged between client and server(including the server's address and port), the ABI hides this network-level detail and instead of focuses on the interface for interacting with smart contracts.  The ABI defines the function signature, parameters, and return types, ensuring that both the contract and external applications follow a uniform structure for calling and receiving data. In both cases, the goal is to ensure that messages are consistently formatted and understood on both ends of the communication.
```

---

## Suppose I Wanna Implement a Buying StarBuck's Smart Contract What Steps Should I Do?

### How to code with Solidity that Connect to the Etherum?

Solidity is the primary language used for writing smart contracts on the Ethereum blockchain. It's a statically-typed programming language for creating decentralized applications(dApps) and smart contracts.

We can write a smart contract that accepts payments(in ETH or a token) and allows users to purchase products, like a Starbucks coffee, from it. Here is a outline of steps for writing the contact:

#### Install and Set Up the Dev Environment:

- Use **Remix IDE(a browser-based tool)** or setup **Truffle/Hardhat**(local development environments) for building and deploying contracts.

- Install **MetaMask** to interact with Ethereum and test transactions.

#### Sample Smart Contract(Solidity Code):

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract StarbucksPurchase {
    address public owner;
    uint256 public coffeePrice; // Price's unit is wei(the smallets unit in Ethereum, 1 ETH = 10^18 wei = 10^9 Gwei)

    event CoffeePurchased(address indexed buyer, uint256 amount);

    constructor(uint256 _coffeePrice) {
        // Set contract deployer as the owner
        owner = msg.sender;

        // Set the price of one coffee
        coffeePrice = _coffeePrice;
    }

    // Function to purchase coffee
    function purchaseCoffee() external payable {
        require(msg.value == coffeePrice, "Incorrect ETH amount sent");

        // Transfer ETH to the owner (Starbucks in this case)
        payable(owner).transfer(msg.value);

        // Emit event for logging the purchase
        emit CoffeePurchased(msg.sender, msg.value);
    }

    // Function to update coffee price (only owner can call this function)
    function updateCoffeePrice(uint256 newPrice) external {
        require(msg.sender == owner, "Only owner can update Coffee New Price");
        coffeePrice = newPrice;
    }

    // Withdraw contract's balance (only owner can withdraw)
    function withdraw() external {
        require(msg.sender == owner, "Only owner can withdraw");

        // transfer the received buy coffee amount of ETH(wei unit)
        // to owner's self wallet balance
        payable(owner).transfer(address(this).balance);
    }
}
```

### How to implement the inner logic?

The inner logic descripted below:
**Purchase Coffee**: Users call `purchaseCoffee()` this function to send the exact ETH amount required, the amount of ETH is regared as the validate condition to verify a purchase is valid.

**Price Validation**: The contract checks if the sent value equals the set coffee price(defined in the scope of contract).

**Transfer to Owner**: After validation the amount of ETH, the ETH is transferred to the owner's address.

**Event Logging**: When a purchase is made(transfer is ok and finished), it(the smart contract) emits(broadcast an event to the scope of while blockchain) for transparency and audit tracing.

**Owner Control**: The contract allows only the owner to update and price and withdraw the funds(received amount of ETH).

### How to purchase the first token via the Smart Contract I wrote ?

To make this first purchase, you'll need a few preparations:
**ETH in your wallet**: You can get **testnet ETH** from a faucet if you're deploying on a test network like **Rinkeby** or **Ropsten**.

**MetaMask**: Install MetaMask as a browser extensions to interact with the contract.

**Deploy the Contract**: Use **Remix IDE**, **Truffle** or **Hardhat** to deploy your contract on Ethereum.

**Purchase Token**: Interact with your contract through **MetaMask** or any Ethereum wallet interface: Call the `purchaseCoffee()` function, sending the reuiqred ETH amount.

### How to deploy the Smart Contract onto the Ethereum via something like K8S or DevOps Tools and Ask others to use it ?

To deploy the smart contract onto Ethereum, follow these steps:

**Using Remis**:

- **Write and Compile**: Write the contract in **Remix IDE** and compile it.

- **Deploy**: Connect **Remix** to your Ethereum wallet(e.g., MetaMask) and deploy to the Ethereum mainnet or a test network.

- **Interaction**: After deployment, you can share the contract address for others to interact with it.

**Using Hardhat or Truffle for More Control**:

- **Set Up Hardhat/Truffle Project**: Use Hardhat or Truffle to create a project.
- **Write Tests**: Write automated tests for you contract.
- **Deploy Script**: Write a deploy script that deploys the contract to Ethereum.

```javascript
async function main() {
  const [deployer] = await ethers.getSigners();
  console.log();

  const Starbucks = await ethers.getContractFactory("StarbucksPurchase");

  // this is an example of 0.05 ETH
  const coffeePrice = ethers.utils.parseEther("0.05");

  const starbucks = await Starbucks.deploy(coffeePrice);

  console.log("Starbucks contract deployed to : ", starbucks.address);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
```

**Deploy with DevOps**

- **CI/CD**: We can automate deployment using tools like GitHub Actions, Jenkins or GitLab CI.
- **Kubernetes(K8S)**: For smart contract interaction, we do not deploy the smart contract to K8S directly. However, we can deploy an applicaiton as backend service that interacts with Ethereum, or set up monitoring/management tools in K8S.

### Will Deploy My Own Smart Contract Require Gas Fee and How to Pay for that?

Yes, deploying a smart contract requires gas fees on Ethereum network. Gas is the computation cost of executing transactions, including contract deployment.

**Paying for Gas**

- **Mainnet Gas**: We will need ETH to pay for the deployment on Ethereum's mainnet.
- **Test Networks**: If deploying on testnets(like **Rinkeby**, **Ropsten**, or **Goerli**), we can get free testnet ETH from faucets to pay for gas.
- **Paying Gas**: When deploying via **Remix, Truffle, or Hardhat**, our Ethereum wallet (e.g., MetaMask) will ask us to approve the gas fee transaction.

Gas fee are calculated based on the complexity of the contract and network demand, so they can vary. Ethereum's gas fees are paid using ETH, and our Ethereum wallet handles the payment during deployment.

### Conclusion

In summary:

- **Write the contract in Solidity**: focusing on purchase logic, event handling, and price updates.

- **Deploy the contract**: using Remix, Hardhat, or Truffle, and pay gas fees to deploy it on Ethereum.

- **Interact with the contract**: through MetaMask or other Ethereum wallet interfaces.

- **Gas Fees**: that are required for deployment and transactions on Ethereum, and they are paid with ETH.
