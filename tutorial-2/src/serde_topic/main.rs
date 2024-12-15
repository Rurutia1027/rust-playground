use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Deserialize)]
struct Foo {
    a: u64,
    #[serde(skip)]
    b: String,
}

// here we remove the original Serialize from #[derive(...)]
// and try to implement Serialize for Foo below
// impl Serialize for Foo {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut s = serializer.serialize_struct("Foo", 2)?;
//         s.serialize_field("a", &self.a)?;
//         s.serialize_field("b", &self.b)?;
//         s.end();
//     }
// }

fn main() {
    println!("Hello World");
}
