use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Foo {
    a: u64,
    b: String,
}
fn main() {
    println!("Hello World");
}
