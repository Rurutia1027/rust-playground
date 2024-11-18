# Referencing and Dereferencing in Rust 

## Scenario 1
```rust
struct Student {
    name: String,
    grade: Option<u32>,
}

fn get_grade_version_1(
    // Pass student_name as a reference to avoid moving the String and transferring ownership.
    // This is efficient because it avoids copying or transferring large data, 
    // especially for heap-allocated items. 
    student_name: &String,

    // Pass student_db as a reference to avoid moving the entire Vec<Student>,
    // which could involve a costly transfer of ownership or copying the collection 
    student_db: &Vec<Student>,
) -> Option<u32> {

    // here, temporarily use Option::None as a placeholder return value to 
    // indicate no matching student found 
    for stu in student_db {

        // !!! here use the dereferencing (*) to access the value of the student_name reference !!!
        // !!! this is because student_name is a reference, but stu.name is an owned String --> it is owned by the stu even though Vec<Student> is a reference
        // simply speaking stu.name is a type of String
        // student_name is a type of reference(&String) 
        // String and &String are two types, one is data on heap, the other is address of the heap 
        // two types cannot compare directly 
        if stu.name == *student_name {
            return stu.grade;
        }
    }
    Option::None
}


fn get_grade_version_2(
    student_name: &String,
    student_db: &Vec<Student>,
) -> Option<u32> {
    for stu in student_db {
        // !!! but, we directly use student_name that skip the explicit '*' this is ok !!!
        // !!! this is because when comparing &String with String Rust's PartialEq for String and &String handles the dereferencing interally 
        // so this expression will also work as expected 
        if stu.name = student_name {
            return stu.grade; 
        }
    }
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
}
```

### Explanation of Key Concepts:
#### Passing references for efficiency 
Both students_name and student_db are passed as references(&String and &Vec<Student> respectively) to avoid transfering ownershp. This is important because transferring ownershp would prevent the original variables from being used further might involve necessary copying. 

#### Dereferencing 
Since student_name is a reference (&String) using *student_name dereferencing it to access the String it points to. This is necessary because stu.name is an owned String, and you cannot compare &String with a String. 
Alternatively, you could compare without explicit dereferencing by using the fact that Rust's PartialEq implementation for String and &String which allows direct comparision. 
