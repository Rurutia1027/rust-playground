// - Combinators

use std::result;

fn main() {
    let words = vec!["apple", "banana", "grape", "orange", "pear"];
    let mut ret: Vec<String> = vec![];

    for word in words.clone() {
        if word.starts_with("a") || word.starts_with("b") {
            let uppercase_word = word.to_uppercase();
            ret.push(uppercase_word);
        }

        println!("result: {:?}", ret);
    }

    // here is another solution
    let ret = words
        .clone()
        .into_iter()
        .filter(|&word| word.starts_with("a") || word.starts_with("b"))
        .map(|item| item.to_uppercase())
        .collect::<Vec<String>>();

    println!("ret value : {:?}", ret);
}
