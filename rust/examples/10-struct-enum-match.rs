use std::f32::consts::PI;
use std::fmt;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone)]
enum Shape {
    Circle(Point, f32),
    Rectangle(Point, Point)
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Shape::Circle(p, r) => {
                write!(f, "A circle with center {} and radius {}", p, r)
            },
            Shape::Rectangle(p1, p2) => {
                write!(f, "A rectangle with points {} and {}", p1, p2)
            }
        }
    }
}

fn surface(s: Shape) -> f32 {
    match s {
        Shape::Circle(_, r) => { PI * r.powf(2.0) },
        Shape::Rectangle(Point { x: x1, y: y1 },
                         Point { x: x2, y: y2 }) => {
            (x2 - x1).abs() * (y2 - y1).abs()
        }
    }
}

fn main() {
    let circle = Shape::Circle(Point { x: 3.0, y: 4.0 }, 10.0);
    println!("{}", circle);
    println!("Has the surface area of {}", surface(circle));

    let rect = Shape::Rectangle(Point { x: 2.0, y: 4.0 },
                                Point { x: 4.0, y: 1.0});
    println!("{}", rect);
    println!("Has the surface area of {}", surface(rect));
}
