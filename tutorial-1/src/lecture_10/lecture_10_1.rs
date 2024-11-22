// Closures

use std::collections::HashSet;

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    salary: u32,
}

fn validate_user(name: &str) -> bool {
    name.len() != 0
}

// here we define a function that supports takeing closures as parameters
fn is_valid_user<V1, V2>(
    name: &str,
    age: u8,
    c_validator_1: V1,
    c_validator_2: V2,
) -> bool
where
    V1: Fn(&str) -> bool,
    V2: Fn(u8) -> bool,
{
    c_validator_1(name) && c_validator_2(age)
}

fn main() {
    let p1 = User {
        name: "meme".to_string(),
        age: 24,
        salary: 100_000,
    };

    println!("validate result value : {}", validate_user(&p1.name));

    // The previous implementation involved passing some inner fields of the struct
    // to specify functions for processing.

    // Now, we demostrate another approach using closures.
    // Closures allow us to define executable logic inline, similar to funcitons,
    // but without explicitly declaring a named fucntion .

    // Question: What are the benefits of using closures compared to regular functions?
    // Is it about saving resources, improve efficiency, or avoiding ownership-related risks?
    // As a developer coming from a Java background, closures feel similar to create an
    // anonymous function by without the need for an explicit declaration of a 'named' function.

    // Answer: (from GPT):
    // 1. Inlined Context and Simplified Code:
    //    - closures allow you to define logic directly at the point of use, reducing the need for creating
    //    - named functions that may clutter the code . This is especially useful for one-off operations
    //    - or when the logic is tightly copupled with its usage.

    // 2. Capturing Environment:
    //    - closures can capture variables from their enclosing scope. This elimintates the need to
    //    - explicitly pass those variables as arguments(and abstract function signatures),
    //    - this making them highly convenient for scenarios where the surrounding state is relevant.

    // 3. Flaxibility with Ownership:
    //    - Closures provide fine-grained control over ownerships:
    //    --- move closures allow transferring ownership of captured variables.
    //    --- non-move closures borrow variables, avoiding unnecessary ownership transfer or copies.
    //    - Closures help manage Rust's ownership and borrowing rules effectively.

    // 4. Efficiency:
    //    - Closures in Ruest are often inlined and optimized by the compiler. For small, simple operations, using closures
    //      can reduce function call overhead and improve runtime performance compared to declaring a separate function.

    // 5. Improved Code Readability:
    //    - By avoiding separate declarations, closures keep the logic closer to where it is invoked and called,
    //      this making the code easier to read and maintain.
    //
    // 6. Anonymous and Resusable:
    //    - Closures are anonymous and can be passed around as function parameters (e.g., higher-order functions like map,
    //      map, filter, etc.), enabling a more functional programming style. This reduces boilerplate code.
    //
    // When to Use Closures
    // - Use closures for short-lived, concise logic that depends on its surrounding context or doesn't need reuse.
    // - Use declared functions for larger, reuseable logic that can exist indepdnently of the surrounding context or needs better documentaiton.
    let validate_result = |name: &str| name.len() != 0;

    // FnOnce, if we want to continue use the closure's outer scoped
    // variable for multiple time, we should avoid use the FnOnce which will invoke ownership of outer variables
    // move inside of the inner closure permenately 
    let mut user_black_list = HashSet::new();
    user_black_list.insert("meimei");
    user_black_list.insert("nana");

    // this can also be implemented like this
    let validate_name_v2 = |name: &str| {
        println!("ok here we are inside of the closure, can inside closure, \n
        we can get access to mulitiple items from the outer scope of the context, \n
        such as we can get access to the parameters of this closure name: {}\n
        well... more than that, we can even get acces to the instance of the User like: {:?}", name, p1);

        // borrow the outer hashset instad of taking its ownership
        let black_list = &user_black_list;
        name.len() != 0 && !black_list.contains(name)
    };

    // continue define a closure
    let validate_age = |age: u8| {
        println!("ok here we are inside of the closure, can inside closure, \n
        we can get access to mulitiple items from the outer scope of the context, \n
        such as we can get access to the parameters of this closure age: {}\n
        well... more than that, we can even get acces to the instance of the User like: {:?}", age, p1);
        age > 99
    };

    // it can be used as a function
    println!(
        "ret value of validate_ret is:  {}",
        validate_name_v2(&p1.name)
    );

    validate_age(p1.age);

    // here we invoke the function with closure as parameters
    println!(
        "return value of function is_valid_user is {}",
        is_valid_user(&p1.name, p1.age, validate_name_v2, validate_age)
    );
}
