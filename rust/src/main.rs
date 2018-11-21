fn head(v:Vec<u32>) -> u32 {
    v[0]
}

fn main() {
    let vector = vec![43, 567, 2, 34];
    println!("{}", head(vector));
}
