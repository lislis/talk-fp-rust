//! Collatz sequences

// We take a natural number. If that number is even, we divide it by two.
// If it's odd, we multiply it by 3 and then add 1 to that.
// We take the resulting number and apply the same thing to it,
// which produces a new number and so on.
// In essence, we get a chain of numbers.
// It is thought that for all starting numbers, the chains finish at the number 1.

fn odd(n: u32) -> bool {
    n % 2 == 1
}

fn even(n: u32) -> bool {
    !odd(n)
}

fn collatz_recursion(num: u32) -> Vec<u32> {
    match num {
        1 => { vec![1] },
        n if (even(n)) => {
            let v = vec![n];
            [&v[..], &collatz_recursion(n/2)[..]].concat()
        },
        n => {
            let v = vec![n];
            [&v[..], &collatz_recursion((n*3) + 1)[..]].concat()
        }
    }
}

fn collatz_loop(num: u32) -> Vec<u32> {
    let mut acc: Vec<u32> = vec![num];
    let mut n: u32 = num;

    while n != 1 {
        if even(n) {
            n = n / 2;
            acc.push(n);
        } else {
            n = (n * 3) +1;
            acc.push(n)
        }
    }
    acc
}

fn main() {
    println!("{:?}", collatz_loop(30));
    println!("{:?}", collatz_recursion(30));

    // Problem:
    // For all starting numbers between 1 and 100, how many chains have a length greater than 15?

    // NOTE: stack overflow alert!
    // While this looks cool, it crashed my computer

    //let chain_gt_15: usize =
    //    (0..10).map(|n| collatz_recursion(n))
    //           .filter(|chain| chain.into_iter().count() > 15)
    //           .count();
    //println!("{}", chain_gt_15);
}
