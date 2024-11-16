// Loop

fn main() {
    'outer: loop {
        println!("Simple loop");
        break;
    }

    let a = loop {
        break 4;
    };

    println!("a value {a}");

    // here we try for loop
    // first we define a vector
    let vec = vec![3, 4, 3, 6, 7, 8, 90, 23];
    // now loop using the for loop
    for i in vec {
        println!("value of i is {i}");
    }

    // here we use the while loop to iterate the valu ein the vec
    let mut i = 0;

    let mut index = 0;
    while index < 100 {
        index += 1;
        println!("index value is {index}");
    }
}
