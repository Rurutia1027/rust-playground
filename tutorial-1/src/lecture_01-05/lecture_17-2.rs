// Option

struct Student {
    name: String,
    grade: Option<u32>,
}

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

// here we continue add a function to check whether a given student_name
// is exist in the student_db, different from the previous get_grade
// this time we will let the function support if there is a student with the given name then return his/her grade value
// but if the student we are searching for is not exist in the student_db, we return an Error instead of a None which
// represents no students entity exists and query fail

// and this time we use the enumeration of Result instead of Option
// and Result's Rust inner definition is shown below
// enum Result<T, E> {
//    Ok(T),
//    Err(E), --> this is different from None it support wrapping different types of error in it
// }
fn check_student(
    student_name: &String,
    student_db: &Vec<Student>,
) -> Result<(), String> {
    for item in student_db {
        if (item.name == *student_name) {
            return Ok(());
        }
    }

    Err(format!(
        "No students found with the student name {} in the student database",
        student_name
    )
    .to_string())
}

fn check_student_get_grade(
    student_name: &String,
    student_db: &Vec<Student>,
) -> Result<Option<u32>, String> {
    for item in student_db {
        if (item.name == *student_name) {
            return Ok(item.grade);
        }
    }

    Err(format!(
        "No students found with the student name {} in the student database",
        student_name
    )
    .to_string())
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
    let student_status = check_student_get_grade(&student_name, &student_db);

    match student_status {
        Ok(options_grade) => {
            if let Some(grade) = options_grade {
                println!("Grade is {grade}");
            }
        }
        Err(err_msg) => {
            println!("{err_msg}");
        }
    }

    let student_name = String::from("A");
    let student_status = check_student_get_grade(&student_name, &student_db);
    match student_status {
        Ok(options_grade) => {
            if let Some(grade) = options_grade {
                println!("Grade is {grade}");
            }
        }
        Err(err_msg) => {
            println!("{err_msg}");
        }
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
