# Transactions in Blockchain 
As a freshman who interested in both Rust and Blockchain, I have a serious questions and confusions about them. So I record all the question I met during leanring, and take a note for the questions and the answers from both online documents or GPT's in this note. 

## Q: Types of Transactions 
### Coinbase Transactions  
* Purpose: Introduces new coins into the system as a reward for mining and validating a block.
* Inputs(Spending): None - there are no previous transactions being spent. 
* Outputs(Receiving):  Transfers the newly minted coins to the miner's address. 

### Regular Transactions 
* Purpose: Move existing coins between addresses by spending outputs from previous transactions. 
* Inputs(Spending): Refers to outputs of previous transactions. 
* Outputs(Receiving): Transafers the value to new addresses, and there are different addresses in the record of outputs, so it is described as 're-distribute' like 'shuffle' between mappers and reducers.

### Summary: 
* Coinbase transactions: those are unique because they create new coins and have no inputs. The recipients of these coins are typically miners(or validators), who earn these rewards by contributing computational and storage resources to the blockchain network through mining or validation. 
* Regular transactions: those are redistribute existing coins, and the inputs outputs in such transactions require to be valided.
* Coinbase transaction is created each time a new block is mined.


## Q: Logic for Calculating Values in a Transactions
### What should be verified?
#### 1 Verify both Inputs and Outputs should be correctly referenced. 
* Ensure that the inputs correctly reference outputs from previous transactions. 
* Ensure that the total value of the inputs is `>=` to the total value of the outpus, which receiving(outputs) accmulated value should always `>=` than spending(intpus) accumulated value.
* After calculating the `sum(output#values) - sum(input#values)` the result we call it as  `surplus(fee)`, this is usally sent to the miner as a transaction fee.   

#### 2 Verify calculated value from both Inputs' records and Output's records. 

#### 3. Input and Output to be calculated records match conditions. 


