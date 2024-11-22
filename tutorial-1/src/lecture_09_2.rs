// Traits & Generic

use std::fmt::Debug;

#[derive(Debug, Clone)]
struct drawing_info {
    line_width: u8,
    color: String,
}

#[derive(Debug, Clone)]
struct Square {
    side: f32,
    info: drawing_info,
}

#[derive(Debug, Clone)]
struct Rectangle {
    length: f32,
    width: f32,
    info: drawing_info,
}

// impl Square {
//     fn calculate_area(&self) {
//         println!("area is {}", self.side * self.side);
//     }
// }

// impl Rectangle {
//     fn area(&self)  -> f32 {
//         self.length * self.width
//     }
// }

fn shape_properties<T: Shape>(object: &T) {
    object.area();
    object.common_func();
}

fn shape_trait_generic_1<T: Shape>(obj: &T) {
    // here we use the obj to invoke the functions defined in only Shape trait
    obj.area();
    obj.common_func();
}

fn shape_trait_generic_2(obj: &impl Shape) {
    // this is another expression of the trait and generic
    // which restrict that the obj is the implementator of the trait Shape
    // then it is allowed to invoke the functions that defined only in Shape
    obj.area();
    obj.common_func();
}

fn shape_trait_generic_3<T>(obj: &T)
where
    T: Shape + Clone + Debug,
{
    // this is a high frequent use of generic + trait
    // expecially for the situations that: if a instance has bind multiple traits

    // this {:?} only when the obj implement/derive the trait of Debug
    println!("obj info : {:?}", obj);
    // here we inovke the fucntions that defined in the trait Shape
    obj.area();
    obj.common_func();
}

// generic + trait as return type
fn build_shape<T>() -> T
where
    T: From<Rectangle> + Shape,
{
    let ret = Rectangle {
        width: 2.0,
        length: 2.0,
        info: drawing_info {
            line_width: 3,
            color: "Red".to_string(),
        },
    };
    T::from(ret)
}

trait Shape {
    fn area(&self) -> f32;
    fn common_func(&self) -> f32 {
        println!("common function can be invoked by both square and rectangle");

        0.0
    }
}
// trait name always come to first
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let ret = self.length * self.width;
        println!("area of rectanble is {}", ret);
        ret
    }

    // this implementation will overwrite the original one
    fn common_func(&self) -> f32 {
        println!("[Shape] = Rectangle invoked common_func");
        self.length * self.width
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        let ret = self.side * self.side;
        println!("area of square is {}", ret);
        ret
    }

    // this implementation will overwrite the original one
    fn common_func(&self) -> f32 {
        println!("[Shape] = Sqaure invoked common_func ");
        self.side * self.side
    }
}

fn main() {
    let r = Rectangle {
        width: 5.0,
        length: 4.0,
        info: drawing_info {
            line_width: 3,
            color: "Red".to_string(),
        },
    };

    let s = Square {
        side: 3.0,
        info: drawing_info {
            line_width: 4,
            color: "White".to_string(),
        },
    };

    r.area();
    s.area();

    r.common_func();
    s.common_func();

    // here we test generic & trait binding functions
    shape_trait_generic_1(&r);
    shape_trait_generic_2(&r);
    shape_trait_generic_3(&r);

    shape_trait_generic_1(&s);
    shape_trait_generic_2(&s);
    shape_trait_generic_3(&s);

    let ret1: Rectangle = build_shape();

    // here this will cause compiler go wrong,
    // because we already declare the type of this generic is Rectanble
    // let ret2: Shape = build_shape();
    println!("ret : {:?}", ret1);

    // println!("ret : {:?}", ret2);
}
