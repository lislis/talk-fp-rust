//! custom type without display
// NOTE:  This will throw an error

struct Point {
    x: i32,
    y: i32
}

fn main() {
    let p = Point { x: 12, y: 12};
    println!("{}", p);
}
