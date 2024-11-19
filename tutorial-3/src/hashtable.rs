/*
Define the Hashtable trait, which provides an interface for hashing functionality.

This trait includes two functions:
1. `bytes`: This function must be implemented by any struct that uses this trait.
    It defines how the struct's internal data(variables or objects) will be serialized into a vector of bytes.

2.`hash`: This function provides a default implementation for computing the hash of the serialized byte data.
    It uses the SHA-256 algorithm form the `crypto_hash` library to produce a cryptographic hash of the output
    from the `bytes` function. This ensures that the hash computation is consistent across implementaiton.
*/
pub trait Hashtable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Vec<u8> {
        crypto_hash::digest(
            crypto_hash::Algorithm::SHA256,
            &self.bytes(),
        )
    }
}
