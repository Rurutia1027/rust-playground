// Generics

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

impl Point<i32, i32> {
    fn printing(&self) {
        println!("the value of the coordinates are {}, {}", self.x, self.y);
    }

    // if we declare a new function , the compiler will also go wrong
    // because there is already a generic new function in use and they are conflict with each other
    // compiler -> go error, duplicated declaration
    // fn new(x: i32, y: i32) -> Point<i32, i32> {
    //     Point { x, y }
    // }
}

impl Point<f64, f64> {
    fn printing(&self) {
        println!("print values {}, {}", self.x, self.y);
    }
}

fn add_points<T, U>(p1: &Point<T, U>, p2: &Point<T, U>) {
    unimplemented!()
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    let p1 = Point { x: 1.0, y: 4.0 };

    let p2 = Point {
        x: 5,
        y: "5.0".to_string(),
    };

    let origin_new = Point::new(0, 0);
    let p1_new = Point::new(1.0, 4.0);
    let p2_new = Point::new(1.0, "5.0".to_string());

    // origin with generic type declared as i32, i32 match the printlning function type
    // but, other generic types are not satisfy the types,like p1, p1_new, p2, p2_new
    // they cannot invoke printing function becuase type mismatch
    origin.printing();

    // compiler will warning: becuase generaic type not match
    // p1.println();
    // -- > this cannot work, because , in this case
    // the T, and U's corresonding type are not the same in p1, and origin
    // so compiler will go wrong add_points(&p1, &origin);

    // this will also go wrong, with the same type not match reason
    // add_points(&p1, &p2);
}
