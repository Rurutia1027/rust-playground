// Super Traits

use std::{fmt::Debug, task::ready};

struct Square {
    side: f32,
    line_width: u8,
    color: String,
}

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}

trait Shape: Draw + Debug + Clone + OtherTrait + SomeOtherTrait {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value");
        4.0
    }
}

// the Draw trait is the super trait which
// inherit by the trait of Shape
trait Draw {
    fn draw_object(&self);
}

// -- impls for traits --

// if a struct implements a trait, it also need to provide
// the implementaitons of the both the trait and super trait at the same time
// it just like the abstract class usage in java:
// you let a class inherit an abstract class without providing it's(the abstract class)
// inner abstracted functions' implemathion, then the compiler will not happy(go wrong)
// --
// and here another point is, if we want the trait that has multiple super traits
// just use the {trait-1} '+' {trait-2} and so on, but provie the implemations of the struct fot he super trait
// correspoindingly if there are no implemations of the fucntions in the scope of the super traits
impl Shape for Square {
    fn area(&self) -> f32 {
        unimplemented!()
    }

    fn perimeter(&self) -> f32 {
        unimplemented!()
    }

    // // but we cannot impl those in the scope of Shape implemenation
    // // provide the impl for the super trait outside
    // fn draw_object(&self) {
    //     unimplemented!()
    // }
}

impl Draw for Square {
    fn draw_object(&self) {
        unimplemented!()
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        unimplemented!()
    }

    fn perimeter(&self) -> f32 {
        unimplemented!()
    }
}

impl Clone for Rectangle {
    fn clone(&self) -> Self {
        unimplemented!()
    }

    fn clone_from(&mut self, source: &Self) {
        unimplemented!()
    }
}

impl Clone for Square {
    fn clone(&self) -> Self {
        unimplemented!()
    }

    fn clone_from(&mut self, source: &Self) {
        unimplemented!()
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Draw for Rectangle {
    fn draw_object(&self) {
        unimplemented!()
    }
}

impl Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

// here we define other traits for testing the generic & traits test
trait OtherTrait {}

impl OtherTrait for Rectangle {}

impl OtherTrait for Square {}

trait SomeOtherTrait {}

impl SomeOtherTrait for Rectangle {}

impl SomeOtherTrait for Square {}

// suppose we need to implement the generic + (multiple) trait(s)
// and let the generic type support all the implementation of trait Shape

// and we also can treat the T: Shape this expression
//as the T this generic type's restriction definitions

// and the restrictions by different super traits or traits can also
// provide a filter options to filter the correct or satisfied objects to be invoked
// in the scope of the generic functions, and guarantees the

fn generic_trait_func<T>(obj: &T)
where
    T: Shape + OtherTrait + SomeOtherTrait,
{
    unimplemented!()
}

// here is another expression of the generic + trait
// 1. let Shape this base trait extend multiple super traits
// 2. then append where expression, we only need to add the Shape as the Genertic Type's restricted condition

// but, this is useful and convinent when we want the T has multiple restrictions
// and do not want to write multiple restricted conditions in the generic fucntion where's expression
fn generic_traits_func_2<T>(obj: &T)
where
    T: Shape,
{
    unimplemented!()
}

fn main() {}
