// Comments in Rust

use serde::de::Expected;

fn main() {
    // THe current line is a comment line
    // This is the second line of comment

    /*
     Thi sis a multiple line of comment
     second line
    */

    print!("This is a print command");
    println!("THis is going to e printed on the same line");
    println!("\n another line");
    println!("test comment \"value\" ");

    println!(
        "print arg-2 {2}, this is the first argument{1}, and this is the 0 arg {0}",
        "a", "b", 0
    );

    println!(
        "{language_name} is a system programming language which is cool to {activity_name} in.",
        language_name = "Rust",
        activity_name = "Code"
    );

    // here we try to get input data from the user
    let mut n = String::new();

    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");

    println!(
        "receive from console value is {n} with length {:?}",
        n.len()
    );

    let n: f64 = n.trim().parse().expect("invalid input");
    println!("receive data from console should be in type of u64 {n}");

    // here are some usage in Rust of the underscore
    let _number_one = 45.0;
    let x = 40_000;

    // and here we try to use the static key word to declare
    // a varaible of static
    static WELCOME: &str = "Welcome to the World of Rust";
    println!("here is the content of the str {WELCOME}");

    const PI: f32 = 3.14;

    let a = PI;
    let b = PI;
}
