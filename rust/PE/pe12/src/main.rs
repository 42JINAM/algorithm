use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdin_iter = stdin.lock().lines();
    let t = stdin_iter
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();
    for _ in 0..t {
        let n = stdin_iter
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap();
        println!("{}", triangular_number(n as u32));
    }
}

fn count_divisor(mut n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    if n == 2 || n == 3 {
        return 2;
    }

    let sqrt_n: u32 = (n as f64).sqrt() as u32;
    let mut cnt: u32 = 1;
    for i in 2..=sqrt_n {
        let mut exp = 1;
        while n % i == 0 {
            exp += 1;
            n /= i;
        }
        if exp > 1 {
            cnt *= exp;
        }
        if n == 1 {
            break;
        }
    }
    if n != 1 {
        cnt *= 2;
    }
    cnt
}

fn triangular_number(limit: u32) -> u32 {
    let mut i: u32 = 1;
    let mut cnt: u32 = 0;
    let mut sum: u32 = 0;

    while cnt <= limit {
        sum += i;
        cnt = count_divisor(sum);
        i += 1;
    }
    sum
}
