// Functions && Code Blocks

fn main() {
    my_fn("This is my function");
    multiplication(1, 2);
    let ret = basic_math(1, 2);
    let (ret1, ret2, ret3) = basic_math(3, 4);

    let retttt = {
        let first_name = "a";
        let last_name = "b";
        format!("{first_name} {last_name}");
    };
}

fn my_fn(s: &str) {
    println!("value of passing string:  {s}")
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    println!("Computing multiplicaiton");
    let ret = num1 * num2;
    println!("final result value is {ret}");
    ret
}

fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 + num2, num1 * num2, num2 / num1)
}
