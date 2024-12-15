# Generics

## Basic Generics

**Generic Struct**
Generics allow us to write flexible and reusable code by defining types or functions that operate on multiple types.

- Generic Struct

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let int_point = Point{x: 5, y:10};
    let float_point = Point {x: 1.2, y: 3.4};
}

```

## Generic Functions

Generic Functions can be written to accept parameters of any type.

```rust
fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

fn main() {
    let (x, y) = swap(5, 10);
    let (x, y) = swap("AA", "BB");
}
```

## Generic Enums

Enums can also use generics

```rust
enum MyOption<T> {
    Some(T),
    None
}

fn main() {
    let num_opt = MyOption::Some(42);
    let non_opt:MyOption<i32> = MyOption::None;

    match num_opt {
        MyOption::Some(val) => println!("Value: {}", val),
        MyOption::None => println!("None Value");
    }
}
```

## Generics with Traits

```rust
fn print_display<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

fn main() {
    print_display(123);
    print_display("Hello Rust");
}
```

## Generic Struct with Trait BOunds

- Generic Struct with Trait Bounds

`Add<Output = T>` This means T must implement the `std::ops::Add` this trait, which defines the '+' operator and the result of T + T must also be of type T.

`Copy`: This means T must implement the Copy Trait, which allows values of T to be duplicated without moving or cloning.

```rust
use std::ops::Add;

struct Rectangle<T: Add<Output = T> + Copy> {
    width: T,
    height: T,
}

impl <T: Add<Output = T> + Copy> Rectangle<T> {
    fn area(&self) -> T {
        self.width + self.height
    }
}
```

- Multiple Trait Bounds

```rust
use std::fmt::Display;

fn compare_and_display<T: PartialOrd + Display>(a: T, b:T) {
    if a > b {
        println!("{} is greater than {}", a, b)
    } else {
        println!("{} is less than or equal to {}", a, b)
    }
}
```

- Lifetimes with Generics

```rust
fn longest<'a, T>(x: &'a T, y: &'a T) -> &'a T
where
T: PartialOrd,
{
    if x > y {
        x
    } else {
        y
    }
}


fn main() {
    let num1 = 42;
    let num2 = 58;
    println!("Longest {}", longest(&num1, &num2));

    let str1 = "hello";
    let str2 = "world";
    println!("Longest: {}", longest(&str1, &str2));
}

```
