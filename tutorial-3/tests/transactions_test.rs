#[cfg(test)]
mod tests {
    use blockchain::{
        hashtable::Hashtable,
        transactions::{Output, Transaction},
    };
    use rand::Rng;
    use uuid::Uuid;

    fn generate_random_output() -> Output {
        let random_uuid = Uuid::new_v4();
        let mut rng = rand::thread_rng();
        let random_to_addr = random_uuid.to_string();
        let random_value: u16 = rng.gen();

        Output::new(
            random_to_addr.to_owned(),
            random_value as u64,
        )
    }

    #[test]
    fn test_transaction_create() {
        let inputs: Vec<Output> = vec![];
        let outputs: Vec<Output> = vec![];
        let trans: Transaction =
            Transaction::new(inputs, outputs);

        println!("Transaction Debug Info: {:?}", trans);

        let mut bytes: Vec<u8> = vec![];

        // so, here even though we pass two empty vectors of inputs and outputs
        // the bytes and the hash funciton invokcation will be executed as expected.
        let trans_bytes = trans.bytes();
        assert_eq!(bytes, trans_bytes);

        // then let continue verify the hash works as expected: that even though the
        // inputs & outputs vectors are empty, the hash() function will not throw any exception or errors
        let mut hash = crypto_hash::digest(
            crypto_hash::Algorithm::SHA256,
            &bytes,
        );

        let trans_hash = trans.hash();
        assert_eq!(hash, trans_hash);
    }

    #[test]
    fn test_transaction_create_with_inputs() {
        // here let create a vector of input with len = 10
        let mut inputs: Vec<Output> = vec![];
        let outputs: Vec<Output> = vec![];

        for i in 0..10 {
            let input = generate_random_output();
            inputs.push(input);
        }

        let trans: Transaction =
            Transaction::new(inputs, outputs);

        let mut bytes: Vec<u8> = vec![];

        // append transaction's inner variables to the bytes
        trans
            .inputs
            .iter()
            .flat_map(|item| item.bytes())
            .for_each(|item| bytes.push(item));

        trans
            .outputs
            .iter()
            .flat_map(|item| item.bytes())
            .for_each(|item| bytes.push(item));

        assert!(bytes.len() > 0);
        // 440
        println!("length of outer bytes is {}", bytes.len());

        // let's convert the outer's bytes into the hash value
        // and continue convert it into hex format
        let mut hash_str = hex::encode(crypto_hash::digest(
            crypto_hash::Algorithm::SHA256,
            &bytes,
        ));
        println!("outer bytes hash in hex {:?}", hash_str);

        // here directly invoke inner function to get hash value,
        // and then continue convert it into hex format
        let mut trans_hash_str = hex::encode(&trans.hash());

        println!(
            "trans bytes hash in hex {:?}, \n Transaction[{:?}]",
            trans_hash_str, trans
        );

        // so, here the outer hash value in hex should match with the trans's inner hash value return in hex
        assert_eq!(trans_hash_str, hash_str);
    }

    #[test]
    fn test_transaction_create_with_inputs_outputs() {
        let mut inputs: Vec<Output> = vec![];
        let mut outputs: Vec<Output> = vec![];

        // generate 10 input & output items and push those to inputs vector and outputs vector
        for i in 0..10 {
            let input = generate_random_output();
            let output = generate_random_output();

            inputs.push(input);
            outputs.push(output);
        }

        let trans: Transaction =
            Transaction::new(inputs, outputs);

        // continue our logic, create outsider bytes and hash
        // and compare the outer bytes & hash with the trans function return value
        let mut bytes: Vec<u8> = vec![];

        // append each items in inputs
        trans
            .inputs
            .iter()
            .flat_map(|item| item.bytes())
            .for_each(|item| bytes.push(item));

        // append each items in outputs
        trans
            .outputs
            .iter()
            .flat_map(|item| item.bytes())
            .for_each(|item| bytes.push(item));

        assert!(bytes.len() > 0);
        println!("length of outer bytes is {}", bytes.len());
        assert_eq!(bytes, trans.bytes());

        let mut hash_str = hex::encode(crypto_hash::digest(
            crypto_hash::Algorithm::SHA256,
            &bytes,
        ));

        let mut trans_hash_str = hex::encode(trans.hash());

        // this should be match
        assert_eq!(hash_str, trans_hash_str);

        // here we continue validate the total value of each inputs and outputs
        let input_total_value = trans.input_total_value();
        let outer_input_total_value: u64 =
            trans.inputs.iter().map(|item| item.value).sum();

        let output_total_value = trans.output_total_value();

        let outer_output_total_value: u64 =
            trans.outputs.iter().map(|item| item.value).sum();

        // asser the total value of inner calcuated and outer calculated should be the same
        assert_eq!(outer_input_total_value, input_total_value);
        assert_eq!(outer_output_total_value, output_total_value);

        // here we validate the input_hashes and output_hashes
        let input_hash_set = trans.input_hashes();
        let output_hash_set = trans.output_hashes();
        // since elements in inputs many converted to multiple items in hash set

        assert!(input_hash_set.len() >= trans.inputs.len());
        assert!(output_hash_set.len() >= trans.outputs.len());
    }
}
