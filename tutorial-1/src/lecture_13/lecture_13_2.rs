// Builder Pattern

use std::mem;

#[derive(Debug, Default, Clone)]
struct Customer {
    name: String,
    username: String,
    membership: Membershiptype,
    gender: char,
    country: String,
    age: u8,
}

#[derive(Debug, Clone)]
enum Membershiptype {
    new,
    causual,
    loyal,
}

impl Default for Membershiptype {
    fn default() -> Self {
        Membershiptype::new
    }
}

impl Customer {
    fn new(name: String) -> CustomerBuilder {
        CustomerBuilder {
            name: name,
            ..Default::default() // username: None,
                                 // membership: None,
                                 // gender: None,
                                 // country: None,
                                 // age: None,
        }
    }

    fn new_1(name: String) -> Self {
        Customer {
            name: name,
            ..Default::default()
        }
    }

    fn new_2(name: String, username: String) -> Self {
        Customer {
            name: name,
            username: username,
            ..Default::default()
        }
    }

    fn new_3(
        name: String,
        username: String,
        membership: Membershiptype,
    ) -> Self {
        Customer {
            name: name,
            username: username,
            membership: membership,
            ..Default::default()
        }
    }
}

#[derive(Default)]
struct CustomerBuilder {
    name: String,
    username: Option<String>,
    membership: Option<Membershiptype>,
    gender: Option<char>,
    country: Option<String>,
    age: Option<u8>,
}

impl CustomerBuilder {
    fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    fn membership(&mut self, membership: Membershiptype) -> &mut Self {
        self.membership = Some(membership);
        self
    }

    fn gender(&mut self, gender: char) -> &mut Self {
        self.gender = Some(gender);
        self
    }

    fn country(&mut self, country: String) -> &mut Self {
        self.country = Some(country);
        self
    }

    fn age(&mut self, age: u8) -> &mut Self {
        self.age = Some(age);
        self
    }

    fn build(&mut self) -> Customer {
        Customer {
            name: self.name.clone(),
            username: self.username.clone().unwrap_or_default(),
            membership: self.membership.clone().unwrap_or_default(),
            gender: self.gender.unwrap_or_default(),
            country: self.country.clone().unwrap_or_default(),
            age: self.age.unwrap_or_default(),
        }
    }
}

fn main() {
    // we can see from this example, if we want to implement constructors with multiple parameters
    // the params list are getting longer and longer

    // to solve this problem, we can use builder pattern instead
    let user1 = Customer::new_1("aaa".to_owned());
    let user2 = Customer::new_2("aaa".to_owned(), "bbb".to_owned());
    let user3 = Customer::new_3(
        "name".to_string(),
        "username".to_string(),
        Membershiptype::loyal,
    );

    let user = Customer::new("MaeMae".to_owned()).build();
    println!("user content {:?}", user);
    let user_with_login = Customer::new("Josephy".to_owned())
        .username("username_field".to_string())
        .build();

    println!("user content 2 : {:?}", user_with_login);
}
