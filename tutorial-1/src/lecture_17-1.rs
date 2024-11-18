// Option

struct Student {
    name: String,
    grade: Option<u32>,
}

// this function show how we can use Option as a return value
// and how Option is declare as the return value of a function
fn get_grade(
    // here we use borrow --> which pass the parameters as reference
    // because both Vec and String are heap-located items
    // directly passing item will result in ownership transfer
    student_name: &String,
    student_db: &Vec<Student>,
) -> Option<u32> {
    // just like this we declare the function use Option::None as temporary return value
    // to occupy the space of tobe implemented return value

    for stu in student_db {
        // here we use (* + reference_name) to retrieve heap value
        if stu.name == *student_name {
            // inner for return is necessary
            return stu.grade;
        }
    }
    Option::None
}

fn main() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(98),
        },
        Student {
            name: "Bob".to_string(),
            grade: Some(91),
        },
        Student {
            name: "Mae".to_string(),
            grade: Option::None,
        },
    ];

    let student_name = String::from("Bob");
    let student_grade = get_grade(&student_name, &student_db);

    match student_grade {
        Some(u32) => {
            println!(
                "matching stu name {} with grade {}",
                student_name,
                student_grade.unwrap()
            );
        }
        None => {
            println!("no matching student name {}", student_name);
        }
    }

    if let Some(grade_value) = student_grade {
        println!("Grade is {grade_value}");
    }
}

// Rust has its inner defined enums
// Option<T> is one of them,
// suppose a scenario that we need to create a struct instance with series of variables
// but, we just want to set some default values to occpy the variable for temporary and set their value latter
// in this situation, we can use Option::None this enum to occpy the variable as a default value
// which will  not raise a compile error, and later refine the logic of it

// here is the defintion of enum Option in Rust
// enum Option<T> {
//     None, --> this represents the absence of the value, but if we want to use it to occopy
//           --> we need to delcare the variable as Option<type_name> at first
//     Some(T),
// }
