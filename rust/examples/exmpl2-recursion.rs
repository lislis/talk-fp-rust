//! Get nth element of fibonacci seq

fn fibonacci(nth: i32) -> i32  {
    match nth {
        0 =>  { 0 },
        1 => { 1 },
        n => {
            fibonacci( n - 1 ) + fibonacci( n - 2)
        }
    }
}

fn main() {
    println!("{}", fibonacci(6));
}
