// here we can use use category::Category; to shrink the code's representaiton
// use category::Category;
pub use category::Category;

#[derive(PartialEq, Debug)]
pub struct Product {
    id: u64,
    name: String,
    price: f64,
    category: Category,
}

// here in rust, a child module can get access to all structs and functions that define in parent modules
// but, parent cannot get access to private child modules' function or structs
// so we need to set enum Category the child module public so that it can be used in mod product correctly
pub mod category;

impl Product {
    pub fn new(
        id: u64,
        name: String,
        price: f64,
        category: Category,
    ) -> Self {
        Product {
            id,
            name,
            price,
            category,
        }
    }

    fn calculate_tax(&self) -> f64 {
        self.price * 0.1
    }

    pub fn product_price(&self) -> f64 {
        self.price + self.calculate_tax() as f64
    }
}
