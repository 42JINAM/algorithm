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
        println!("{}", get_sum(n as u64));
    }
}

fn get_sum(n: u64) -> u64 {
    fn arithmetic_sum(n: u64, d: u64) -> u64 {
        let count = (n - 1) / d;
        d * count * (count + 1) / 2
    }

    arithmetic_sum(n, 3) + arithmetic_sum(n, 5) - arithmetic_sum(n, 15)
}
