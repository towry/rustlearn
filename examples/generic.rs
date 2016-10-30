///
/// cargo run --example generic
///

use std::fmt;

struct Circle<T> {
    // x: T,
    // y: T,
    radius: T,
}

trait HasArea {
    fn area(&self) -> i64;
}

impl<T> HasArea for Circle<T> {
    fn area(&self) -> i64 {
        print!("ok\n");
        32
    }
}

impl<T> Circle<T> where
    T: fmt::Display {
    fn hello(&self) {
        println!("hello {}", self.radius);
    }
}

fn main() {
    let circle = Circle {
        // x: 30, y: 33,
        radius: 44
    };
    circle.hello();
}
