//! implement Display trait

use std::fmt;
use std::fmt::Display;

struct Point {
    x: i32,
    y: i32
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn exclamation<T: Display>(s: T) -> String {
    format!("{}!!!!!!", s)
}

fn main() {
    let p = Point { x: 12, y: 13 };
    println!("{}", exclamation(p));

    println!("{}", exclamation(42));
}
