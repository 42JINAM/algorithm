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
            .parse::<i64>()
            .unwrap();
        println!("{}", largest_prime_factor(n as u64));
    }
}

fn largest_prime_factor(mut limit: u64) -> u64 {
    let mut largest = 0;
    let mut n = 2;

    while (n * n <= limit) {
        while limit % n == 0 {
            largest = n;
            limit /= n;
        }
        n += 1;
    }
    if limit > 1 {
        largest = limit;
    }
    largest
}
