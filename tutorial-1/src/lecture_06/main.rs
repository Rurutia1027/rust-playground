// External Dependencies

use array_tool::vec::*;
use tutorial_1::{Category, Customer, Order, Product};
fn main() {
    let prod1 = Product::new(
        1,
        "name1".to_string(),
        10.1,
        Category::Books,
    );
    let prod2 = Product::new(
        2,
        "name2".to_string(),
        10.2,
        Category::Clothing,
    );
    let prod3 = Product::new(
        3,
        "name3".to_string(),
        10.3,
        Category::Electronics,
    );

    let set1: Vec<&Product> = vec![&prod1, &prod2];
    let set2: Vec<&Product> = vec![&prod2, &prod3];
    let intersection = set1.intersect(set2);
    println!(
        "The intersection len: {}, and it's content: {:?}",
        intersection.len(),
        intersection
    );
}
