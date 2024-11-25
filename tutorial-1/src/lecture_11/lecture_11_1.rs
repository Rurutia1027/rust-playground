// Concrete Lifetimes in Rust

fn main() {
    // the lifetime of this variable i starts when it is created
    // and ends at the end of the main function.
    let i = 4;
    let j = i;
    println!("value of i {}", i);

    {
        // but the lifetime for x this block scoped variable
        // ends when the end of the block ends
        // which means it is cleared from the memory, and when logic go outside of the block
        let x = 33;
    }

    let str1 = String::from("test str");
    let str2 = str1;
    // this is ownership change caused str1's lifetime end
    // println!("value of str1 {}", str1);

    // this is function param passing caused ownership change lifetime end
    // str_fn(str1);
    // println!("value of str1 {}", str1);

    let i;
    {
        let j = 99;
        // an error here:i = &j, `j` does not live long enough
        // i = &j;

        // but no error if we i = j;
        // this is because, if we use i = j, the memory of j will be sent to i to point
        // so this piece of memory will be maintain,

        // if we use i = &j, it means the memory still belong to j, i only as a type of reference
        // which store the value of the to be released memory address value.
        i = j;
    }

    println!("value of i is {}", i);

    // mutable && immutable reference associated lifetime problems
    let mut vec_1 = vec![2, 3, 5, 2, 3];
    let ref_1 = &vec_1; // ref_1 lifetime starts
    println!("value of ref_1 {:?}", ref_1); // ref_1 lifetime ends

    let ref_2 = &mut vec_1; // ref_2 start
    ref_2.push(1);
    println!("value of ref_2 {:?}", ref_2); // ref_2 end

    // here something will go wrong, that we because,
    // to a heap-located instance, either {multiple immutable references} or {only one mutable reference}
    // can point to the heap-located instance to avoid data race and dirty data read
    // if create multiple reference the previous created reference will be lifetime end
    // println!("value of ref_1 {:?}", ref_1);
}

fn str_fn(s: String) {
    println!("received s value {}", s);
}
