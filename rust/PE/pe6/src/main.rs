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
        println!("{}", sum_square_difference(n as i64));
    }
}

fn sum_square_difference(n: i64) -> i64 {
    sum(n).pow(2) - square_sum(n)
}

fn sum(n: i64) -> i64 {
    n * (1 + n) / 2
}

fn square_sum(n: i64) -> i64 {
    (1..=n).map(|x| x * x).sum()
}
