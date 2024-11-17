#[cfg(test)]
mod tests {
    use std::ptr::addr_eq;

    use blockchain::{
        hashtable::Hashtable,
        transactions::Output, u64_bytes,
    };

    #[test]
    fn test_output_creation() {
        let output = Output::new(
            "target_address".to_owned(),
            40,
        );

        assert_eq!(
            output.to_addr,
            "target_address"
        );
        assert_eq!(output.value, 40)
    }

    #[test]
    fn test_output_bytes() {
        let output = Output::new(
            "to_addr".to_owned(),
            128,
        );

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

    #[test]
    fn test_output_hash() {}

    #[test]
    fn test_output_hash_hex() {}
}
