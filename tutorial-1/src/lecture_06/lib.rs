// libray crate often provides basic services, like:
// adding products, maintaining customer data and processing order
// and we want products, customers and orders separated to different modules

// this cannot be used, becaues the product is not public , even though its inner module category is public
// but out of the scope of the product, it still unavailable

// pub + use module::items can let those items be public in the scope of the project,
// because it is lib.rs shared across the project
pub use customer::Customer;
pub use order::Order;
pub use product::category::Category;
pub use product::Product;

/// use crate::product::category;
/// Struct for storing product related information. 
mod product;

// here we also need to know, make a module public
// it's inner variables like structs or functiosn will not become public automatically and iteratively
mod customer;

mod order;
