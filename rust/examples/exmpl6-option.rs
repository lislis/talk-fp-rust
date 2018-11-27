//! Vector of a generic type, with Option<T>

fn head<T>(v: &Vec<T>) -> Option<&T> {
    v.first()
}

fn main() {
    let empty: Vec<u32> = vec![];
    match head(&empty) {
        Some(val) => { println!("Head is {:?}", val); },
        None => { println!("No head here!"); }
    }

    let numbers = vec![43, 567, 2, 34];
    let _num_head = head(&numbers).expect("No head!");
}
