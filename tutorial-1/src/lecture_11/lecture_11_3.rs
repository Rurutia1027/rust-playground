// Lifetime Elision

fn main() {
    let str_1 = "some str";
    let str_2 = "other str";
    let received_str = return_str(&str_1, &str_2);

    println!("received str value {}", received_str); 
}

fn return_str<'a, 'b>(s_1: &'a str, s_2: &'b str) -> &'a str {
    s_1
}
