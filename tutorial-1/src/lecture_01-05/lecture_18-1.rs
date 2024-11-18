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

    if person.contains_key("Nouman") {
        println!("The value exist");
    } else {
        println!("The value does not exist");
    }

    // use pattern match to repalce if/else logic
    match person.get("Nouman") {
        // here the Some(...) should pass a variable if we need to use it in the pattern match
        // expression
        Some(value) => {
            println!("The value of Norman is {} it is exist", value)
        }
        None => println!("The value of Nouman does not exist!"),
    }

    // here is another way to traverse entries in hash map
    for (name, age) in person {
        println!("The person {} has an age of {}", name, age);
    }

    // another case for hash map
    let mut likes: HashMap<&str, &str> = HashMap::new();
    likes.insert("mae", "video games");
    // the second operation will overwrite the value
    likes.insert("mae", "meat");
    println!("likes hash table is {:?}", likes);

    //  here we use this expression to do the following things
    // first check whether the key's has a correspoinding value exists in the HashMap
    // if not, then insert the correspoinding value to the map
    likes.entry("key").or_insert("steam");
    likes.entry("key").or_insert("ps5");

    // then print the value, then, this time the second insert operation will not be overwritten an already exist value
    println!("likes hash map {:?}", likes);
}
