// here we also need to know, make a module public
// it's inner variables like structs or functiosn will not become public automatically and iteratively
pub struct Customer {
    id: u64,
    name: String,
    email: String,
}

impl Customer {
    pub fn new(id: u64, name: String, email: String) -> Self {
        Customer { id, name, email }
    }
}
