// Hash Maps
use std::collections::HashMap;

fn main() {
    let mut person: HashMap<&str, i32> = HashMap::new();
    // here insert data to hash map
    person.insert("Nouman", 40);

    person.insert("Kamran", 39);
    person.insert("Shahid", 55);

    // the get function is used to retrieve some specific entry
    println!(
        "The age of Nouman is {}",
        person.get("Nouman").unwrap()
    );
}
