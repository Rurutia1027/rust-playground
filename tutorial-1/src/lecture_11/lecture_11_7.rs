// Reference Counting Smart Pointer

use std::rc::Rc;

// Comparing with Box Smart Pointer RC Counting Pointer:
// RC Pointer supports the share of the pointer in a more secure way
// by recording the counter of the pinter borrowing times
enum List {
    Cons(i32, Option<Rc<List>>),
}

fn main() {
    // By rc clone we now have three owners of the data a,b,c
    let a = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None)))));

    // here we print the RC's reference counter value
    println!("rc counter value : {}", Rc::strong_count(&a));

    {
        let b = List::Cons(3, Some(Rc::clone(&a)));
        let c = List::Cons(4, Some(Rc::clone(&a)));

        println!("rc counter value : {}", Rc::strong_count(&a));
    }
    // when all of the a,b,c drop the data
    // then it comes back to the RC, RC finds its inner reference counter = 0
    // then it drops the variable and clean the memroy

    // here we print the rc counter
    println!("rc counter value : {}", Rc::strong_count(&a));
}
