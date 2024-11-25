// Box Smart Pointer

// here we have a re-fine version of using Box Smart Pointer to implement recursive type defintion
#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}

#[derive(Debug)]
struct Huge_Data;

#[derive(Debug)]
struct Small_Data;

trait Storage {}

impl Storage for Huge_Data {}

impl Storage for Small_Data {}

fn main() {
    // let list = List::Cons(
    //     1,
    //     Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))),
    // );

    // println!("list content {:?}", list);

    // Boxes are also useful when copy large amount of data, and when transferring ownership

    let data_1 = Huge_Data;
    let data_2 = Box::new(Huge_Data);

    let data_3 = data_1;
    let data_4 = data_2;

    // here the ownership of data_1 is transferred to the data_3
    // and the ownership of data_2 is now being transferred to data_4
    // -- > when transferring item's ownership, data is copied on the stack < --

    // but in data_3 <- data_1 , this will invoke data copy on the heap, the Huge_Data space will re-applied, and allocated, and data will be copied to the new space
    // but in data_4 <- data_2, this will only invoke the reference's copie
    // acturally in data_4 <- data_2 this period, the data is not copied,

    let data_5 = Box::new(Small_Data);

    // and here there is an error of data_5 the compiler gives the information of 'expected struct Box<Huge_Data>'
    // this is because, vectors can only hold the values that have the same type
    // so use the raw type of vector is not ok, let's use the dyn feature that supported by the Box
    // let data = vec![Box::new(data_3), data_4, data_5];

    // and this is tell the vector it can holds all the instances which implement the Storage trait
    // and this time , it is ok
    let data: Vec<Box<dyn Storage>> = vec![Box::new(data_3), data_4, data_5];
}
