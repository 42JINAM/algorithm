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
        let first_multiple_input: Vec<String> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let num = stdin_iterator.next().unwrap().unwrap();
        println!("{}", largest_product_in_series(n, k, num));
    }
}
fn largest_product_in_series(n: i32, k: i32, num: String) -> u64 {
    num.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as u64)
        .collect::<Vec<u64>>()
        .windows(k as usize)
        .map(|win| win.iter().product())
        .max()
        .unwrap()
}
