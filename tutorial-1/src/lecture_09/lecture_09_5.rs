// Derived Traits
// Marker Traits

/// Definition of Marker Trait in Rust
/// A marker trait in Rust is a trait with no methods or associated items.
/// Its purpose is not to define behavior but to indicate or "mark" that a
/// type possesses a certain properties or capability.
///
/// Marker traits are often used to provide compile-time guarantees or enable optimizations by
/// informing the compiler about specific characgteristics of types.
///
/// Some Marker Traits
/// like: `Send`, and `Sync` are most well-known marker traits in Rust.
/// * Send: indicates that the decorated type can safely be transferred between threads.
/// * Sync: indicates that atype can be safely accessed from multiple threads simultaneously.

// here we declare a Marker Traits
trait Properties: PartialEq + Default + Clone {}

#[derive(Debug, PartialEq, Default, Clone)]
struct Student {
    name: String,
    age: u8,
    sex: char,
}

impl Properties for Student {}

fn main() {
    let s1 = Student {
        name: String::from("name1"),
        age: 32 as u8,
        sex: 'M',
    };

    let s2 = Student {
        name: String::from("name2"),
        age: 33,
        sex: 'M',
    };

    println!("student: {:?}", s1);
    println!("student: {:?}", s2);

    println!("equal val: {}", (s1 == s2));
}
