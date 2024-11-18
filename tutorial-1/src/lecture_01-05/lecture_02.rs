fn lecture_02() {
    // Unsigned integers
    // u16, u32, u64, u128
    let unsigned_num: u8 = 5;

    // Signed integers
    // i16, i32, i64, i128
    let signed_num: i8 = 4;

    // Floating point numbers
    let float_num: f32 = 5.0; // f64

    // Platform specific integers
    let arch_1: usize = 4;
    let arch_2: isize = 5;

    // Characters
    let char: char = 'a';

    // Boolean
    let b: bool = true;

    // Type aliasing
    type Age = u16;
    let peter_age: Age = 14;

    // Type Conversion
    let a: u16 = 65535;
    println!("value a: {a}");

    // here let convert a into u64
    let a1 = a as u32;
    println!("value a1: {a1}");

    // here try to convert it in a u8
    let a2 = a as u8;
    println!("value a2: {a2}");
}
