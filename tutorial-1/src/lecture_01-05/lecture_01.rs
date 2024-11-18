pub fn lecture_01() {
    // Definition
    let x: i16 = 10;
    println!("x is {x}");

    // Mutability
    let mut y: i32 = 5;
    y = 10;

    // Scope
    {
        let x = 50;
    }

    // this should not work
    // let xx = z;

    // Shadowing
    let t = 10;
    //
    let t = t + 10;
    println!("t is {t}");

    let v: i32 = 30;
    {
        let v = 40;
        println!("Inner v is {v}");
    }
    println!("v is: {v}");

    // Constants
    const MAX_VALUE: u32 = 100;
}
