// here we also need to know, make a module public
// it's inner variables like structs or functiosn will not become public automatically and iteratively
pub struct Customer {
    id: u64,
    name: String,
    email: String,
}

impl Customer {
    /// # Example
    /// ```
    /// Customer::new(1, "customer name".to_string(), "customer_email@xxx.com")
    /// ```
    pub fn new(id: u64, name: String, email: String) -> Self {
        Customer { id, name, email }
    }
}
