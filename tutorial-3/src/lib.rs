// here we define the Hash and Address
type Hash = Vec<u8>;
type Address = String;

// Credit: https://stackoverflow.com/a/44378174/2773837
use std::time::{SystemTime, UNIX_EPOCH};

/*
This function will first retrieve current timestamp in seconds and milliseconds,
and then convert both of the values into the unsigned integer that with 128 bits.
*/
pub fn now() -> u128 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

/*
Function converts unsigned integer with 32 bits into an array with length 4 and types unsigned integer with 8 bits.
The maximum value of u8 is 255 which can be calculated in bianry 11111111 = 2^8 - 1 = 255,
and u32's max value can be calculated in the same way: 2^32 - 1.

Function u32_bytes receives parameters in type of immutable references, which means original u32's data ownership
will not hand over to the function, and the values of original data is immutable in the scope of the funciton.

About the implementation of this function, first from the function's signature we know that the return value
is an array with type of u8(unsigned integer with 8 bit) and with length 4.

In the function, it convert the lowest 8 bits in the given value u by 'as u8' and let the converted value stored in the array's 0-index location.
Then continue right moving the 16 bits, which means the first lowest 8 bits value of u will be discarded.
and the second lowest 8 bits will be converted into the unsigned integer 8 by 'as u8', then moving the 24 bits, 32 bits until all the 8 unit bits are
converted into the unsigned integer with 8 bits and stored inside the vector.
0x0 is hexadecimal value 0, 0x1 is hexadecimal value 1, 0x2 is hexadecimal value 2, 0x3 is hexadecimal value 3.



suppose u is value of 32 bits long,
11111111 -- [0, 7]     --> this can be retrieved by u >> (8 * 0 <0x0>) = 0 (means discard non bits)        -> (11111111) -> converted into u8 -> vec[0]
00000000 -- [8, 15]    --> this can be retrieved by u >> (8 * 1 <0x1>) = 8 (means discard lowest 8 bits)   -> (00000000) | (11111111) -> (00000000) -> converted into u8 -> vec[1]
11111111 -- [16, 23]   --> this can be retrieved by u >> (8 * 2 <0x2>) = 16 (means discard lowest 16 bits) -> (11111111) | (00000000) (11111111) -> (11111111) -> converted into u8 -> vec[2]
00000000 -- [24, 32]   --> this can be retrieved by u >> (8 * 3 <0x3>) = 24 (means discard lowest 24 bits) -> (00000000) | (11111111) (00000000) (11111111) -> (00000000) -> vec[3]
*/
pub fn u32_bytes(u: &u32) -> [u8; 4] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
    ]
}

/*
Function converted unsigned integer with 64 bits into array with length 8
and with type unsigned integer with 8 bits.
*/
pub fn u64_bytes(u: &u64) -> [u8; 8] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
    ]
}

/*
Function converted unsigned integer with 128 bits into array with length 16
and with type unsigned integer with 8 bits.
*/
pub fn u128_bytes(u: &u128) -> [u8; 16] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
        (u >> 8 * 0x8) as u8,
        (u >> 8 * 0x9) as u8,
        (u >> 8 * 0xa) as u8,
        (u >> 8 * 0xb) as u8,
        (u >> 8 * 0xc) as u8,
        (u >> 8 * 0xd) as u8,
        (u >> 8 * 0xe) as u8,
        (u >> 8 * 0xf) as u8,
    ]
}

// declare block, blockchain, hashtable and transacitons as modules in the scope of the project
pub mod block;
pub mod blockchain;
pub mod hashtable;
pub mod transactions;
