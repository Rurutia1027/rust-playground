// RefCell Smart Pointer

use std::cell::RefCell;

fn main() {
    let mut x = 49;
    let x1 = &x;
    let x2 = &x;

    // here we can continue get access to the value of the x1 and x2
    println!("x1: {}, x2: {}", x1, x2);

    // but, here, after we create a mutable reference to the variable of x
    // compiler will go wrong, because Rust supports either 1 mutable reference or multiple reference
    // point to the variable at the compile time, otherwise there will be a data-race

    let a = RefCell::new(10);
    let b = a.borrow();
    let c = a.borrow();
    println!("b {}, c {}", b, c);

    // but if we drop our immutable reference, the later apply for mutable reference operation is allowed.
    drop(b);
    drop(c);

    let d = a.borrow_mut();

    // here we print the variales that are immutable borrows

    // here we can find in the compile time, the compiler does detect any issues
    // but during the runtime we will got an error that is : already borrowed: BorrowMutError--> which is caused by the
    // expression of a#borrow_mut()
    // println!("b {}, c {}", b, c);

    // and again, we cannot use a to get access to the value
    // because we already create mutable reference to d, so a cannot get access to the value
    // println!("value of a {:?}", a);

    drop(d);

    // but if we drop the previous mutable reference to a, a is available
    println!("value of a {:?}", a);
}
