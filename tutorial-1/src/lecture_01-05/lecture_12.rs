// Borrowing in Functions

/**
 * - Borrowing Rules:
 *   - At any time, you can either one mutable reference or any number of immutable references.
 *   - References mube always be valid.
*/

fn main() {
    case_1_immutable_borrows(); 
    case_2_mutable_borrows(); 
}

fn take_ownership_borrow(vec: &Vec<u32>) {
    println!("vec is {:?}", &vec);
}

fn take_ownership(vec: Vec<u32>) {
    println!("vec is {:?}", vec);
}

fn takes_and_gives_ownership(vec: &mut Vec<u32>) {
    vec.push(99);
}

fn case_1_immutable_borrows() {
    let vec_1 = vec![1, 2, 3];
    // here we create a immutable reference of vec_1
    let ref1 = &vec_1;

    // // this is invalid because the ownership will cause vec_1 heap space is already clean up
    // take_ownership(vec_1);
    // println!("vec_1 is {:?}", vec_1);

    // here we use borrow to avoid ownershop hand over and duplicate clone
    // and this should be work
    take_ownership_borrow(ref1);
}

fn case_2_mutable_borrows() {
    let mut vec_1 = vec![1,2,3]; 
    let ref1 = &mut vec_1; 
    takes_and_gives_ownership(ref1); 
    println!("ref1 {:?}", ref1); 
}
