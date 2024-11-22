// Trait Object && Dynamic Dispatch

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

trait Shape: Draw {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value");
        4.0
    }
}

trait Draw {
    fn draw_object(&self);
}

impl Shape for Square {
    fn area(&self) -> f32 {
        println!("[Shape] = Square area invoked");
        3.4
    }

    fn perimeter(&self) -> f32 {
        println!("[Shape] = Square perimeter invoked");
        5.2
    }
}

impl Draw for Square {
    fn draw_object(&self) {
        println!("draw obj in Square");
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        println!("[Shape] = Rectangle area invoked");
        0.9
    }

    fn perimeter(&self) -> f32 {
        println!("[Shape] = Rectangle perimeter invoked");
        9.3
    }
}

impl Draw for Rectangle {
    fn draw_object(&self) {
        println!("draw object for Rectanble")
    }
}

fn shape_properties_static<T>(obj: T)
where
    T: Shape,
{
    obj.area();
    obj.perimeter();
}

// here we use the Rust's dynamic dispather
// which is more smart than the previous generic & trait function: shape_properties
// it can inference the final type and the type's asscicated functions and restrictions automatically
// so if use the Box<dyn Shape> as the parameter of the fucntion
// we no longer need add where this expression any more

// this is dynamic dispatch
fn shape_properties_dynamic(obj: Box<dyn Shape>) {
    println!("area value {}", obj.area());
    println!("perimeter value {}", obj.perimeter());
    obj.draw_object();
}

// when we want to define a return generic type,
// the dynamic dispatch is always a recommended solution
// and this is a demo that even though Square and Rectanble are two instance of struct that impl the trait of Shape.
// and we declare the Shape as the return type, it also let the compiler not happy
// this is because, at compiler time, the return value's references should be match with the exact implementation of the
// trait of Shape, and we return two possible instances {Square or Rectable} so at compile time the compiler
// does not know which kind of exactly reference it should create, so the compile will go wrong.

// fn returns_shape_static(flag: u32) -> impl Shape {
//     match flag {
//         1 => Square {
//             side: 0.3,
//             line_width: 3,
//             color: "Red".to_string(),
//         },
//         _ => {
//             Rectangle {
//             length: 3.21,
//             width: 23.3,
//             line_width: 3,
//             color: "Black".to_string(),
//         }
//     }
// }
// --- to solve this problem, we let the return type as the Box wrapped type
// which is the dynamic dispatch, this kind of Box decorated reference can bind to any type of the struct insance
// which implements the trait of Shape,

// but we also need to convert return value as Box instance by invoking the function Box::new(instance of the struct)
fn returns_shape_dynamic(flag: u32) -> Box<dyn Shape> {
    match flag {
        1 => {
            let ret = Square {
                side: 0.3,
                line_width: 3,
                color: "Red".to_string(),
            };

            Box::new(ret)
        }
        _ => {
            let ret = Rectangle {
                length: 3.21,
                width: 23.3,
                line_width: 3,
                color: "Black".to_string(),
            };
            Box::new(ret)
        }
    }
}

/// difference between functions of **_static and **_dyanmic:
/// - shape_properties_static: This uses generics with trait bounds.
///   The compiler determines the exact type (`T`) at compile time and generates specific code for each type.
///   This approach is efficient because there's no runtime overhead for determining the exactly type or function calls.
///   However, it results in a larger binary size if used with many different types, as separate code generated for each type.
///
/// - shape_properties_dynamic: This uses a `Box<dyn Shape>`, which is a **trait object**.
///   Here, the exact type is not known at compile time. Instead, the program uses a vtable (a lookup table)
///   at run time to resolve method calls. This allows flexibility but incurs a slight runtime overhead due to
///   indirection and type checking.
///
///   The `Box` ensures that the object implementaiton the `Shape` trait is stored on the heap, and the `dyn Shape`
///   means that we are using a reference to a trait object instead of a concrete type.
/// Summary:
/// - Static Dispatch: The compiler knows the type at compile time. Faster, no runtime overhead, but less flexible.
/// - Dynamic Dispatch: The compiler resolves the type at runtime. More flexible but slightly slower due to runtime indirection.

fn main() {
    let r1 = Rectangle {
        length: 2.0,
        width: 3.0,
        line_width: 3,
        color: "White".to_string(),
    };

    let s1 = Square {
        line_width: 2,
        side: 3.0,
        color: "Red".to_string(),
    };

    shape_properties_dynamic(Box::new(r1));
    shape_properties_dynamic(Box::new(s1));

    let ss = returns_shape_dynamic(1);
    let rr = returns_shape_dynamic(2);

    // --- trait object && dynamic dispatch  ---
    // [Shape] = Square area invoked
    // [Shape] = Square perimeter invoked
    // draw obj in Square
    // [Shape] = Rectangle area invoked
    // [Shape] = Rectangle perimeter invoked
    // draw object for Rectanble
    // --- trait object && dynamic dispatch  ---

    println!("-------------------");
    // let square ss executes a series of actions
    ss.area();
    ss.perimeter();
    ss.draw_object();

    // let rectangle rr executes a series of actions
    rr.area();
    rr.perimeter();
    rr.draw_object();
}
