fn main() {
    // &str and String
    let fixed_str = "Fixed length string";
    let mut flexible_str = String::from("This string will grow");

    // so what happens if we try to set let this ?
    // the fixed_str_v2 will not in type of &str but instead will conveted into String dynamically
    // but when we try to reassign a string value to it, it will have warning message in editor
    let fixed_str_v2 = String::from("What type of string will this be?");
    println!("fixed str v2 {fixed_str_v2}");
    // this would be wrong
    // fixed_str_v2 = "converted into fix ?";

    // Arrays

    // this means that we create a flexible array that has been allocated space with 4 elements
    // and each element's type is i32

    // and even though we call it as the mut array but it's size is fixed
    // so every time we try to re-assign it to an new array the size and type should be matched strictially
    let mut array_1: [i32; 4] = [1, 2, 3, 4];

    //this cannot work, because it requires 4 elements
    // array_1 = [1, 2];
    // println!(array_1);
    // this should work, because we apply array by 'mut' so re-assign it is ok
    array_1 = [1, 2, 3, 4];

    // here we use a lamda to iterate each element in the mutable array
    array_1.iter().for_each(|val| println!("{val}"));

    println!("{:?}", array_1);

    let array_2 = [1, 4, 6, 7, 54, 3];

    // Vectors
    let vec_1: Vec<i32> = vec![4, 5, 6, 7];
    let num = vec_1[3];
    println!("{num}");

    // Tuples
    let my_info = ("Salary", 3728930, "Age", 39);
    let salary_value = my_info.1;
    let age = my_info.3;
    let (salary_name, salary_value, age_name, age_value) = my_info;
    println!("{age_value}");
    println!("my info is : {:?}", my_info);

    // here we declare an empty tuple
    let unit: () = ();
}
