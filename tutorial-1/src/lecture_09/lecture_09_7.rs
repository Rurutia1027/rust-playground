// Choosing Associated Type vs Generic Types

// This lecture is very interesting, and it introduce the basic rules
// when we write codes associated with trait's inner associated types definitons and the generic types
// which is very useful when we trying to declare or design some more higher level logic or functions in Rust

use std::fmt::Debug;

trait Addition {
    type Rhs;
    type Output;
    fn add(self, rhs: Self::Rhs) -> Self::Output;
}

trait Addition_v2<Rhs>
where
    Rhs: Debug,
{
    type Output;
    fn add_v2(&self, rhs: &Rhs) -> Self::Output;
}

// here we continue with the defintion of the trait Addition_v3
// in this trait, we set both the input parameter and output type
// as the generic type , in this way, we no longer need to
// re-declare the type of the reutrn type in previous trait defintions

// here we also add some extra restrictions for the generic type
// that is the input parameter type should implement the trait of Debug at the same time

trait Addition_v3<Rhs, Output>
where
    Rhs: Debug,
    Output: Debug,
{
    fn add_v3(&self, rhs: &Rhs) -> Output;
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Addition for Point {
    type Rhs = Point;
    type Output = Point;

    fn add(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Suppose we want to add another implementation of the `Addition` trait for `Point`
// with different return type parameters.
// However, the compiler will throw an error due to conflicting duplicated implementations.
// To resolve this, we can introduce a generic type parameter for the `Addition` trait.
// This approach avoids the conflict and allows multiple implementaiton of the trait
// with different type declarations.

// impl Addition for Point {
//     type Rhs = i32;
//     type Output = Point;

//     fn add(self, rhs: Self::Rhs) -> Self::Output {
//         Point {
//             x: self.x + rhs,
//             y: self.y + rhs,
//         }
//     }
// }

// let Point implement the trait Addition_v2 which inner generic type defined
// and this implements Point + Point
impl Addition_v2<Point> for Point {
    type Output = Point;

    // here we retrieve the value of inner Point scope by 'self'
    // this will cause the ownership transfer to the function of `add_v2`
    // once ownership transferred , we want to continue manipuate the item by
    // using the for example let x = Point {...}; x.other_func();
    // will caused error.
    // to avoid this, we need to passing a reference of the &self instead of the self directly.
    fn add_v2(&self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// and this implement Point + i32
impl Addition_v2<i32> for Point {
    type Output = Point;
    fn add_v2(&self, rhs: &i32) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Addition_v3<i32, Point> for Point {
    fn add_v3(&self, rhs: &i32) -> Point {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Addition_v3<Point, Line> for Point {
    fn add_v3(&self, rhs: &Point) -> Line {
        Line {
            start: Point {
                x: self.x,
                y: self.y,
            },
            end: Point { x: rhs.x, y: rhs.y },
        }
    }
}
fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    // let p3 = p1.add(p2);
    // assert_eq!(p3.x, 4);
    // assert_eq!(p3.y, 6);
    println!("----- version 2 -----");
    let p3 = p1.add_v2(&p2);
    println!("p3: {:?}", p3);

    let p4 = p1.add_v2(&4);
    println!("p4: {:?}", p4);
    println!("----- version 3 -----");
    let p5 = p1.add_v3(&p1);
    println!("p5: {:?}", p5);

    let p6 = p1.add_v3(&10);
    println!("p6: {:?}", p6);
}
