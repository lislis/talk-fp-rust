//! Vector of a generic type

fn head<T>(v: &Vec<T>) -> &T {
    v.first().unwrap()
}
fn main() {
    let numbers = vec![43, 567, 2, 34];
    let strings = vec!["hello", "foo", "world"];
    println!("{}, {}", head(&numbers), head(&strings));

    // This would cause a panic!
    //let empty: Vec<u32> = vec![];
    //println!("{}", head(&empty));
}
