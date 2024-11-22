use testing::{Category, Customer, Order, Product};
#[test]
fn test_total_bill_without_discount() {
    let product = Product::new();
    let customer = Customer::new();
    let order = Order::new();

    assert_eq!(format!("{:.2}", order.total_bill()), "65.67");
}
