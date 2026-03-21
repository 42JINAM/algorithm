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

    let sieve = get_sieve(1000000 + 1);
    let primes = get_primes(1000000 + 1, &sieve);
    for _ in 0..t {
        let n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        println!("{}", get_sum_of_primes(n as usize, &primes));
    }
}

fn get_sieve(max: usize) -> Vec<bool> {
    let mut sieve: Vec<bool> = vec![true; max as usize + 1];
    sieve[0] = false;
    sieve[1] = false;

    let limit = (max as f64).sqrt() as usize;
    for i in 2..=limit {
        if sieve[i] {
            let mut j = i * i;
            while j <= max {
                sieve[j] = false;
                j += i;
            }
        }
    }
    sieve
}

fn get_primes(max: usize, sieve: &Vec<bool>) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    for i in 1..sieve.len() {
        if sieve[i] && i <= max as usize {
            primes.push(i as u64);
        }
    }

    return primes;
}

fn get_sum_of_primes(max: usize, primes: &Vec<u64>) -> u64 {
    primes.iter().filter(|x| **x <= max as u64).sum()
}
