// Iterator

// inner defintion of iterator trait
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

use std::io::empty;

#[derive(Debug, Clone)]
struct Employee {
    name: String,
    salary: u16,
}

struct Employee_Records {
    employee_db: Vec<Employee>,
}

impl Iterator for Employee_Records {
    type Item = Employee;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.employee_db.is_empty() {
            let result = self.employee_db[0].clone();
            self.employee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let mut emp1 = Employee {
        name: String::from("a"),
        salary: 42_000,
    };
    let mut emp2 = Employee {
        name: String::from("ab"),
        salary: 42_000,
    };
    let mut emp3 = Employee {
        name: String::from("acd"),
        salary: 42_000,
    };
    let mut emp4 = Employee {
        name: String::from("aef"),
        salary: 52_000,
    };
    let mut emp5 = Employee {
        name: String::from("ag"),
        salary: 22_000,
    };

    let mut emp_db = Employee_Records {
        employee_db: vec![emp1, emp2, emp3, emp4, emp5],
    };

    // println!("item in emp db {:?}", emp_db.next());
    // println!("item in emp db {:?}", emp_db.next());
    // println!("item in emp db {:?}", emp_db.next());
    // println!("item in emp db {:?}", emp_db.next());
    // println!("item in emp db {:?}", emp_db.next());
    // println!("item in emp db {:?}", emp_db.next());

    for emp in emp_db {
        println!("{:?}", emp);
    }
}
