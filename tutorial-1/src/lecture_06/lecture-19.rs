// Lecture: Privacy in Modules

// we need to let the library be public then we can import those libs in this way:
use tutorial_1::{Category, Customer, Order, Product};

fn main() {
    let product = Product::new(
        1,
        String::from("product_name"),
        8.3,
        Category::Books,
    );

    let customer = Customer::new(
        1,
        String::from("Alice"),
        String::from("alc@33.com"),
    );

    let order = Order::new(1, product, customer, 2);
    println!("Total cost of the order: ${}", order.total_bill());
}
