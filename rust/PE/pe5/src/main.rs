use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..t {
        let n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        println!("{}", smallest_evenly_divisible(n as u64));
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    let mut t: u64;

    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn lcm(a: u64, b: u64) -> u64 {
    return (a * b) / gcd(a, b);
}

fn smallest_evenly_divisible(n: u64) -> u64 {
    let mut smallest: u64 = 1;

    for i in 2..=n {
        smallest = lcm(smallest, i);
    }
    return smallest;
}
