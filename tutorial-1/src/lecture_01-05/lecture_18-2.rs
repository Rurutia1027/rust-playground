// Hash Map

use std::collections::{btree_map::Keys, HashMap};

fn main() {
    let some_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 8, 7, 6, 5];
    let mut fre_vec: HashMap<i32, u32> = HashMap::new();

    // here we use &some_vec to iterate values in it
    // but not hand over it's ownership to the for loop
    for i in &some_vec {
        // i is a reference, we if we need to get its value we need to dereferencing by add '*' as prefix
        // it just liek the java hashmap's getOrDefault( here set a default return value if the key mapping to None)
        let freq: &mut u32 = fre_vec.entry(*i).or_insert(0);

        // here we use the let freq: &mut u32 already point to the heap data of the map's value space
        // here we just use *freq is to get its space and add it to 1 direcly updated to the heap space
        // so we don't need execute fre_vec.insert(..., ...)

        // this update operation will directly update the +1 value to the heap space
        *freq += 1;
    }

    println!("HashMap content: {:?}", fre_vec);
}
