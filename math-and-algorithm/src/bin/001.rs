use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mikan: u64 = input.trim().parse().unwrap();

    let apple: u64 = 5;
    let ans = apple + mikan;
    println!("{}", ans)
}
