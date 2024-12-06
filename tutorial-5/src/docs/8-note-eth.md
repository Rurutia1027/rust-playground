# Notes for Ethereum Knowledge

## Ether(ETH)

- The native crptocurrency of Ethereum
- ETH often used as a high-level unit for user-friendly representation, e.g., 1 ETH.
- It is subdivisible into smaller units like _Gwei_ and _Wei_ for precision in smart contracts and transactions.

## Wei

- The smallest unit of Ether, analogous to "cents" in a dollar.
- **1 ETH = 10 ^ 18 Wei.**
- Using Wei ensures calculations in _smart contracts_ are previse and avoids floating-point arithmetic errors.

## Gwei

- _Gwei_ is a common intermediary unit used for gas fees in Ethereum
- **1 Gwei = 10 ^9 Wei, 1 ETH = 10 ^ 9 Gwei**

## Gas and Gas Prices:

- Gas represents the computational effort required to execute transactions or smart contracts on Ethereum.
- Gas prices are typically specified in Gwei and vary based on network demand.

## Conversions in Ethereum Development:

- Developers often need to switch between units depending on the use case.
- Gas calculations use Gwei, while contract balances may require Wei.
- User-facing applications present amounts in ETH for simplicity.
