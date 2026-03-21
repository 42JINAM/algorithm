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
        println!("{}", nth_prime(n as usize));
    }
}

fn nth_prime(n: usize) -> i64 {
    let limit = if n < 6 {
        15
    } else {
        (n as f64 * ((n as f64).ln() + (n as f64).ln().ln())) as usize + 100
    };
    sieve_of_eratosthenes(limit)[n - 1] as i64
}

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];

    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    let mut primes = Vec::new();
    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
        }
    }
    primes
}
