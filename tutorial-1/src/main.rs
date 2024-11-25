// - Simplifying Structs

struct A {
    f1: u32,
    f2: u32,
    f3: u32,
}

fn fn1(a: &mut A) -> &u32 {
    &a.f2
}

fn fn2(a: &mut A) -> u32 {
    a.f1 + a.f3
}

fn fn3(a: &mut A) {
    let x = fn1(a);
    let y = fn2(a);
}

fn main() {}
