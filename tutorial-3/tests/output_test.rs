#[cfg(test)]
mod tests {
    use std::ptr::addr_eq;

    use blockchain::{hashtable::Hashtable, transactions::Output, u64_bytes};

    #[test]
    fn test_output_creation() {
        let output = Output::new("target_address".to_owned(), 40);

        assert_eq!(output.to_addr, "target_address");
        assert_eq!(output.value, 40)
    }

    #[test]
    fn test_output_bytes() {
        let output = Output::new("to_addr".to_owned(), 128);

        assert_eq!("to_addr", output.to_addr);
        assert_eq!(output.value, 128);
        let hash_value = output.bytes();
        assert!(hash_value.len() > 0);

        // here we re-generate the bytes in the output and compare it
        // to the output's inner bytes func return value, this should be match
        let mut bytes = vec![];
        bytes.extend(output.to_addr.bytes());
        bytes.extend(&u64_bytes(&output.value));

        assert_eq!(bytes, output.bytes());
    }

    // in this test we create instance of output
    // and create bytes based on the inner variables of the instance of output
    // and adopt Hashtable's hash function converted it into hex value
    // and compare the outside hex hash value compare with the
    // output's inner function return hash value and they should match with each other
    #[test]
    fn test_output_hash_verify() {
        let output = Output::new("193.33.12.34".to_owned(), 400);
        // first, append all variables to current mutable byte array
        let mut bytes = vec![];
        bytes.extend(output.to_addr.bytes());
        bytes.extend(&u64_bytes(&output.value));
        assert_eq!(bytes, output.bytes());

        // second, convert the byte array into hash by invoking the hash funciton which
        // is already implemented in trait of Hashtable
        let hash_value =
            crypto_hash::digest(crypto_hash::Algorithm::SHA256, &bytes);

        // third, assert those values equal to each other
        assert_eq!(hash_value, output.hash());
    }
}
