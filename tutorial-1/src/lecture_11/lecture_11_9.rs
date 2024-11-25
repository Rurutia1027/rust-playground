use std::cell::RefCell;

fn main() {
    // in previous lectures, it is introduced that
    // if we create a immutable variable, there is no way to get its mutable reference
    // like this
    let x = 1234;
    // this is not allowed, because variable of x is immutable
    // let x1 = &mut x;

    // but it became different, if we use RefCell::new(...)
    let a = RefCell::new(10);
    let mut b = a.borrow_mut();
    *b = 13;

    // but we need to drop value b here , other wise we cannot use a to get access to the value
    drop(b);
    println!("value of b is {:?}", a);
}
