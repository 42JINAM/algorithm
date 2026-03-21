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
        println!("{}", get_sum(n as u64));
    }
}

fn get_sum(limit: u64) -> u64 {
    let (mut a, mut b) = (1, 1);
    let mut result = 0;

    while b < limit {
        if b % 2 == 0 {
            result += b;
        }
        let next = a + b;
        a = b;
        b = next;
    }
    result
}
