//! Vector of a generic type, with Option<T>

fn head<T>(v: &Vec<T>) -> Option<&T> {
    match v.first() {
        Some(inner) => { Some( inner ) },
        None => { None }
    }
}
fn main() {
    let numbers = vec![43, 567, 2, 34];
    let strings = vec!["hello", "foo", "world"];
    let empty: Vec<u32> = vec![];
    println!("{:?}, {:?}, {:?}",
             head(&numbers), head(&strings), head(&empty));
}
