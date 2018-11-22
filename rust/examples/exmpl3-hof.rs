//! Higher order functions and closures

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    let upper = 10000;

    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .fold(0, |acc, n_squared| acc + n_squared);

    println!("Result: {}", sum_of_squared_odd_numbers);x
}
