use crate::customer;
use crate::customer::*;
use crate::product;
use crate::product::*;
pub struct Order {
    id: u64,
    // here we use crate to retrieve the product module and import its Product
    product: Product,
    customer: Customer,
    quantity: u32,
}

impl Order {
    pub fn new(
        id: u64,
        product: Product,
        customer: Customer,
        quantity: u32,
    ) -> Self {
        Order {
            id,
            product,
            customer,
            quantity,
        }
    }

    pub fn calculate_discount(&self) -> f64 {
        println!("discount got quantity: {}", self.quantity);

        if self.quantity > 5 {
            0.5
        } else {
            0.2
        }
    }

    pub fn total_bill(&self) -> f64 {
        let discount = self.calculate_discount();
        println!(
            "discount:{}, product_price: {}",
            discount,
            self.product.product_price()
        );
        let total_before_discount =
            self.product.product_price() * self.quantity as f64;
        total_before_discount - (total_before_discount * discount)
    }
}
