// Itearting Through Collections

use std::collections::{btree_map::Keys, HashMap};

fn main() {
    // we can convert the collection into iterator and then use the iterator to traverse the element in the collection
    let vec = vec![1, 2, 3, 4, 6, 78, 9, 4, 7, 9, 4];

    // #iter() this gives an iterator of immutable item references
    // #iter_mut() this gives an iterator of mutable item references
    // #into_iter this returns an iteartor of values without any references
    let iter1 = vec.iter();
    let iter2 = vec.clone().into_iter();

    for item in iter1 {
        println!("value of item is {:?}", item);
    }

    println!("------");

    for item in iter2 {
        println!("value of item is {:?}", item);
    }

    // hash map
    let mut person: HashMap<String, i32> = HashMap::new();
    person.insert("name".to_string(), 23);
    person.insert("s3 ".to_string(), 23);
    person.insert("de".to_string(), 23);
    person.insert("333".to_string(), 23);

    println!("k: {:?}", person);

    // we often use the refrences to traverse the iteartor of hash map
    for (k, v) in &person {
        println!("key: {:?}, value: {:?}", k, v);
    }

    // if we traverse the person directly, the value of the person will be owned by the k and v
    // we nolonger can get access out scope of the traverse
    for (k, v) in person {
        println!("key {:?}, value {:?}", k, v);
    }

    //     println!("k: {:?}", person);
}
