// IntoIterator
/*
 // definition of IntoIteartor
 trait IntoIterator {
     type Item;
     type Iterator: Iterator;
     fn into_inter(self) -> Self::Iterator;
 }
*/

#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    genre: String,
}

struct BookIterator {
    properties: Vec<String>,
}

impl Iterator for BookIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.properties.is_empty() {
            Some(self.properties.remove(0))
        } else {
            None
        }
    }
}

impl IntoIterator for Book {
    type Item = String;
    type IntoIter = BookIterator;

    fn into_iter(self) -> Self::IntoIter {
        BookIterator {
            properties: vec![self.title, self.author, self.genre],
        }
    }
}

fn main() {
    let b1 = Book {
        title: "title".to_string(),
        author: "author".to_string(),
        genre: "science book".to_string(),
    };

    let mut book_iter = b1.into_iter();
    for book_info in book_iter {
        println!("value {:?}", book_info);
    }
}
