// each binary crate must have a main function as it's entry point
// and we already know that package:crate = 1:N
// package:binary_crate = 1:M,
// ---- binary crates are executable binary files(should cotnain entry point -- the main function)

// package:lib_crate = 1:1,
// ----- library crate is library file for sharing the structs and functions across the package scope available

// this binary crate can be execute by cargo run --${binary_name},
// here the binary name is the rust file's name that is `app1`
fn main() {
    println!("Hello from app1 ");
}
