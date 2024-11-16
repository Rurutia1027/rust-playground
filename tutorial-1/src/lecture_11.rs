// Borrowing in Rust
// In previous lecture, the lecturer introduced the ownership feature usage and how to use it
// expecially for Vec those kind of objects in the scenario of insider function and outside funciton
// passing and receiving instances.
// however, if we need to passing variables to function, and do some modifications in the scope of the function
// and then let the return instance has affected by the modification logic, we often do not use ownership
// instead there is a better solution for that scenario, that is the Borrowing

// what's borrowing ? it means establishing a reference to some data
// it works like a pointer in C/C++, but it does not take the ownership of the data or instance(which stores in the heap space)
// which means the modificaitons in the function scope will affect the data we passing
// and when we exit the scope of the funciton,the ownership still remains in the scope of the context(actually it does hand over at the begin)
// so when exit the scope of the invoke scope, the heap space of the data will not be released or deleted.
// and it can still be available in current context.

// the lecturer gives the question that is why we use borrowing/
// one is because sometimes we just need do handle the modification logic to the function scope
// we need to use the value when context switch from function back to current context -- there is no need to hand over the ownership from current context to the function

// and the seconde is we just do not want apply for extra space (which clone does)
// we just create another reference the same as the context one but in the scope of funciton context
// and let it point to the same heap space for the data/instance which is passed to the fucntion as a parameter.

// lecture also give us two rules to follow if we want to use the reference
// one is: at any time, you can either one mutable reference or any number of immmutable references
//      --  i think this is mainly avoid two mutable reference may cause dirty data generation or two modificaiton not synchronize caused data not consistent
// the other is: reference must always be valid:
//      -- i think this must be the refernce works like a pointer, and it must points to a valid space which is allocate on the heap.
//      -- otherwise an invalid reference try to get access data may cause memory leak

// simply speaking one is solve out 'Data race' -- which two references both want to do modificaitons upon one value,
// the order of the modification may cause error
//'Dangling refrence' this should be the reference trying to get access to an already released heap space will result in
// some danger things happen in the program
// and i thing this must be the Rust's safety guarantee's implementation which let Rust more secure than C/C++'s pointer rules.

use std::vec;

fn main() {
    let mut vec_1 = vec![4, 5, 6];

    // here we create two mutable reference -- which will be prevent by the compiler
    let ref_1 = &mut vec_1;

    // this will result in data race which compiler will not you to do so
    // println!("ref1: {:?}, ref2: {:?}", ref_1, ref_2);

    // here we create two immutable reference-- immutable refrence only allow you
    // get access to the values of the data but you cannot use the reference to do any modificaitons
    // that directly affect the original data stored on heap

    // and multiple immutable referens creation is always allowed in Rust
    let ref_3 = &vec_1;
    let ref_4 = &vec_1;
    let ref_2 = &mut vec_1;
    println!("ref_3: {:?}, ref_4: {:?}, ref_2: {:?}", ref_3, ref_4, ref_2);

    // here is another thing we need to notice, that is we can either
    // 1. create a series of immutable references
    // or
    // 2. only one mutable reference at the same time

    // here we try the other conditon: make sure the reference should always be valid

    // in the ide we can see warnnings of the compiler
    // this is because the block holds the ownershp of the vec_3 which stores in the heap
    // but when get out of the scope of the block the block's ownership comes to the end
    // , and vec_3 will be deleted and released the space allocated on heap will be invalid
    // and let refrence vec_2 pointed to an invalid space is not allowed in Rust.
    // this is the Dangling reference
    let vec_2 = {
        let vec_3 = vec![1, 2, 3];
        &vec_3
    };
}
