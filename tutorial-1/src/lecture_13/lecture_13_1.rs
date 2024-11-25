// Initializing Struct Instance

use tutorial_1::Student;

fn main() {
    // let std_1 = Student {
    //     age: 20,
    //     name: "Mae".to_string(),
    // };

    let std_1 = Student::new("meme".to_owned());
    println!("std: {:?}", std_1);

    let std_2 = Student::new("abc123".to_owned()).unwrap_or_default();
    println!("std: {:?}", std_2);

    // another way to invoke default constructor

    let std_3 = Student {
        age: 18,
        ..Default::default()
    };

    println!("std: {:?}", std_3);
}
