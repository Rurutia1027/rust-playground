# Publish Your Crate to [crates.io](https://crates.io)

## Steps to Publish Your Local Cargo Project as a Public Crate to [crates.io](https://crates.io)
* Login [crates.io](https://crates.io) by Github Account 
* Verify your email address on crates.io
* Add comments for modules and functions and structs, by such format: 
```
///
/// # 
```
* On local project run `cargo doc --open`, this will setup a local online document for your project 
* Modify and add more details on your source codes
* Add description and license to your project's `Cargo.toml` file 
* Run `cargo publish --allow-dirty`, allow dirty allows local codes not commit to github