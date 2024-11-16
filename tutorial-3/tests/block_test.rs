use blockchain::block::Block;

/*
Generate an initial value as bitcoin's mining difficulty.
*/
#[cfg(test)]
mod tests {
    use blockchain::{block::Block, hashtable::Hashtable};
    // use hex;

    fn gen_difficulty() -> u128 {
        return 0x0000000000000fff;
    }

    #[test]
    fn test_block_creation() {
        let block = Block::new(13, 0, vec![0; 32], 0, gen_difficulty());
        assert_eq!(block.index, 13);
        assert_eq!(block.timestamp, 0);
        assert_eq!(block.hash, vec![0; 32]);
    }

    #[test]
    fn test_block_bytes() {
        let block = Block::new(1, 1627836483, vec![0; 32], 12345, 2);
        let bytes = block.bytes();

        // bytes is converted from
        // index:u32 -> Vec<u8> * 4
        // timestamp:u128 -> Vec<u8> * 16
        // prev_block_hash: vec![0; 32] -> Vec<u8> * 32
        // nonce: u64 -> Vec<u8> * 8
        // difficulty: u128 -> Vec<u8> * 16
        assert_eq!(bytes.len(), 4 + 16 + 32 + 8 + 16);

        // use cargo test -- --nocapture to let println info print to console
        println!("Block inner info {:?}", block);
    }
}
