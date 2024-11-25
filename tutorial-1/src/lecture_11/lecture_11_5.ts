// Box Smart Pointer

// here we define a enum type which is recursive type
// which is self-contained from defining level,
// the problem for this type is when we declare it
// the compiler does not know how long it is, or exactly size of memory space allocated to this variable
// length of this kind cannot decide at the compile time --> this will cause compile error
#[derive(Debug)]
enum List {
    // how much space should be allocated,
    // it is a recursive type

    // the compiler recommend use Box wrap the recursive type
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // as we know that basic variables in Rust are stored in stack memory
    let x = 0.23456;

    // but this creation will invoke a heap-allocation
    // which mean the y is the heap-located variable

    // the value of y now is a Box Pointer which is pointing to
    // heap memory contain the value of  0.23456
    let y = Box::new(x);

    // let create a reference to x
    // z is also a pointer, but it is the pointer points to the stack memory
    let z = &x;

    let list = Box::new(List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    ));

    println!("content of list {:?}", list);
}
