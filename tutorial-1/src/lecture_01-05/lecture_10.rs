// Function's Ownership

fn main() {
    let vec_1 = vec![1, 2, 3];

    // here is the feature of Rust when we pass the vec_1 to the function take_ownershp
    // the ownership will be handed over to the take_ownership from current context
    // and there are no return values from the funciton, and after executing the function's
    // inner all variables will be dropped, and result in the vec_1's value or the space in the heap is released
    // and simply speaking the ownership handed over to the function and not renturn so after calling the fucntion
    // return to current context the vec_1's ownership not belong to current context anymore and its value already not available for current context
    // that's why it cannot be get access to in current context function
    // -- take_ownership(vec_1);
    // -- println!("vec 1 is: {:?}", vec_1);

    // but wait a minute, if we pass an instance clone to the
    // function of take_ownership instead, it means that we handing the clone ownership to the function
    // the original one's ownership still remains in current context
    // this will not affect current context's vec_1's later invocation

    // well this is a little bit different from the address passing and refrence passing concept in the c or c++ scope
    // the concept of ownership helps me better understand its working strategy
    // but, it seems that in c++/c we passing a reference to the function, the refrences pointed to value's modificaiton
    // will not affect invoker's outsider's condition, and the instance that the reference points to can still work
    // maybe this ownership strategy can totally avoid memory leak in case the vec_1 this 'pointer' points to a invalid heap space
    // in the coming logic
    take_ownership(&vec_1);
    println!("vec_1's content is {:?}", vec_1);

    let vec_2 = gives_ownership();
    println!("vec_2 is {:?}", vec_2);

    let mut vec_3: Vec<u32> = vec![1, 2, 3, 3, 4];
    takes_and_gives_ownership(&mut vec_3);
    println!("vec_3: {:?}", vec_3);

    let x = 19;
    stack_function(x);
    // when we try to passing Rust inner defined basic types like integer, char, float or other basic values
    // to the funciton we find that the ownership strategies do not work here,
    // this is because the Rust it treates every basic type as clone operation when developer wants to passing them as parameters to the function
    // clone operation is executed, so the operation in the scope of the function is execute upon the clone instance othe outer scoped isntance value will not be affected.
    println!("in main x value is {x}");
    //
}

fn take_ownership(vec: &Vec<u32>) {
    println!("vec_1 clone's content is : {:?}", vec);
}

fn gives_ownership() -> Vec<i32> {
    vec![4, 5, 6]
}

// add &mut Vec<u32> here if we do modification in the scope of the function
// then the modificaitoin will updated to the parameters of vec
fn takes_and_gives_ownership(vec: &mut Vec<u32>) {
    &vec.push(99);
}

fn stack_function(mut var: i32) {
    var = 56;
}
