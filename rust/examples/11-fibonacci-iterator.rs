//! implement fibonacci data type
// inspired by https://codereview.stackexchange.com/questions/130042/implement-a-generic-fibonacci-sequence-in-rust-without-using-copy-trait/130085

struct Fibonacci {
    current: i32,
    index: i32
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci {
            current: 0,
            index: 1
        }
    }
    pub fn nth(nth: i32) -> i32 {
        match nth {
            0 =>  { 0 },
            1 => { 1 },
            n => {
                Fibonacci::nth( n - 1 ) + Fibonacci::nth( n - 2)
            }
        }
    }
}

impl Iterator for Fibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item>{
        self.index += 1;
        let c = Fibonacci::nth(self.index);
        self.current = c;
        Some(self.current)
    }
}

#[test]
fn fib_test_iter() {
    let values: Vec<i32> = Fibonacci::new().take(5).collect();
    assert_eq!(values, [1, 2, 3, 5, 8]);
}

#[test]
fn fib_test_nth() {
    let seq12: i32 = Fibonacci::nth(12);
    assert_eq!(seq12, 144);
}


// Among the first 11 numbers of the Fibonacci sequence, is there one odd number larger than 100?

fn main() {
    let f: bool = Fibonacci::new()
        .take(10)
        .filter(|n| { n % 2 == 1 })
        .any (|n| { n > 100});
    println!("{}", f);
}
