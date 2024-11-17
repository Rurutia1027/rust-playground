/*
Definition of the Blocks.
*/

use std::fmt::{self, Debug, Formatter};

use crate::{
    hashtable::Hashtable,
    transactions::{self, Transaction},
    u128_bytes, u32_bytes, u64_bytes, Hash,
};

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub difficulty: u128,
}

/*
implement inner function fmt which declared in the Debug trait in the Block context
by passing self reference and inner instance's vairable values can be retrieved via the reference.
*/
impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}]: {} at: {}, trans cnt: {}, nonce: {}, difficulty: {}",
            &self.index,
            hex::encode(&self.hash),
            &self.timestamp,
            &self.transactions.len(),
            &self.nonce,
            &self.difficulty
        )
    }
}

/*
We declare the functions that belongs to the scope of struct Block by
impl Block { ... declarations of Block's scoped functions ... }
*/
impl Block {
    /**
    Funciton new is just the name of the function which is different from the key word
    defined in the C++ or Java.
    Function new provides an interface that allows users to pass parameters to the function,
    and in the function passing the parameters to create a instance of struct of Block
     */
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: Vec<Transaction>,
        nonce: u64,
        difficulty: u128,
    ) -> Self {
        // here we hand the received parameters to struct
        Block {
            index,
            timestamp,
            // Block's init hash value is an empty array with type u8 and with length 32
            hash: vec![0; 32],
            prev_block_hash,
            transactions,
            nonce,
            difficulty,
        }
    }

    /*
    Function mine is trying to mimic the process of mining a block-coin from the blockchain
    */
    pub fn mine(&mut self) {
        todo!("add more details after we finish the transaction logic in Block")
    }

    pub fn trans_hash(&mut self) -> Vec<u8> {
        self.transactions
            .iter()
            .flat_map(|item| item.hash())
            .collect::<Vec<u8>>()
    }
}

/*
Let Block implement trait Hashtable and provide the implementation details of the function bytes.
*/
impl Hashtable for Block {
    fn bytes(&self) -> Vec<u8> {
        // first declare the return value: a mutable empty Vector with type of u8
        let mut bytes = vec![];

        // first, append Block's inner index to the Vec<u8>
        // and in Block we declare the index as tyep of u32,
        // so we call u32_bytes function to covnert the u32 into Vec<u8>

        // and there is something we need to notice here: is &u32_bytes(...) as a freshman to Rust
        // why we use the '&' here it is because we use the strategy of 'Borrowing' in Rust
        // which means that we pass the return value's from the u32_bytes(...) reference to the bytes#extend(...) funciton,
        // instead if we call bytes#extend(u32_bytes(...)) it means we hand over the return value's ownership
        // to the bytes#extend(...) this function, the value's heap space we pass to the extend will be released and come to invalid
        // and cannot be accessible when get out of the scope of the bytes#extend function
        // to avoid this, we use &u32_bytes(...) to only passing reference and triggering the 'borrowing' strategy.
        bytes.extend(&u32_bytes(&self.index));

        // second, we contine append Block's timestamp to the bytes array
        // however, timestamp is in type of u128 so we call u128_bytes function to convert it
        // and we also need to maintain the borrow strategy in Rust, so we need to use &u128_bytes(...)
        bytes.extend(&u128_bytes(&self.timestamp));

        // third, continue append Block's prev_block_hash to bytes array
        // and in lib.rs we already alias prev_block_hash: Hash as the type of Vec<u8>
        // so directly append is ok, however, again, passing the reference instead of the value directly
        // in case of prev_block_hash's ownership modificaiton caused data invalid error
        bytes.extend(&self.prev_block_hash);

        // fourth, continue append Block's nonce value to the bytes array
        bytes.extend(&u64_bytes(&self.nonce));

        // fifth, iterate current block's transaction vector
        // and extract each item's bytes append to the bytes
        // (and this bytes will feed hash function as to be hashed data)
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|item| item.bytes())
                .collect::<Vec<u8>>(),
        );

        // last, we append the inner difficulty
        // todo: [Can seder help us ???] and I need to leave a question here: convert all the items into bytes is the process of serialize
        // can use use the seder library to help us finish this step ? instead of convert all items in Block by our own ?
        bytes.extend(&u128_bytes(&self.difficulty));

        // finally, we return our bytes array and hand over the data to crypto which already
        // implemented in the function `hash`
        bytes
    }
}
