// Dereferencing

fn main() {
    // basic -- type -- stack -- located --
    // first we need to guarantee that the variable should be mutable
    let mut some_data = 42;

    // here we create a reference ref_1
    // then create a reference of it -- here just like java
    let ref_1 = &mut some_data;

    // here we create a dereferencing copy by operating upon the reference
    // deref_copy here will trigger a clone operations
    // clone the value of some_data(which ref_1 points)
    // and point
    let deref_copy = *ref_1;

    // and with the help of dereferencing we can do operations to modify the original data
    *ref_1 = 313;

    println!("some_data now is {some_data}, deref_copy_is: {deref_copy}");

    // other -- types -- are -- heap -- located --

    let mut heap_data = vec![5, 6, 7];
    // here we create a reference to the vector
    let ref_1 = &mut heap_data;

    // the compiler is not happy with this
    // and this is because the integers, char, bool those Rust basic types
    // are static allocated, their space is allocated on stack instead of heap
    // and when they execute a assign it will invoke a clone operations, which mean
    // stack will allocate another space to save the re-assigned value

    // here compiler not happy is we trying to re-assign a heap-located data means
    // a change of ownershp of that heap-located data
    // and there are two potentially two problems when changing the ownership:
    // >> 1 first the ref_1 is only the reference of the data, it just 'borrow' the data not 'own' the data,
    //      so ref_1 cann't modify the heap-located data's ownership
    // >> 2 moving a value out of a mutable reference could potentially leave the reference invalid
    // so, this is not allowed let deref_copy = *ref_1;
    // but we can use clone function instead, this means create a duplicated heap-located vector
    // and let deref_copy refer to the duplicated/cloned one
    let deref_copy = ref_1.clone();
    let move_out = ref_1;
    // here again, this again will also let compiler not happy
    // this is because mutable reference, cannot be copied, but only be moved.
    // that's why we cannot have multiple mutable references
    // suppose mutable references are allowed to copied, we can have one heap-data with multiple references
    // and they all can modifiy the heap-data which will again cause !!! data race!!!
    // let move_out_agagin = ref_1;
    move_out.push(12);
    // this shows that 12 will be append because we create a mutable references to the mutable heap-data
    println!("move_out: {:?}", move_out);

    // but this is allowed, because we use &heap_vector not &mut heap_vector
    // the &heap_vector means create a immutable reference to the heap-data instance
    // none of the immutable references are allowed to do modicaitons to the heap-data so ,
    // the!!!data race!!! will not happen among multiple immutable references
    let mut heap_vec = vec![3, 4, 5, 6, 7, 34];
    let ref_a = &heap_vec;
    let ref_b = &heap_vec;
}
