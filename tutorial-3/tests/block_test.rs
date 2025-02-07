#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Ok, Result};
    use blockchain::{
        block::Block,
        hashtable::Hashtable,
        now,
        transactions::{Output, Transaction},
    };
    use crypto_hash::hex_digest;
    use rand::Rng;
    use uuid::Uuid;
    /*
    Generate an initial value as bitcoin's mining difficulty.
    */
    fn gen_difficulty() -> u128 {
        return 0x0000000000000fff;
    }

    #[test]
    fn test_block_creation() {
        // first, we need to init a series of transaction
        let trans = gen_random_transactions(9).unwrap();
        let block = Block::new(13, 0, vec![0; 32], trans, 0, gen_difficulty());
        assert_eq!(block.index, 13);
        assert_eq!(block.timestamp, 0);
        assert_eq!(block.hash, vec![0; 32]);
    }

    #[test]
    fn test_block_bytes() {
        let trans: Vec<Transaction> = gen_random_transactions(10).unwrap();
        let block = Block::new(1, 1627836483, vec![0; 32], trans, 12345, 2);
        let bytes = block.bytes();

        // bytes is converted from
        // index:u32 -> Vec<u8> * 4
        // timestamp:u128 -> Vec<u8> * 16
        // prev_block_hash: vec![0; 32] -> Vec<u8> * 32
        // nonce: u64 -> Vec<u8> * 8
        // difficulty: u128 -> Vec<u8> * 16
        // here should be modify becase we append trans to block
        assert!(bytes.len() > 4 + 16 + 32 + 8 + 16);

        // use cargo test -- --nocapture to let println info print to console
        println!("Block inner info {:?}", block);
    }

    /*
    In this test case, we create a new instance of Block,
    and invoke it's hash functions to let the generated bytes converted into the crypto hash value.
    */
    #[test]
    fn test_block_hash() {
        // first first, create a Transaction vector
        let trans: Vec<Transaction> = gen_random_transactions(10).unwrap();
        // first, create a new Block
        let block = Block::new(22, now(), vec![0; 32], trans, 1234, 3);
        let bytes = block.bytes();

        // because we now appand trans to block so modify this from '=' into '>'
        assert!(bytes.len() > 4 + 16 + 32 + 8 + 16);
        let hash_val = block.hash();
        println!("hash value of the block is {:?}", hash_val);

        // and let's convert the Vec<u8> into hex
        println!("hex hash value of the block is {:?}", hex::encode(hash_val))
    }

    fn gen_random_output() -> Result<Output> {
        let random_uuid = Uuid::new_v4();
        let mut rng = rand::thread_rng();
        let random_to_addr = random_uuid.to_string();

        // here we use u16 in case of all value accumulated overflow
        let random_value: u16 = rng.gen();

        let ret =
            Ok(Output::new(random_to_addr.to_owned(), random_value as u64));
        ret
    }

    fn gen_random_outputs(cnt: u8) -> Result<Vec<Output>> {
        let mut ret = vec![];
        assert!(cnt > 0, "cnt should be >= 0!");

        for i in 0..cnt {
            ret.push(gen_random_output().unwrap());
        }

        Ok(ret)
    }

    fn gen_random_transaction() -> Result<Transaction> {
        let mut rng = rand::thread_rng();
        let mut ret: Transaction;

        let input_cnt_random = rng.gen_range(1..=10);
        let output_cnt_random = rng.gen_range(1..=10);

        let inputs = gen_random_outputs(input_cnt_random).unwrap();
        assert!(inputs.len() > 0, "inputs items generate failed!");
        let outputs = gen_random_outputs(output_cnt_random).unwrap();
        assert!(outputs.len() > 0, "outputs items generate failed!");

        ret = Transaction::new(inputs, outputs);

        Ok(ret)
    }

    fn gen_random_transactions(cnt: u8) -> Result<Vec<Transaction>> {
        let mut ret: Vec<Transaction> = vec![];

        assert!(cnt > 0, "recv cnt value should be >= 0!");

        for i in 0..cnt {
            ret.push(gen_random_transaction().unwrap());
        }

        Ok(ret)
    }

    // todo: here add more test cases for block's transactions here
    #[test]
    fn test_trans_in_block() {
        // create block with 21 transactions in it
        let trans = gen_random_transactions(21).unwrap();
        assert!(trans.len() == 21, "trans count should be match!");

        let block =
            Block::new(1, now(), vec![0; 32], trans, 12345, gen_difficulty());

        // here we print inner debug info of the block we just created
        println!("Block info {:?}", block);

        // here we invoke blocks bytes(Hashtable#bytes) functions
        // to check after add transactions to the block the bytes can be created correctly
        let bytes = block.bytes();
        assert!(bytes.len() > 0, "byte array length should > 0");

        // here invoke hash function
        let outer_hash_str = hex::encode(crypto_hash::digest(
            crypto_hash::Algorithm::SHA256,
            &bytes,
        ));

        let inner_hash_str = hex::encode(block.hash());

        assert_eq!(outer_hash_str, inner_hash_str);
    }
}
