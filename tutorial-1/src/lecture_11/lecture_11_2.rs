// Generic Lifetimes
use rand::random;
fn main() {
    let int1 = 5;
    let picked_value;
    {
        let int2 = 10;
        picked_value = picking_int_static(&int1, &int2);
    }
    println!("{picked_value}");
}

// solution of generic type
fn picking_int<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if rand::random() {
        i
    } else {
        j
    }
}

// solution of using 'static keywords
fn picking_int_static(i: &i32, j: &i32) -> &'static i32 {
    let y: &'static i32 = &6;
    y
}
