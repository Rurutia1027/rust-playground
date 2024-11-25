#[derive(Debug, Default)]
// use derive decorator + Default is the function2 to enable the struct implement the struct default instance
pub struct Student {
    pub id: u8,
    pub age: u8,
    pub name: String,
}

impl Student {
    pub fn new(s_name: String) -> Result<Self, String> {
        if s_name.chars().all(|x| matches!(x, 'a'..='z')) {
            Ok(Self {
                id: 0,
                age: 29,
                name: s_name,
            })
        } else {
            Err("The name is invalid".to_string())
        }
    }
}

// func-1 to implement a default constructor for Student
// impl Default for Student {
//     fn default() -> Self {
//         Self {
//             id: 0,
//             age: 20,
//             name: "defaultname".to_owned(),
//         }
//     }
// }
